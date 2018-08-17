//provide under-level world read and write operation
// as well as player data
//full-functional java-edition level
//https://minecraft.gamepedia.com/Level_format

use std::path::Path;
use std::fs;
use std::io::{self, Result};
use byteorder::{BigEndian, ReadBytesExt};
use crate::{block, chunk};

// reference to a world path
// unbuffered!
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
        let mut session_lock_file = fs::File::open(session_lock_path)?;
        session_lock_file.read_i64::<BigEndian>()
        // files are automatically closed when they go out of scope
    }

    fn read_region(&self, region_x: i32, region_z: i32) -> io::Result<()> {
        // todo return value type
        let file_name = format!("r.{}.{}.mca", region_x, region_z);
        let file_path = self.path.as_ref().join("region").join(file_name);
        let region_file = fs::File::open(file_path)?;
        
        //Ok(())
        unimplemented!();
    }

}


impl<P: AsRef<Path>> block::ReadExact for McJavaWorld<P> {
    fn read_block_exact(&self, _pos: block::Pos) -> Result<block::Meta> {
        unimplemented!()
    }

    fn contain_block_exact(&self, _pos: block::Pos) -> Result<bool> {
        unimplemented!()
    }
}

impl<P: AsRef<Path>> chunk::ReadExact for McJavaWorld<P> {
    fn read_chunk_exact(&self, _pos: chunk::Pos, _buf: &mut chunk::Chunk) -> Result<()>{
        unimplemented!()
    } 

    fn contains_chunk_exact(&self, _pos: chunk::Pos) -> Result<bool> {
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
        let level_dat_file = fs::File::open("./test_worlds/water_only/level.dat")?;
        let data = GzDecoder::new(level_dat_file).read_nbt_data()?;
        println!("{:?}", data);
        Ok(())
    }   
    
    #[test]
    fn read_chunk_test() -> io::Result<()> {
        use flate2::read::ZlibDecoder;
        use std::io::*;
        let mut file = fs::File::open("./test_worlds/water_only/region/r.1.1.mca")?;
        file.seek(SeekFrom::Start(53253))?;
        let mut buf = vec![0u8; 577];
        file.read_exact(&mut buf)?;
        for a_byte in buf.iter() {
            print!("{:X}, ", a_byte);
        }
        let cur = Cursor::new(buf);
        let data = ZlibDecoder::new(cur).read_nbt_data()?;
        println!("{:?}", data);
        Ok(())
    }   
}
