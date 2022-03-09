# Music

```
D_* entries is the directory refer to lumps that are music. This
music is in the MUS file format, which goes like this:

offset  type    contents

0       ASCII   "MUS" and CTRL-Z (hex 4d 55 53 1a)
4      <short>  # of bytes of music data
6      <short>  # of bytes of header data (offset to start of music)
8      <short>  number of primary channels
10     <short>  number of secondary channels
12     <short>  number of instrument patches
14     <short>  0
16     <short>s instrument patch numbers
X to end  ?     Music data

  X is the header size (the second short). Drum patch numbers (greater
than 128) are 28 less than the numbers listed in the DMXGUS lump.
  What the exact format of the music data is, I don't know.
```
