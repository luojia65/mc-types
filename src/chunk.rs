// chunks are useful structures used in mc network protocol
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

    fn contains_chunk(&self, pos: Pos) -> Result<bool>;
}

pub trait WriteExact { // chunk::WriteExact
    // write a whole chunk
    fn write_chunk_exact(&mut self, pos: Pos, chunk: &Chunk) -> Result<()>; 
}

pub struct Cursor<I> { // chunk::Cursor
    inner: I,
    pos: Pos
}

// consider macros?
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

pub trait Read { // chunk::Read

    fn read_chunk(&self, buf: &mut Chunk) -> Result<()>;
}

pub trait Write { // chunk::Write

    fn write_chunk(&mut self, new_chunk: &Chunk) -> Result<()>;
}

impl<I: ReadExact> Read for Cursor<I> {
    fn read_chunk(&self, buf: &mut Chunk) -> Result<()> {
        self.inner.read_chunk_exact(self.pos, buf)
    } 
} 

impl<I: WriteExact> Write for Cursor<I> {
    fn write_chunk(&mut self, new_chunk: &Chunk) -> Result<()> {
        self.inner.write_chunk_exact(self.pos, new_chunk)
    }
}

