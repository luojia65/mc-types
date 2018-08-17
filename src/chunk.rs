// chunks are useful structures used in minecraft's network protocol
// as well as the world storage.
// a chunk contains 16*128*16 blocks. by referring to a block system,

pub use crate::pos::ChunkPos as Pos;

#[derive(Clone, Eq, PartialEq, Hash, Debug)] // does NOT derive Copy as it's expensive
pub struct Chunk {
    inner: [[[u16; 16]; 256]; 16]
} 

// pub trait ChunkRead 

// ChunkWrite

pub trait ReadExact { // chunk::ReadExact
    
    fn read_chunk_exact(&self, pos: Pos, buf: &mut Chunk) -> Result<()>; 
}

pub trait WriteExact { // chunk::WriteExact
    // write a whole chunk
    fn write_chunk_exact(&mut self, pos: Pos, chunk: &Chunk) -> Result<()>; 
}

pub trait Validate {

    fn contains_chunk(&self, pos: Pos) -> Result<bool>;
}
