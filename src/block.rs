pub use crate::pos::BlockPos as Pos;
pub use crate::id::Id;

use std::io::Result;

/*

let world = FileWorld::from("file://~/worlds/test_world/");
let mut cur = BlockCursor::new(&mut world);

let pos = BlockPos::from_xyz(1, 2, 3);
cur.set_block_id(pos, "minecraft:air");

 */

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Meta {
    inner: u16
} // Id, meta, etc.

impl Meta {
    crate fn new(inner: u16) -> Meta {
        Meta { inner }
    }
}

// pub struct Buf {
//     inner: Vec<u8>
// }

/*

Where is the difference between Read and ReadExact?

Read somehow provides a way to read a block at one position,
which is useful for continuously monitoring of one single block.
For example we can have a SignCursor<T> where T: block::Read
to help read a string value of one single block of sign
whose text might change as we concern.

Never add event system into it!

Developers must note that this is not relevant with event system.
This Read trait should only be used in low-level storage.
Do NOT even try to add a event hook into this trait - it would
be inappropriate with the original intention of this name of Read.
Never modify this trait without careful thinking.

-- Luo Jia

*/

// usually for wrappers, such as `Cursor`, `SignCursor`, etc.
pub trait Read { // block::Read
    // read the block from current position.
    // the current position is NOT advanced.
    // panic if there is no block at current position
    fn read_block(&self) -> Result<Meta>;
    // check if there is a block at current position
    fn check_current_block(&self) -> Result<bool>;
}

// usually for wrappers, such as `Cursor`
pub trait Write { // block::Write
    // write the block into current position.
    // the current position is NOT advanced. 
    // the writer may also initialize block buffer for it
    // panic if the current position is not valid
    fn write_block(&mut self, new_meta: Meta) -> Result<()>;
    // ensure the blocks reach their destination
    // for example, saved to files
    fn flush_block(&mut self) -> Result<()>;
}

// usually for buffered world itself
pub trait ReadExact {
    // read block exactly at the position `pos`
    fn read_block_exact(&self, pos: Pos) -> Result<Meta>;

    fn contain_block_exact(&self, pos: Pos) -> Result<bool>;
}

// usually for buffered world itself
pub trait WriteExact {

    fn write_block_exact(&mut self, pos: Pos, meta: Meta) -> Result<()>;

    fn flush_block(&mut self) -> Result<()>;
}


pub enum SeekFrom {
    Absolute(Pos),
    Relative(i32, i32, i32),
}

pub trait Seek {
    
    fn seek_block(&mut self, from: SeekFrom) -> Result<()>;
}

// // Spawn point and other stuff
// pub trait SeekSpecial {

//     type Special;

//     fn seek_block_special(&mut self, from: Self::Special) -> Result<()>;
// }

// a system maps universal Id into internal Meta
// for example it converts "minecraft:stone" into Meta with `1` as inner.
// the inner number is intended for internal use and may vary between implementations.
// often contained in worlds. one world imply one block system, and may not change in runtime
pub trait IdSystem {
    // check if this meta is registered
    fn has_block_meta(&self, meta: Meta) -> bool;
    // panic if block meta not found
    fn block_meta_to_id(&self, meta: Meta) -> Id;
    // check if this block id is registered
    fn has_block_id(&self, id: Id) -> bool;
    // panic if block meta not found
    fn block_id_to_meta(&self, id: Id) -> Meta;
}

pub trait IdOperate {

    fn block_id_system(&self) -> &dyn IdSystem;
}

// `I` implies an underlying block storage
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

impl<I: ReadExact> Cursor<I> {
    // must check that this is at a valid position first
    // or this method will panic
    pub fn get_block_meta(&mut self, pos: Pos) -> Result<Meta> {
        self.set_position(pos);
        self.read_block()
    }
}

impl<I: ReadExact> Read for Cursor<I> {
    fn read_block(&self) -> Result<Meta> {
        self.inner.read_block_exact(self.pos)
    }

    fn check_current_block(&self) -> Result<bool> {
        self.inner.contain_block_exact(self.pos)
    }
}

impl<I: WriteExact> Cursor<I> {
    // must ensure that this is at a valid position first
    // or this method will panic
    pub fn set_block_meta(&mut self, pos: Pos, meta: Meta) -> Result<()> {
        self.set_position(pos);
        self.write_block(meta)
    }
}

impl<I: WriteExact> Write for Cursor<I> {
    fn write_block(&mut self, new_meta: Meta) -> Result<()> {
        self.inner.write_block_exact(self.pos, new_meta)
    }

    fn flush_block(&mut self) -> Result<()> {
        self.inner.flush_block()
    }
}

impl<I: IdOperate + ReadExact> Cursor<I> {
    // must ensure that this is a valid position
    pub fn get_block_id(&mut self, pos: Pos) -> Result<Id> {
        let meta = self.get_block_meta(pos)?;
        Ok(self.inner.block_id_system().block_meta_to_id(meta))
    }
}

impl<I: IdOperate + WriteExact> Cursor<I> {
    // must ensure that this is a valid position
    pub fn set_block_id(&mut self, pos: Pos, id: Id) -> Result<()> {
        let meta = self.inner.block_id_system().block_id_to_meta(id);
        self.set_block_meta(pos, meta)
    }
}

impl<I> Seek for Cursor<I> {

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

// default system def
use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct HashSystem {
    mti: HashMap<Meta, String>,
    itm: HashMap<String, Meta>,
    next_inner: u16
}

impl HashSystem {
    pub fn new() -> HashSystem {
        Default::default()
    }

    pub fn register_block(&mut self, id: Id) -> Meta {
        let id = id.to_string();
        let meta = Meta::new(self.next_inner);
        self.itm.insert(id.clone(), meta.clone());
        self.mti.insert(meta.clone(), id);
        self.next_inner += 1;
        meta
    }
}

impl IdSystem for HashSystem {
    
    fn has_block_meta(&self, meta: Meta) -> bool {
        self.mti.contains_key(&meta)
    }
    
    fn block_meta_to_id(&self, meta: Meta) -> Id {
        Id::new(self.mti[&meta].clone())
    }   
    
    fn has_block_id(&self, id: Id) -> bool {
        self.itm.contains_key(&id.to_string())
    }
    
    fn block_id_to_meta(&self, id: Id) -> Meta {
        self.itm[&id.to_string()]
    }
}

macro_rules! reg_blocks {
    ($($id_name: ident, $id_string: expr,)+) => {

$(pub static $id_name: &'static str = $id_string;)+

pub fn global_id_system() -> HashSystem {
    let mut ans = HashSystem::new();
    $(ans.register_block(Id::new($id_string));)+
    ans
}
    };
}

reg_blocks!(
    ID_BLOCK_AIR, "minecraft:air",
    ID_BLOCK_STONE, "minecraft:stone",
    ID_BLOCK_SPONGE, "minecraft:sponge",
);

// define our own cursors
// match with System, following PC rules

/*
Anvil: facing
Banner(standing): rotation, waterlogged
Banner(wall): facing, waterlogged
Wall: facing, waterlogged
Bed: facing, occupied, part
Beetroot: age 0~3
Bone block: axis
Brewing stand: has_bottle_{0~2}
Button: face, facing, powered
Cactus: age 0~15
Cake: bites 0~6
Carrot: age 0~7
Carved pumpkin: facing
Cauldron: level 0~3
Chest(Trapped ~): facing, type, waterlogged
Chorus flower: age 0~5
Chorus plant: north, south, east, west, up, down
Cobblestone wall(Mossy ~): north, south, east, west, up, waterlogged
Cocoa: age, facing
Command block: conditional, facing
Daylight detector: inverted, power
Dispenser: facing, triggered
Door: facing, half, hinge, open, powered
Dropper: facing, triggered
Ender chest: facing, waterlogged // not able to connect chests
End portal frame: eye, facing
End rod: facing
Farmland: moisture 0~7
Fence: north, south, east, west, waterlogged
Fence gate: facing, in_wall, open, powered
Fire: age, up, north, south, east, west
Frosted Ice: age 0~3
Furnace: facing, lit
Glass pane(Stained ~): north, south, east, west, waterlogged
Glazed Terracotta: facing
Grass Block, Mycelium, Podzol: snowy
Hay bale: axis
Hopper: enable, facing
Iron bars: north, south, east, west
Jack o'lantern: facing
Jukebox: has_record
Kelp: age 0~25
Ladder: facing, waterlogged
Large Flowers: half
Lava: level
Leaves: persistent, distance
Lever: face, facing, powered
Logs: axis
Melon stem/Pumpkin stem: age 0~7
Melon stem/Pumpkin stem(attached): facing
Mob head(on floor): rotation
Mob head(on wall): facing
Mushroom block: down, east, north, south, up, west
Nether wart: age 0~3
Nether portal: axis
Observer: facing, powered
Pistons(Static, Sticky ~): extended, facing
Pistons(Moving): facing, type
Piston Head: facing, short, type
Potato: age 0~7
Pressure plate: powered
Pressure plate(weighted): power 0~15
Purpur pillar: axis
Quartz pillar: axis
Rails: shape
Rail(Activator ~, Detector ~, Powered ~): powered, shape
Redstone comparator: facing, mode, powered
Redstone dust: ??
Redstone ore: lit
Redstone repeater: delay. facing, locked, powered
Redstone torch: lit
Redstone torch(wall): lit, facing
Sapling: stage
Seagrass: half // unable to be placed out of water
Sea pickle: pickles, waterlogged
Shulker box: facing
Sign: rotation, waterlogged
Wall: facing, waterlogged
Slabs: type, waterlogged
Snow: layers 1~8
Stairs: facing, half,shape, waterlogged
Structure block: mode
Sugar canes: age 0~15
Tall grass, Large fern: half
TNT: unstable //needs test
Trapdoor: facing, half, open, powered, waterlogged 
Tripwire: attached, disarmed, north, south. east, west, powered
Tripwire hook: attached, facing, powered
Turtle egg: eggs 1~4, hatch 0~2
Vines: north, south, east, west, up
Wall torch: facing
Water: level
Wheat: age 0~7
Wood: axis
//finish here

 */
/*
pub trait SignRead {...}
pub trait SignWrite {...}
impl<I> SignRead for Cursor<I> {...}
...


//block facing


//procceed sth about liquid & waterlogged

//流体包括空气和液体
pub enum FluidType {} 

pub trait FluidRead {...}


// grass & flower
//structure block
 */

// PistonPolicy (varies between editions)
// RedTransmitPolicy
// DropPolicy

#[cfg(test)]
mod tests {
    use crate::block::*;
    use std::io::{Error, Result};
    // every block is "minecraft:air" except (0, 60, 0) is "minecraft:sponge"
    struct TestWorld(Box<IdSystem>);

    impl TestWorld {
        pub fn new() -> TestWorld {
            TestWorld(Box::new(global_id_system()))
        }
    }

    impl ReadExact for TestWorld {
        fn read_block_exact(&self, pos: Pos) -> Result<Meta> {
            Ok(match pos {
                p if p != Pos::from_xyz(0, 60, 0) => self.0.block_id_to_meta(Id::new("minecraft:air")),
                _ => self.0.block_id_to_meta(Id::new("minecraft:sponge"))
            })
        }

        fn contain_block_exact(&self, _pos: Pos) -> Result<bool> {
            Ok(true) // every block is valid
        }
    }

    impl WriteExact for TestWorld {
        fn write_block_exact(&mut self, _pos: Pos, _meta: Meta) -> Result<()> {
            Err(Error::new(std::io::ErrorKind::PermissionDenied, "Operation not supported"))
        }

        fn flush_block(&mut self) -> Result<()> {
            Ok(()) // no-op
        }
    }

    impl IdOperate for TestWorld {
        fn block_id_system(&self) -> &dyn IdSystem {
            self.0.as_ref()
        }
    }

    #[test]
    fn read_write_block() -> Result<()> {
        let world = TestWorld::new();
        let mut cur = Cursor::new(world);
        assert_eq!(cur.get_block_id(Pos::from_xyz(0, 0, 0))?, "minecraft:air");
        assert_eq!(cur.get_block_id(Pos::from_xyz(0, 60, 0))?, "minecraft:sponge");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn write_block() {
        let world = TestWorld::new();
        let mut cur = Cursor::new(world);
        cur.set_block_id(Pos::from_xyz(1, 1, 1), Id::new("minecraft:stone")).unwrap();
    }
    
    #[test]
    fn seek() -> Result<()> {
        let mut cur = Cursor::new(TestWorld::new());
        cur.set_position(Pos::from_xyz(10, 10, 10));
        cur.seek_block(SeekFrom::Relative(2, -1, 1))?;
        assert_eq!(cur.position(), Pos::from_xyz(12, 9, 11));
        Ok(())
    }
}
