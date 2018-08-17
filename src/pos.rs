// x, y, z
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct BlockPos(u64);

impl BlockPos {
    pub fn from_u64_repr(inner: u64) -> BlockPos {
        BlockPos(inner)
    }

    pub fn from_xyz(x: i32, y: i32, z: i32) -> BlockPos {
        let (x, y, z) = (x as u64, y as u64, z as u64);
        BlockPos(((x & 0x3FFFFFF) << 38) | ((y & 0xFFF) << 26) | (z & 0x3FFFFFF))
    }

    pub fn to_u64_repr(&self) -> u64 {
        self.0
    }

    pub fn to_xyz(&self) -> (i32, i32, i32) {
        let (x, y, z) = (self.0 >> 38, (self.0 >> 26) & 0xFFF, self.0 << 38 >> 38);
        let (x, y, z) = (x as i32, y as i32, z as i32);
        (if x >= 33554432 {  x - 67108864 } else { x },
        if y >= 2048 { y - 4096 } else { y }, 
        if z >= 33554432 { z - 67108864 } else { z })
    }

    pub fn to_chunk_pos(&self) -> ChunkPos {
        let (x, _, z) = self.to_xyz();
        ChunkPos::from_xz(x / 16, z / 16)
    }
}

impl From<(i32, i32, i32)> for BlockPos {
    fn from(src: (i32, i32, i32)) -> BlockPos {
        BlockPos::from_xyz(src.0, src.1, src.2)
    }
}

// no magic here
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct ChunkPos(i32, i32);

impl ChunkPos {
    pub fn from_xz(chunk_x: i32, chunk_z: i32) -> ChunkPos {
        ChunkPos(chunk_x, chunk_z)
    } 

    pub fn to_xz(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn block_pos() {
        let p1 = BlockPos::from_xyz(10, 20, 30);
        let p2 = BlockPos::from((10, 20, 30));
        let p3 = BlockPos::from((-10, 20, -30));
        assert_eq!(p1, p2);
        assert_ne!(p2, p3);
        assert_eq!(p1.to_u64_repr(), 2750121246750);
        let p4 = BlockPos::from_u64_repr(2750121246750);
        assert_eq!(p2, p4);
    }

    #[test]
    fn chunk_pos() {
        let p1 = ChunkPos::from_xz(10, -10);
        let p2 = ChunkPos::from_xz(10, -10);
        let p3 = ChunkPos::from_xz(-10, 10);
        assert_eq!(p1, p2);
        assert_ne!(p2, p3);
    }
}
