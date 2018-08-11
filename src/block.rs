use crate::pos::BlockPos;
use crate::id::Id;

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
// not suitable for None (transparent block)
pub trait BlockSystem {
    // None: no such block. Some(id): block found.
    fn block_state_to_id(&self, state: BlockState) -> Option<Id>;
    // None: no such state. Some(state): found state.
    fn block_id_to_state(&self, id: Id) -> Option<BlockState>;

    fn buf_for_block_state(&self, state: BlockState) -> Option<BlockBuf>;
}

pub mod hash_block_system {
    use std::collections::{HashMap, HashSet};
    use super::{Id, BlockState, BlockBuf, BlockSystem};

    #[derive(Default)]
    pub struct HashBlockSystem<'a> {
        state_to_id: HashMap<u16, Id<'a>>,
        id_to_state: HashMap<Id<'a>, u16>,
        state_should_create: HashSet<u16>,
        next_state: u16
    }

    impl HashBlockSystem<'a> {
        pub fn new() -> HashBlockSystem<'a> {
            Default::default()
        }

        pub fn register_new_block_unbuffered(&mut self, id: impl Into<Id<'a>>) {
            let id = id.into();
            self.reg_block(id.clone());
            self.reg_buf(id, false);
            self.next_state += 1;
        }

        pub fn register_new_block_buffered(&mut self, id: impl Into<Id<'a>>) {
            let id = id.into();
            self.reg_block(id.clone());
            self.reg_buf(id, true);
            self.next_state += 1;
        }

        #[inline]
        fn reg_block(&mut self, id: Id<'a>) {
            if !self.id_to_state.contains_key(&id) {
                self.id_to_state.insert(id.clone(), self.next_state);
                self.state_to_id.insert(self.next_state, id);
            }
        } 

        #[inline]
        fn reg_buf(&mut self, id: Id, buffered: bool) {
            let state = self.id_to_state.get(&id).unwrap();
            match buffered {
                true => self.state_should_create.insert(*state),
                false => self.state_should_create.remove(state),
            };
        } 
    }

    impl BlockSystem for HashBlockSystem<'a> {

        fn block_state_to_id(&self, state: BlockState) -> Option<Id> {
            self.state_to_id.get(&state.runtime_internal).map(|s| s.to_owned())
        }

        fn block_id_to_state(&self, id: Id) -> Option<BlockState> {
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
    pub fn get_block(&self, _pos: impl Into<BlockPos>) -> Result<Id<'a>> {
        use std::borrow::Cow;
        Ok(Id::from(Cow::from("minecraft:stone")))
    }

    pub fn set_block(&mut self, _pos: impl Into<BlockPos>, _block_id: impl Into<Id<'a>>) -> Result<()> {
        Ok(())
    }
}

//

// consider macros
// this is a unique module

mod sign {
    use super::Result;
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

}