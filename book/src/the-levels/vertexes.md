# VERTEXES

```
These are the beginning and end points for LINEDEFS and SEGS. Each
vertice's record is 4 bytes in 2 <short> fields:

(1) X coordinate
(2) Y coordinate

  On the automap within the game, with the grid on (press 'G'), the
lines are 128 apart (0x80), two lines = 256 (0x100).
  A note on the coordinates: the coordinate system used for the vertices
and the heights of the sectors corresponds to pixels, for purposes of
texture-mapping. So a sector that's 128 high, or a multiple of 128, is
pretty typical, since many wall textures are 128 pixels high.
  And yes, the correct spelling of the plural of "vertex" is "vertices".
```
