# mc-types

Types for Minecraft, including NBT support

## Performance  

### NBT read and write speed by ns/iter  

| Item | mc-types | hematite-nbt | mojang official |
|:----:|:--------:|:------------:|:-------------------:|
| read_nbt_big | 2,382 ±225 | 2,441 ±126 | 32,619 ±28,842 |
| read_nbt_small | 298 ±36 | 321 ±69 | 8,234 ±7,227 |

Note: Smaller is better.  For [mojang official nbt implemetation](https://www.mojang.com/2012/02/new-minecraft-map-format-anvil/), the speed tester source is [here](https://github.com/luojia65/nbt_speed_test), JVM startup time excluded.
