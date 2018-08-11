use crate::pos::BlockPos;
use crate::id::BlockId;

// id and meta, including facing
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BlockState {
    runtime_internal: u16
}

use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    BlockNotFound,
    IoError(IoError)
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Error>;

pub type BlockBuf = Vec<u8>;

// runtime system, may NOT vary between client editions & versions
pub trait BlockSystem {
    // None: no such block. Some(id): block found.
    fn block_state_to_id(&self, state: BlockState) -> Option<BlockId>;
    // None: no such state. Some(state): found state.
    fn block_id_to_state(&self, id: BlockId) -> Option<BlockState>;

    fn buf_for_block_state(&self, state: BlockState) -> Option<BlockBuf>;
}

pub mod hash_block_system {
    use std::collections::{HashMap, HashSet};
    use super::{BlockId, BlockState, BlockBuf, BlockSystem};

    #[derive(Default)]
    pub struct HashBlockSystem {
        state_to_id: HashMap<u16, BlockId>,
        id_to_state: HashMap<BlockId, u16>,
        state_should_create: HashSet<u16>,
        next_state: u16
    }

    impl HashBlockSystem {
        pub fn new() -> HashBlockSystem {
            Default::default()
        }

        pub fn register_new_block_unbuffered(&mut self, id: impl Into<BlockId>) {
            let id = id.into();
            self.reg_block(id.clone());
            self.reg_buf(id, false);
            self.next_state += 1;
        }

        pub fn register_new_block_buffered(&mut self, id: impl Into<BlockId>) {
            let id = id.into();
            self.reg_block(id.clone());
            self.reg_buf(id, true);
            self.next_state += 1;
        }

        #[inline]
        fn reg_block(&mut self, id: BlockId) {
            if !self.id_to_state.contains_key(&id) {
                self.id_to_state.insert(id.clone(), self.next_state);
                self.state_to_id.insert(self.next_state, id);
            }
        } 

        #[inline]
        fn reg_buf(&mut self, id: BlockId, buffered: bool) {
            let state = self.id_to_state.get(&id).unwrap();
            match buffered {
                true => self.state_should_create.insert(*state),
                false => self.state_should_create.remove(state),
            };
        } 
    }

    impl BlockSystem for HashBlockSystem {

        fn block_state_to_id(&self, state: BlockState) -> Option<BlockId> {
            self.state_to_id.get(&state.runtime_internal).map(|s| s.to_owned())
        }

        fn block_id_to_state(&self, id: BlockId) -> Option<BlockState> {
            self.id_to_state.get(&id).map(|&inner| BlockState { runtime_internal: inner })
        }

        fn buf_for_block_state(&self, state: BlockState) -> Option<BlockBuf> {
            match self.state_should_create.contains(&state.runtime_internal) {
                true => Some(vec![0; 0]),
                false => None
            }
        }
    }
}

pub trait BlockRead {

    fn read_block_state(&self, pos: BlockPos) -> Result<BlockState>;

    fn block_buf(&self, pos: BlockPos) -> &BlockBuf;

    fn contains_block(&self, pos: BlockPos) -> bool;
}

pub trait BlockWrite {

    fn write_block_state(&mut self, pos: BlockPos, new_state: BlockState) -> Result<()>;

    fn flush_blocks(&mut self) -> Result<()>;
    
    fn block_buf_mut(&mut self, pos: BlockPos) -> &mut BlockBuf;
}

pub trait BlockOperate {

    fn get_block_system(&self) -> &dyn BlockSystem;
}

pub struct BlockCursor<T>(T);

impl<T> BlockCursor<T> {
    pub fn new(t: T) -> BlockCursor<T> {
        BlockCursor(t)
    } 
}

impl<T: BlockRead> BlockCursor<T> {
    pub fn block_buf(&self, pos: impl Into<BlockPos>) -> &BlockBuf {
        self.0.block_buf(pos.into())
    }
}

impl<T: BlockWrite> BlockCursor<T> {
    pub fn block_buf_mut(&mut self, pos: impl Into<BlockPos>) -> &mut BlockBuf {
        self.0.block_buf_mut(pos.into())
    }
}

impl<T: BlockOperate + BlockRead + BlockWrite> BlockCursor<T> {
    pub fn get_block(&self, pos: impl Into<BlockPos>) -> Result<BlockId> {
        let state = self.0.read_block_state(pos.into())?;
        self.0.get_block_system().block_state_to_id(state).ok_or(Error::BlockNotFound)
    }

    pub fn set_block(&mut self, pos: impl Into<BlockPos>, block_id: impl Into<BlockId>) -> Result<()> {
        let new_state = self.0.get_block_system().block_id_to_state(block_id.into()).ok_or(Error::BlockNotFound)?;
        self.0.write_block_state(pos.into(), new_state)
    }
}

//

// consider macros
// this is a unique module

use std::io::{Cursor, BufRead, Write};

pub struct SignCursor<T>(T);

impl<T> SignCursor<T> {
    pub fn new(t: T) -> SignCursor<T> {
        SignCursor(t)
    }
}

pub trait SignRead {
    fn read_sign_lines(&self, lines_output: &mut [String; 4]) -> Result<()>;
}

pub trait SignWrite {
    fn write_sign_lines(&mut self, lines_input: &[impl ToString; 4]) -> Result<()>;
}

impl<T> SignRead for SignCursor<T> where T: AsRef<BlockBuf> {
    fn read_sign_lines(&self, lines_output: &mut [String; 4]) -> Result<()> {
        let cur = Cursor::new(self.0.as_ref());
        for (i, line) in cur.lines().enumerate() {
            lines_output[i] = line?;
        }
        Ok(())
    }
}

impl<T> SignWrite for SignCursor<T> where T: AsMut<BlockBuf> {
    fn write_sign_lines(&mut self, lines_input: &[impl ToString; 4]) -> Result<()> {
        let mut cur = Cursor::new(self.0.as_mut());
        for i in 0..4 {
            cur.write(lines_input[i].to_string().as_bytes())?;
            cur.write(b"\n")?;
        }
        Ok(())
    }
}

pub mod buf_world {
    use std::collections::HashMap;
    use super::{BlockSystem, BlockPos, BlockOperate, Result, 
                BlockState, BlockBuf, BlockWrite, BlockRead};

    type Chunk = [[[u8; 16]; 256]; 16];

    fn pos_split_to_chunk_index(pos: BlockPos) -> (i32, usize, usize, usize) {
       let (x, y, z) = pos.to_xyz();
       let chunk_index = x / 16 * 0xFFFF + z / 16;
       let in_chunk_x = (x % 16) as usize;
       let in_chunk_y = (y % 256) as usize;
       let in_chunk_z = (z % 16) as usize;
       (chunk_index, in_chunk_x, in_chunk_y, in_chunk_z)
    }

    pub struct BufWorld {
        chunks: HashMap<i32, Chunk>,
        special_pos_map: HashMap<u64, u16>, 
        buf_map: HashMap<u64, Vec<u8>>, 
        block_system: Box<BlockSystem>
    }

    impl BufWorld {
        pub fn new(bs: impl BlockSystem + 'static) -> BufWorld {
            BufWorld {
                chunks: HashMap::new(),
                special_pos_map: HashMap::new(), 
                buf_map: HashMap::new(), 
                block_system: Box::new(bs)
            }
        }

        #[inline]
        fn convert_and_check_contains(&self, pos: BlockPos) -> (u64, bool) {
            let pos = pos.to_u64_repr();
            (pos, self.special_pos_map.contains_key(&pos))
        }

        #[inline]
        fn init_chunk(&mut self, pos: BlockPos) -> (&mut Chunk, usize, usize, usize) {
            let (index, ix, iy, iz) = pos_split_to_chunk_index(pos);
            if !self.chunks.contains_key(&index) {
                self.chunks.insert(index, [[[0u8; 16]; 256]; 16]);
            }
            (self.chunks.get_mut(&index).unwrap(), ix, iy, iz)
        }

        #[inline]
        fn get_chunk(&self, pos: BlockPos) -> (Option<&Chunk>, usize, usize, usize) {
            let (index, ix, iy, iz) = pos_split_to_chunk_index(pos);
            (self.chunks.get(&index), ix, iy, iz)
        }
    }

    fn panic_block_buf_not_found(pos: u64) -> ! {
        panic!("Block buf not found at {:?}! Is your `set_block_state` used correct? 
                Or does this block need a special buffer? Or is the `BlockSystem` valid?", 
                BlockPos::from_u64_repr(pos));
    }

    impl BlockRead for BufWorld {
        fn read_block_state(&self, pos: BlockPos) -> Result<BlockState> {
            let repr = pos.to_u64_repr();
            let (chunk, ix, iy, iz) = self.get_chunk(pos);
            match chunk {
                Some(chunk) => {
                    let ri = chunk[ix][iy][iz];
                    Ok(BlockState { runtime_internal: ri as u16 })
                },
                None => Ok(self.special_pos_map.get(&repr)
                            .map(|&i| BlockState { runtime_internal: i })
                            .unwrap_or(BlockState { runtime_internal: 0 }))
            }
        }

        fn contains_block(&self, pos: BlockPos) -> bool {
            self.convert_and_check_contains(pos).1
        }

        fn block_buf(&self, pos: BlockPos) -> &BlockBuf {
            let pos = self.convert_and_check_contains(pos).0;
            self.buf_map.get(&pos).unwrap_or_else(|| panic_block_buf_not_found(pos))
        }
    }

    impl BlockWrite for BufWorld {
        fn write_block_state(&mut self, pos: BlockPos, new_state: BlockState) -> Result<()> {
            let repr = pos.to_u64_repr();
            let internal = new_state.runtime_internal;
            match internal { 
                0 => self.special_pos_map.remove(&repr), // delete block
                i if i > 0xFF => self.special_pos_map.insert(repr, internal),
                i if i <= 0xFF => {
                    let i = i as u8;
                    let (chunk, ix, iy, iz) = self.init_chunk(pos);
                    chunk[ix][iy][iz] = i;
                    None
                },
                _ => unreachable!()
            };
            match self.block_system.buf_for_block_state(new_state) {
                Some(buf) => self.buf_map.insert(repr, buf),
                None => self.buf_map.remove(&repr)
            };
            Ok(())
        }

        fn flush_blocks(&mut self) -> Result<()> {
            Ok(())
        }

        fn block_buf_mut(&mut self, pos: BlockPos) -> &mut BlockBuf {
            let pos = self.convert_and_check_contains(pos).0;
            self.buf_map.get_mut(&pos).unwrap_or_else(|| panic_block_buf_not_found(pos))
        }
    }

    impl BlockOperate for BufWorld {

        fn get_block_system(&self) -> &dyn BlockSystem {
            &*self.block_system
        } 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_buf_world() -> super::Result<()> {
        use super::{BlockCursor, SignCursor, SignRead, SignWrite};

        use super::hash_block_system::HashBlockSystem;
        let mut bs = HashBlockSystem::new();
        bs.register_new_block_unbuffered(None);
        bs.register_new_block_buffered("minecraft:sign");

        use super::buf_world::BufWorld;
        let world = BufWorld::new(bs);
        let mut block_cur = BlockCursor::new(world);
        block_cur.set_block((7, 7, 7), Some("minecraft:sign"))?;
        assert_eq!(Some("minecraft:sign"), block_cur.get_block((7, 7, 7))?);
        assert_eq!(None, block_cur.get_block((0, 0, 0))?);

        let buf = block_cur.block_buf_mut((7, 7, 7));
        let mut sign_cur = SignCursor::new(buf);
        let input = ["First line", "Then second", "And third", "Finally fourth"];
        sign_cur.write_sign_lines(&input)?;   
        assert_eq!(*block_cur.block_buf((7, 7, 7)), vec![
            70, 105, 114, 115, 116, 32, 108, 105, 110, 101, 10, 
            84, 104, 101, 110, 32, 115, 101, 99, 111, 110, 100, 10, 
            65, 110, 100, 32, 116, 104, 105, 114, 100, 10, 
            70, 105, 110, 97, 108, 108, 121, 32, 102, 111, 117, 114, 116, 104, 10]);

        let buf = block_cur.block_buf_mut((7, 7, 7));
        let cur = SignCursor::new(buf);
        let mut output = [String::new(),String::new(),String::new(),String::new()];
        cur.read_sign_lines(&mut output)?;
        assert_eq!(input, output);
        Ok(())
    }
}
