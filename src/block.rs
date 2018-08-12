use crate::pos::BlockPos;
use crate::id::BlockId;

// basic data of blocks, including id, meta and sometimes facing
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BlockState {
    runtime_internal: u16
}

use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    UnsupportedBlock,
    IoError(IoError)
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

use std::result::Result as StdResult;

// result of block operations
pub type Result<T> = StdResult<T, Error>;

// buffer of a block.
// some blocks need a buffer to store extra value other than its state,
// such as text lines for signs, orientation for skulls, content for signs, etc
pub type BlockBuf = Vec<u8>;

// runtime system, may NOT vary between client editions & versions
// only available for blocks that are non-transparent (not None)
pub trait BlockSystem {
    // Some(id): block found for this state.
    // None: block not found. 
    fn block_state_to_id(&self, state: BlockState) -> Option<BlockId>;
    // Some(state): found state for this id.
    // None: block not found. 
    fn block_id_to_state(&self, id: BlockId) -> Option<BlockState>;
    // None: this block does not need a buffer.
    // Some(buf): creates a buffer for this block.
    fn buf_for_block_state(&self, state: BlockState) -> Option<BlockBuf>;
}

// Allows for reading blocks from a source
pub trait BlockRead {
    // Ok(Some(state)): Reading successful, block state is `state`
    // Ok(None): Reading successful, no block here
    // Err(error): Reading failed
    fn read_block_state(&self, pos: BlockPos) -> Result<Option<BlockState>>;

    // Check if there is a block
    fn contains_block(&self, pos: BlockPos) -> bool;

    // Callers must ensure that there is a buf.
    // If there isn't a buf here, panic.
    fn block_buf(&self, pos: BlockPos) -> &BlockBuf;
}

// A trait for objects thich are block-oriented sinks
pub trait BlockWrite {
    // Ok(()): Writing successful
    // Err(error): Writing failed with error
    fn write_block_state(&mut self, pos: BlockPos, new_state: BlockState) -> Result<()>;

    // Ensure that all intermediately buffered blocks reach their destination
    fn flush_blocks(&mut self) -> Result<()>;
    
    // Must ensure that there is a buf, or program would panic
    fn block_buf_mut(&mut self, pos: BlockPos) -> &mut BlockBuf;
}

// A trait for allows opertions depending on BlockSystem
pub trait BlockOperate {

    // Ensure that the block system does not change the stategy of one block
    // during the whole lifetime of Self.
    fn block_system(&self) -> &dyn BlockSystem;
}

// wraps a block storage, providing it with getters and setters
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

impl<T: BlockOperate + BlockRead> BlockCursor<T> {
    // Ok(Some(id)): Reading successful, block id is `id`
    // Ok(None): Reading successful, no block here
    // Err(error): Reading failed
    pub fn get_block(&self, pos: impl Into<BlockPos>) -> Result<Option<BlockId>> {
        let pos = pos.into();
        let state = self.0.read_block_state(pos)?; // Option<BlockId>
        match state {
            Some(state) => {
                match self.0.block_system().block_state_to_id(state) {
                    Some(id) => Ok(Some(id)),
                    None => Err(Error::UnsupportedBlock)
                }
            }
            None => Ok(None) 
        }
    }
}

impl<T: BlockOperate + BlockWrite> BlockCursor<T> {
    pub fn set_block(&mut self, _pos: impl Into<BlockPos>, _block_id: impl Into<BlockId>) -> Result<()> {
        Ok(())
    }
}
