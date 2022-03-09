# ExMy or MAPxy

```
DOOM 1 levels have an ExMy label in a wad's directory. x is a single
(ASCII) digit 1-3 for the episode number and y is 1-9 for the mission
number.
  DOOM 2 levels have a MAPxy label in a wad's directory. xy can range
from (ASCII) 01 to 32, for the level number.
  This label just indicates that the lump names following it are part
of the designated level. The label does not actually point to a lump,
and the size field in the directory is 0. The assignment of lumps to
this level stops with either the next ExMy or MAPxy entry, or with a
non-map entry like TEXTURE1.
  Without these labels, there would be no way to differentiate amongst
the many lumps named "THINGS", "LINEDEFS", etc.
```
