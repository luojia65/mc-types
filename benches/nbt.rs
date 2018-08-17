#![feature(test)]
use test::*;
use mc_types::nbt::*;

#[bench]
fn read_nbt_big(b: &mut Bencher){
    b.iter(|| {
        let mut cur = std::io::Cursor::new(TEST_BIG_UNCOMPRESSED);
        let _data = cur.read_nbt_data();
    });
}

#[bench]
fn read_nbt_small(b: &mut Bencher){
    let mut blob = vec![
0xa, // compound #1
0x0, 0x5, 0x68, 0x65, 0x6c, 0x6c, 0x6f, // "hello"
    0x1, // byte
    0x0, 0x4, 0x62, 0x79, 0x74, 0x65, // "byte"
        0x80, // -128
0x0, // end of #1
];      
    b.iter(|| {
        let mut cur = std::io::Cursor::new(&mut blob);
        let _data = cur.read_nbt_data();
    });
}