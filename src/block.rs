use crate::pos::BlockPos as Pos;

pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;

/*

let world = FileWorld::from("file://~/worlds/test_world/");
let mut cur = BlockCursor::new(&mut world);


let pos = BlockPos::from_xyz(1, 2, 3);
cur.set_position(pos);
cur.seek_block(/* */)?;
cur.write_block(pos, state);

 */

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct State(u16); // Id, meta, etc.

// `I` implies an underlying block
#[derive(Debug, Clone)]
pub struct Cursor<I> { // block::Cursor
    inner: I,
    pos: Pos,
}

impl<I> Cursor<I> {
    pub fn new(inner: I) -> Cursor<I> {
        Cursor {
            inner,
            pos: Default::default()
        }
    }

    pub fn into_inner(self) -> I {
        self.inner
    }

    pub fn position(&self) -> Pos {
        self.pos
    }
    // does not check if this position is valid
    pub fn set_position(&mut self, pos: Pos) {
        self.pos = pos;
    }

}

// usually for wrappers, such as `Cursor`
pub trait Read { // block::Read
    // read the block from currect position.
    // the current position is NOT advanced.
    // panic if the currect position is not valid
    fn read_block(&self) -> Result<State>;
}

// usually for wrappers, such as `Cursor`
pub trait Write { // block::Write
    // write the block into currect position.
    // the current position is NOT advanced. 
    // the writer may also initalize block buffer for it
    // panic if the currect position is not valid
    fn write_block(&mut self, new_state: State) -> Result<()>;
}

// usually for buffered world itself
pub trait ReadExact {
    // read block exactly at the position `pos`
    fn read_block_exact(&self, pos: Pos) -> Result<State>; 
}

// usually for buffered world itself
pub trait WriteExact {

    fn write_block_exact(&mut self, pos: Pos, state: State) -> Result<()>; 
}

impl<I: ReadExact> Read for Cursor<I> {
    fn read_block(&self) -> Result<State> {
        self.inner.read_block_exact(self.pos)
    } 
} 
 
impl<I: WriteExact> Write for Cursor<I> {
    fn write_block(&mut self, new_state: State) -> Result<()> {
        self.inner.write_block_exact(self.pos, new_state)
    }
}

pub trait Validate {

    fn contains_block(&self, pos: Pos) -> Result<bool>;
}

impl<I: Validate> Cursor<I> {
    pub fn check_current_block(&self) -> Result<bool> {
        self.inner.contains_block(self.pos)
    }
}

impl<I: ReadExact + Validate> Cursor<I> {
    // must ensure that this is a valid position
    pub fn get_block_state(&mut self, pos: impl Into<Pos>) -> Result<State> {
        let pos = pos.into();
        self.set_position(pos);
        self.read_block()
    }
}

impl<I: WriteExact + Validate> Cursor<I> {
    // must ensure that this is a valid position
    pub fn set_block_state(&mut self, pos: impl Into<Pos>, state: State) -> Result<()> {
        let pos = pos.into();
        self.set_position(pos);
        self.write_block(state)
    }
}

pub enum SeekFrom {
    Absolute(Pos),
    Relative(i32, i32, i32),
}

pub trait Seek {
    
    fn seek_block(&mut self, from: SeekFrom) -> Result<()>;
}

impl<I: Validate> Seek for Cursor<I> {

    fn seek_block(&mut self, from: SeekFrom) -> Result<()> {
        use self::SeekFrom::*;
        let pos = match from {
            Absolute(pos) => pos,
            Relative(dx, dy, dz) => {
                let (x, y, z) = self.pos.to_xyz();
                Pos::from_xyz(x + dx, y + dy, z + dz)
            },
        };
        self.set_position(pos);
        Ok(()) 
    }
} 

// // Spawn point and other stuff
// pub trait SeekSpecial {

//     type Special;

//     fn seek_block_special(&mut self, from: Self::Special) -> Result<()>;
// }

// an actual string id for blocks
// available for everything except 'transparent' block
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(String);

impl<I: AsRef<str>> From<I> for Id{
    fn from(src: I) -> Id {
        Id(String::from(src.as_ref()))
    }
}

impl<I: AsRef<str>> PartialEq<I> for Id {
    fn eq(&self, other: &I) -> bool {
        other.as_ref() == self.0
    }
}  

// a block system that maps state with actual string id.
// for example it converts "minecraft:stone" into BlockState(1). 
// the inner number is intended for internal use and may vary between implementations.
// often contained in worlds. one world imply one block system, and may not change in runtime
pub trait System {
    // check if this state is registered
    fn has_block_state(&self, state: State) -> bool;
    // panic if block state not found
    fn block_state_to_id(&self, state: State) -> Id;
    // check if this block id is registered
    fn has_block_id(&self, id: Id) -> bool;
    // panic if block state not found
    fn block_id_to_state(&self, id: Id) -> State;
}

pub trait Operate {

    fn block_system(&self) -> &dyn System;
}

impl<I: Operate + ReadExact + Validate> Cursor<I> {
    // must ensure that this is a valid position
    pub fn get_block_id(&mut self, pos: impl Into<Pos>) -> Result<Id> {
        let state = self.get_block_state(pos)?;
        Ok(self.inner.block_system().block_state_to_id(state))
    }
}

impl<I: Operate + WriteExact + Validate> Cursor<I> {
    // must ensure that this is a valid position
    pub fn set_block_id(&mut self, pos: impl Into<Pos>, id: impl Into<Id>) -> Result<()> {
        let state = self.inner.block_system().block_id_to_state(id.into());
        self.set_block_state(pos, state)
    }
}

// pub trait ChunkRead 

// ChunkWrite

#[cfg(test)]
mod tests {
    use crate::block::*;
    // every block is "minecraft:air" except (0, 60, 0) is "minecraft:sponge"
    struct TestWorld(TestBlockSystem);

    impl TestWorld {
        pub fn new() -> TestWorld {
            TestWorld(TestBlockSystem)
        }
    }

    impl Validate for TestWorld {
        fn contains_block(&self, _pos: Pos) -> Result<bool> {
            Ok(true) // every block is valid
        }
    }

    impl ReadExact for TestWorld {
        fn read_block_exact(&self, pos: Pos) -> Result<State> {
            Ok(match pos {
                p if p != Pos::from_xyz(0, 60, 0) => State(0),
                _ => State(70) //todo state to id
            })
        }    
    }

    impl WriteExact for TestWorld {
        fn write_block_exact(&mut self, _pos: Pos, _state: State) -> Result<()> {
            Err(Error::new(std::io::ErrorKind::PermissionDenied, "Operation not supported"))
        }
    }

    struct TestBlockSystem;

    impl System for TestBlockSystem {

        fn has_block_state(&self, state: State) -> bool {
            state.0 == 0 || state.0 == 70
        }
        
        fn block_state_to_id(&self, state: State) -> Id {
            match state.0 {
                0 => Id::from("minecraft:air"),
                70 => Id::from("minecraft:sponge"),
                _ => panic!("Unsupported block")
            }
        }
        
        fn has_block_id(&self, id: Id) -> bool {
            id.0 == "minecraft:air" || id.0 == "minecraft:sponge"
        }
        
        fn block_id_to_state(&self, id: Id) -> State {
            match id.0 {
                ref id if id == "minecraft:air" => State(0),
                ref id if id == "minecraft:sponge" => State(70),
                _ => panic!("Unsupported block")
            }
        }
    }

    impl Operate for TestWorld {
        fn block_system(&self) -> &dyn System {
            &self.0
        }
    }

    #[test]
    fn read_write_block() -> Result<()> {
        let world = TestWorld::new();
        let mut cur = Cursor::new(world);
        let id = cur.get_block_id((0, 0, 0))?;
        assert_eq!(id, "minecraft:air");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn write_block() {
        let world = TestWorld::new();
        let mut cur = Cursor::new(world);
        cur.set_block_state((1, 1, 1), State(1)).unwrap();
    }
    
}
