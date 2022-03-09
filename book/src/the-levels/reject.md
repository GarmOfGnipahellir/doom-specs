# REJECT

```
The REJECT lump is used to help speed play on large levels. It can
also be used for some special effects like monsters in plain sight who
CANNOT attack or see players.
  The size of a REJECT in bytes is (number of SECTORS ^ 2) / 8, rounded
up. It is an array of bits, with each bit controlling whether monsters
in a given sector can detect and/or attack players in another sector.
  Make a table of sectors vs. sectors, like this:

         sector that the player is in
              0  1  2  3  4
            +---------------
sector    0 | 0  1  0  0  0
that      1 | 1  0  1  1  0
the       2 | 0  1  0  1  0
monster   3 | 0  1  1  1  0
is in     4 | 0  0  1  0  0

  A 1 means the monster cannot become activated by seeing a player, nor
can it attack the player. A 0 means there is no restriction. All non-
deaf monsters still become activated by weapon sounds that they hear
(including the bare fist!). And activated monsters will still pursue
the player, but they will not attack if their current sector vs. sector
bit is "1". So a REJECT that's set to all 1s gives a bunch of pacifist
monsters who will follow the player around and look menacing, but never
actually attack.
  How the table turns into the REJECT resource:
  Reading left-to-right, then top-to-bottom, like a page, the first bit
in the table becomes bit 0 of byte 0, the 2nd bit is bit 1 of byte 0,
the 9th bit is bit 0 of byte 1, etc. So if the above table represented
a level with only 5 sectors, its REJECT would be 4 bytes:

10100010 00101001 01000111 xxxxxxx0 (hex A2 29 47 00, decimal 162 41 71 0)

  In other words, the REJECT is a long string of bits which are read
from least significant bit to most significant bit, according to the
lo-hi storage scheme used in a certain "x86" family of CPUs.
  Usually, if a monster in sector A can't detect a player in sector B,
then the reverse is true too, thus if sector8/sector5 is set, then
sector5/sector8 will be set also. Same sector prohibitions, e.g. 0/0,
3/3, etc. are only useful for special effects (pacifist monsters), or
for tiny sectors that monsters can't get to anyway.

  The REJECT array was designed to speed the monster-detection process.
If a sector pair is prohibited, the game engine doesn't even bother
checking line-of-sight feasibility for the monster to "see" the player
and consider attacking. When a level has hundreds of monsters and
hundreds of sectors, a good REJECT can prevent the drastic slowdowns
that might otherwise occur (even fast CPUs can fall victim to this
phenomenon).
```
