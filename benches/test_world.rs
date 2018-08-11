// #![feature(test)]
// extern crate test;
// extern crate mc_types;

// use test::*;
// use mc_types::block::*;
// use mc_types::block::buf_world::BufWorld;
// use mc_types::block::hash_block_system::HashBlockSystem;
// use mc_types::pos::BlockPos;

// fn test_block_system() -> impl BlockSystem {
//     let mut bs = HashBlockSystem::new();
//     bs.register_new_block_unbuffered(None);
//     bs.register_new_block_unbuffered("minecraft:stone");
//     bs.register_new_block_buffered("minecraft:sign");
//     bs
// }

// #[bench]
// fn world_block_state_read(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     b.iter(|| {
//         let _ = world.read_block_state(BlockPos::from_xyz(7, 7, 7));
//     });
//     Ok(())
// }

// #[bench]
// fn world_block_state_write(b: &mut Bencher) -> Result<()> {
//     let mut world = BufWorld::new(test_block_system());
//     let s = world.read_block_state(BlockPos::from_xyz(7, 7, 7)).unwrap();
//     b.iter(|| {
//         let _ = world.write_block_state(BlockPos::from_xyz(7, 7, 7), s);
//     });
//     Ok(())
// }

// #[bench]
// fn world_block_get_some(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     cur.set_block((7, 7, 7), "minecraft:sign")?;
//     b.iter(|| {
//         let _ = cur.get_block((7, 7, 7));
//     });
//     Ok(())
// }

// #[bench]
// fn world_block_get_none(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let cur = BlockCursor::new(world);
//     b.iter(|| {
//         let _ = cur.get_block((7, 7, 7));
//     });
//     Ok(())
// }

// #[bench]
// fn world_block_set_without_buf(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     b.iter(|| {
//         let _ = cur.set_block((7, 7, 7), "minecraft:stone");
//     });
//     Ok(())
// }

// #[bench]
// fn world_block_set_delete(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     b.iter(|| {
//         let _ = cur.set_block((7, 7, 7), None);
//     });
//     Ok(())
// }

// #[bench]
// fn world_block_set_with_buf(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     b.iter(|| {
//         let _ = cur.set_block((7, 7, 7), "minecraft:sign");
//     });
//     Ok(())
// }

// #[bench]
// fn world_sign_read_empty(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     cur.set_block((7, 7, 7), "minecraft:sign")?;
//     let buf = cur.block_buf_mut((7, 7, 7));
//     let cur = SignCursor::new(buf);
//     b.iter(|| {
//         let mut output = [String::new(),String::new(),String::new(),String::new()];
//         let _ = cur.read_sign_lines(&mut output);
//     });
//     Ok(())
// }

// #[bench]
// fn world_sign_read_full(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     let _ = cur.set_block((7, 7, 7), "minecraft:sign");
//     let buf = cur.block_buf_mut((7, 7, 7));
//     let mut cur = SignCursor::new(buf);
//     cur.write_sign_lines(&[
//         "1234567890123456",
//         "1234567890123456",
//         "1234567890123456",
//         "1234567890123456"
//     ])?;
//     b.iter(|| {
//         let mut output = [String::new(),String::new(),String::new(),String::new()];
//         let _ = cur.read_sign_lines(&mut output);
//     });
//     Ok(())
// }

// #[bench]
// fn world_sign_write_empty(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     let _ = cur.set_block((7, 7, 7), "minecraft:sign");
//     let buf = cur.block_buf_mut((7, 7, 7));
//     let mut cur = SignCursor::new(buf);
//     b.iter(|| {
//         let _ = cur.write_sign_lines(&["", "", "", ""]);
//     });
//     Ok(())
// }

// #[bench]
// fn world_sign_write_full(b: &mut Bencher) -> Result<()> {
//     let world = BufWorld::new(test_block_system());
//     let mut cur = BlockCursor::new(world);
//     let _ = cur.set_block((7, 7, 7), "minecraft:sign");
//     let buf = cur.block_buf_mut((7, 7, 7));
//     let mut cur = SignCursor::new(buf);
//     b.iter(|| {
//         let _ = cur.write_sign_lines(&[
//             "1234567890123456",
//             "1234567890123456",
//             "1234567890123456",
//             "1234567890123456"
//         ]);
//     });
//     Ok(())
// }

// // // 下面演示读取方块
// // let mut world = FileWorld::new("~/my_mc_world"); // 是个双重buffer世界存储结构
// // let mut cur = WorldCursor::new(world); // 创建世界游标
// // let block = cur.get_block((123, 45, 6789))?; // 从坐标读取方块。问号是rust错误处理的方式之一
// // println!("{:?}", block); // 打印该方块的调试信息

// // // 下面演示blockentity的用法。比如说在(123, 45, 6789)坐标有一个木牌
// // let sign_cur = SignCursor::new(cur.block_buf_mut((123, 45, 6789))); // 创建木牌游标
// // sign_cur.write_sign_lines(&["第一行", "第二行", "第三行", "第四行"])?; // 改变木牌文字

