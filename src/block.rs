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

pub trait Read { // block::Read
    // read the block from currect position.
    // the current position is NOT advanced.
    fn read_block(&self) -> Result<State>;
}

pub trait Write { // block::Write
    // write the block into currect position.
    // the current position is NOT advanced. 
    // the writer may also initalize block buffer for it
    fn write_block(&mut self, new_state: State) -> Result<()>;
}


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

    // todo: HashMap operations
}


pub trait Validate {

    fn contains_block(&self, pos: Pos) -> Result<bool>;
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

pub trait ReadExact {
    // read block exactly at the position `pos`
    fn read_block_exact(&self, pos: Pos) -> Result<State>; 
}

pub trait WriteExact {

    fn write_block_exact(&mut self, pos: Pos, state: State) -> Result<()>; 
}

// an actual string id for blocks
// available for everything except 'transparent' block
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(String);

use std::borrow::Cow;
impl From<Cow<'a, str>> for Id {
    fn from(src: Cow<'a, str>) -> Id {
        Id(src.into_owned())
    }
}

// a block system that maps state with actual string id.
// for example it converts "minecraft:stone" into BlockState(1). 
// the inner number is intended for internal use and may vary between implementations.
pub trait System {

    
}

// pub trait ChunkRead 
// ChunkWrite
#[cfg(test)]
mod tests {
    use crate::block::*;
    // every block is "minecraft:air" except (0, 60, 0) is "minecraft:sponge"
    struct TestWorld;

    impl Validate for TestWorld {
        fn contains_block(&self, _pos: Pos) -> Result<bool> {
            Ok(true) // every block is valid
        }
    }

    impl ReadExact for TestWorld {
        fn read_block_exact(&self, pos: Pos) -> Result<State> {
            Ok(match pos {
                p if p != Pos::from_xyz(0, 0, 0) => State(0),
                _ => State(70) //todo state to id
            })
        }    
    }

    impl WriteExact for TestWorld {
        fn write_block_exact(&mut self, _pos: Pos, _state: State) -> Result<()> {
            Err(Error::new(std::io::ErrorKind::PermissionDenied, "Operation not supported"))
        }
    }

    #[test]
    fn read_write_block() -> Result<()> {
        let world = TestWorld;
        let mut cur = Cursor::new(world);
        let state = cur.get_block_state((0, 0, 0))?;
        println!("{:?}", state);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn write_block() {
        let world = TestWorld;
        let mut cur = Cursor::new(world);
        cur.set_block_state((1, 1, 1), State(1)).unwrap();
    }
    
}
