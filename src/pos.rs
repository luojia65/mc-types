// x, y, z
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
}

impl From<(i32, i32, i32)> for BlockPos {
    fn from(src: (i32, i32, i32)) -> BlockPos {
        BlockPos::from_xyz(src.0, src.1, src.2)
    }
}
