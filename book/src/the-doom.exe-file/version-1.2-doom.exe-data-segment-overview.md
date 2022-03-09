# Version 1.2 DOOM.EXE Data Segment Overview

```
The data begins at 0x6f414 (455700) and continues to the end of the
file, 0x8db27 (580391). Here's an overview of the sections:

start length what

6f414  3d30  TEXT STRINGS
73412  1a34  various unknowns, probably to do with I/O, sound, mouse, etc.
74bf8 10000  looks like hard-coded math tables, for speed?
84bf8   148  misc.
84d40    82  gamma correction messages
84dc2   280  "are you sure you want to quit" messages
85042   3a2  MENUS (new game, load game, etc.)
853e4   140  ?
85524   36c  configuration options and defaults, like in DEFAULT.CFG
85890   174  ?
85a04    60  ?
85a64    54  ?
85ab8    c4  ?
85b7c    20  max ammo at start, and ammo per thing
85b9c    c0  ammo type and frame #s for the weapons
85c5c   188  ANIMATED WALLS and FLOORS
85de4   258  SWITCH-WALLS
8603c    c0  ?
860fc    d4  ?
861d0   500  5 colormaps for use with the gamma correction setting 0-4
866e4    fc  ?
867e0    40  pointers to chatmacros, "Green:", etc.
86820    88  pointers to level names, used on Automap
868a8    d8  splat mark coordinates for end-level screen
86980   5a8  wimap patch animations for end-level screen
86f28   224  SONG NAMES list of pointers
8714c   8b8  SOUND TABLE
87a04   1a4  SPRITE NAMES list of pointers
87ba8  3800  STATE TABLE
8b3a8    20  ?
8b3c8  2368  THING TABLE
8d730   3fd  ?
```
