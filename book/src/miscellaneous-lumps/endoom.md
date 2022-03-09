# ENDOOM

```
When you finally have to leave DOOM, you exit to dos, and a colorful
box of text appears. This is it. It is 4000 bytes, which are simply
stored in the screen memory area for 80x25 16-color text mode. Thus
it follows the same format as screen memory does: each character on
the screen takes up two bytes. The second byte of each pair is from
the (extended) ASCII character set, while the first byte of each pair
is the color attribute for that character. The color attribute can
be explained thus:

 bit 7    6   5   4   3   2   1   0
  +-----+---+---+---+---+---+---+---+
  |     |   .   .   |   .   .   .   |
  |Blink| Background|  Foreground   |
  |     |   .   .   |   .   .   .   |
  +-----+---+---+---+---+---+---+---+

  So the foreground color can be from 0-15, the background color can
be from 0-7, and the "blink" attribute is either on or off. All this
very low-level info can be found in many ancient PC programming books,
but otherwise it might be hard to locate...
```
