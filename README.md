# mc-types

Types for Minecraft, including NBT support

## Performance  

### NBT read and write by ns/iter

| Item | mc-types | hematite-nbt |
|:----:|:--------:|:------------:|
| read_nbt_big | 2,382 ±225 | 2,441 ±126 |
| read_nbt_small | 298 ±36 | 321 ±69 |
