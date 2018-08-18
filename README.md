# mc-types

Types for Minecraft, including NBT support

## Performance  

### NBT read and write by ns/iter

| Item | mc-types | hematite-nbt | mojang official-nbt[^note] |
|:----:|:--------:|:------------:|:-------------------:|
| read_nbt_big | 2,382 ±225 | 2,441 ±126 | 32,619 ±28,842 |
| read_nbt_small | 298 ±36 | 321 ±69 | 8,234 ±7,227 |

[^note]: [Speed tester source](https://github.com/luojia65/nbt_speed_test), JVM startup time excluded.
