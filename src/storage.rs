//provide under-level world read and write operation
// as well as player data
//full-functional java-edition level
//https://minecraft.gamepedia.com/Level_format

use std::path::Path;
use std::fs;
use std::io;
use byteorder::{BigEndian, ReadBytesExt};

// reference to a world path
pub struct McJavaWorld<P: AsRef<Path>> {
    path: P
}

impl<P: AsRef<Path>> McJavaWorld<P> {
    pub fn new(path: P) -> McJavaWorld<P> {
        McJavaWorld {
            path
        }
    }

    //create

    //sync_data

    //sync_all

    //world_metadata

    // no set_permissions here as it might change in the future

    // Is this process id? I think it's not time here.
    //todo
    #[allow(unused)]
    fn read_session_lock(&self) -> io::Result<i64> {
        let session_lock_path = self.path.as_ref().join("session.lock");
        let vec = fs::read(session_lock_path)?;
        let mut cur = io::Cursor::new(vec);
        cur.read_i64::<BigEndian>()
    }

}

use crate::block;
use std::io::Result;

impl<P: AsRef<Path>> block::ReadExact for McJavaWorld<P> {
    
    fn read_block_exact(&self, _pos: block::Pos) -> Result<block::Meta> {
        unimplemented!()
    }

    fn contain_block_exact(&self, _pos: block::Pos) -> Result<bool> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nbt::*;
    #[test]
    fn session_lock() -> io::Result<()> {
        let world = McJavaWorld::new("./test_worlds/water_only");
        println!("{}", world.read_session_lock()?);
        Ok(())
    }

    #[test]
    fn read_level_dat() -> io::Result<()> {
        use flate2::read::GzDecoder;
        let level_dat_path = "./test_worlds/water_only/level.dat";
        let vec = fs::read(level_dat_path)?;
        let cur = io::Cursor::new(vec);
        let data = GzDecoder::new(cur).read_nbt_data()?;
        println!("{:?}", data);
        Ok(())
    }   
}
