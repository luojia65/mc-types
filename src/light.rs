// use crate::id::Id;

// level must be in [0, 15]
// pub struct Light {
//     level: u8 
// }

// OverworldLightSystem, NetherLightSystem, EndLightSystem
// pub trait LightSystem {

//     fn block_id_to_light(&self, id: Id) -> Light;
// }

// sky light; top-most block y

/*

block_light_sum = min(15, sum(block_light) 
where block_light = block_light_id - euclid_distance(pos, light_source)
//等等，比我想的复杂...

渲染颜色由block_light与sky_light计算得到，与染色玻璃无关

sky_light不与时间相关，夜晚也是15，
蜘蛛网等会减弱sky_light（与测量点与特殊方块位置相关）
半砖能挡住sky_light

light FLOWS like water!
 */

//looking_at_{liquid, block}, etc

// ALGORITHM
