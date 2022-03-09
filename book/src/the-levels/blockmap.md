# BLOCKMAP

```
The BLOCKMAP is a pre-calculated structure that the game engine uses
to simplify collision-detection between moving things and walls. If a
level doesn't have a blockmap, it will display fine, but everybody walks
through walls, and no one can hurt anyone else.
  A concise definition of the BLOCKMAP is in appendix [A-1]. This is
the full explanation of it.
  The whole level is cut into "blocks", each is a 128 (hex 80) wide
square (the grid lines in the automap correspond to these blocks). The
BLOCKMAP is a collection of lists, one list for each block, which say
what LINEDEFS are wholly or partially in that block (i.e. part of the
line passes through the block). When the game engine needs to check
for an object/wall collision (to prevent a player from walking through
a wall or to explode a rocket when it hits a wall, etc.), it just looks
up the blocklist for the block that the object is in. This tells it
which linedefs it needs to check for collisions. Most blocks will have
few if any lines in them, so there will be a substantial savings in
processor time if it only checks a couple linedefs per object instead
of a thousand or so linedefs per object - it would have to check every
single linedef on the level if not for these blocklists.
  The blocks are also used for object/object collisions, but that is
not visible in the WAD format. During play, each block is also given a
dynamic "thing list", which tells what THINGS are currently in that
block. Again, this negates the need to check every moving object vs.
every other object for collisions - only a few need be tested.
  The BLOCKMAP is composed of three parts: the header, the offsets, and
the blocklists.
  The 8-byte header contains 4 short integers:

(1)     X coordinate of block-grid origin
(2)     Y coordinate of block-grid origin
(3)     # of columns (blocks in X direction)
(4)     # of rows (blocks in Y direction)

  The block-grid origin is the bottom-left corner of the bottom-left
(southwest) block. id's blockmap builder this origin point at 8 less
than the minimum values of x and y achieved in any vertex on the level.
  The number of columns and rows needs to be sufficient to contain
every linedef in the level. If there are linedefs outside the blockmap,
it will not be able to prevent monsters or players from crossing those
linedefs, which can cause problems, including the hall of mirrors effect.

  There are N blocks, N = (number of columns * number of rows). Each
block has a blocklist and an offset to that blocklist. Immediately
following the 8-byte header are N unsigned short integers. The first
is the offset in short-ints NOT bytes, from the start of the BLOCKMAP
lump to the start of the first blocklist. The last offset points to
blocklist (N-1), the last blocklist. Note that all these offsets are
UNSIGNED, so they can point to a location 65535 shorts (131070 bytes)
into the BLOCKMAP. If they were signed, they could only go up to 32767.
  The blocks are numbered going east (right) first, then north (up).
Block 0 is at the southwest corner (row 0, column 0). Block 1 is at
row 0, column 1. If there are 37 columns, then block 38 is at row 1,
column 0, etc.

  After the offsets come the blocklists. Each blocklist starts with
a short-int 0 (0x0000) and ends with a short-int -1 (0xffff). In between
are the numbers of every linedef which has any portion whatsoever in the
128 x 128 coordinate area of that block. If the block-grid origin is at
(0,0), then the first column is X = 0 to 127 inclusive, the second column
is X = 128 to 255 inclusive, etc. So a vertical line with X = 128 which
might seem to be on the border of two columns of blocks is actually only
in the easternmost/rightmost column. Likewise for the rows.
  The first linedef in the LINEDEFS lump is linedef number 0, and so on.
An "empty" block's blocklist only has the two shorts 0 and -1. A non-
empty block might have this as its blocklist: 0 330 331 333 -1. This
means that linedefs 330, 331, and 333 have some part of them pass through
this block. A block that has linedef 0 in it will go: 0 0 .. etc .. -1.

  There is an upper limit to how big a BLOCKMAP can be. Even empty
blocklists require at least 3 shorts - the 0, the -1, and the offset to
the blocklist. The offsets are unsigned shorts, which would imply a
maximum value of short #65535 ( = byte 131070) for the start of the last
blocklist. At a little over 6 bytes per blocklist, that would be a maximum
of around 21000 blocks (145 by 145 blocks, 18560 in coordinates). But the
actual limit is less. Experiments suggest that the maximum total size of
all the blocklists, not counting the offsets, is 65535 bytes. This limits
a minimalist level to around 120 blocks square (15360 in coordinates),
or a realistically complex level to around 100 blocks square (12800 in
coordinates).
```
