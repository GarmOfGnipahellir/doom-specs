# Picture Format

```
Each picture has three sections. First, an 8-byte header composed of
four short-integers. Then a number of long-integer pointers. Then the
picture's pixel/color data. See [A-1] for concise BNF style definitions,
here is a meatier explanation of the format:

(A) The header's four fields are:

  (1) Width. The number of columns of picture data.
  (2) Height. The number of rows.
  (3) Left offset. The number of pixels to the left of the center;
        where the first column gets drawn.
  (4) Top offset. The number of pixels above the origin;
        where the top row is.

  The width and height define a rectangular space or limits for drawing
a picture within. To be "centered", (3) is usually about half of the
total width. If the picture had 30 columns, and (3) was 10, then it
would be off-center to the right, especially when the player is standing
right in front of it, looking at it. If a picture has 30 rows, and (4)
is 60, it will appear to "float" like a blue soul-sphere. If (4) equals
the number of rows, it will appear to rest on the ground. If (4) is less
than that for an object, the bottom part of the picture looks awkward.
  With walls patches, (3) is always (columns/2)-1, and (4) is always
(rows)-5. This is because the walls are drawn consistently within their
own space (There are two integers in each SIDEDEF which can offset the
starting position for drawing a wall's texture within the wall space).

  Finally, if (3) and (4) are NEGATIVE integers, then they are the
absolute coordinates from the top-left corner of the screen, to begin
drawing the picture, assuming the VIEW is full-screen (i.e., the full
320x200). This is only done with the picture of the player's current
weapon - fist, chainsaw, bfg9000, etc. The game engine scales the
picture down appropriatelyif the view is less than full-screen.

(B) After the header, there are N = field (1) = <width> = (# of columns)
4-byte <long> integers. These are pointers to the data for each COLUMN.
The value of the pointer represents the offset in bytes from the first
byte of the picture lump.

(C) Each column is composed of some number of BYTES (NOT integers),
arranged in "posts":

  The first byte is the row to begin drawing this post at. 0 means
whatever height the header (4) upwards-offset describes, larger numbers
move correspondingly down.
  The second byte is how many colored pixels (non-transparent) to draw,
going downwards.
  Then follow (# of pixels) + 2 bytes, which define what color each
pixel is, using the game palette. The first and last bytes AREN'T drawn,
and I don't know why they are there. Probably just leftovers from the
creation process on the NeXT machines. Only the middle (# of pixels in
this post) are drawn, starting at the row specified in the first byte
of the post.
  After the last byte of a post, either the column ends, or there is
another post, which will start as stated above.
  255 (0xFF) ends the column, so a column that starts this way is a null
column, all "transparent". Draw the next column.


-----------------------------------------------
CHAPTER [6]: Flats (Floor and Ceiling Textures)
-----------------------------------------------

  All the lumpnames for flats are in the directory between the F_START
and F_END entries. Calling them flats is a good way to avoid confusion
with wall textures. There is no look-up or meta-structure in flats as
there is in walls textures. Each flat is 4096 raw bytes, making a square
64 by 64 pixels. This is pasted onto a floor or ceiling with the same
orientation as the automap would imply, i.e. the first byte is the color
at the NW corner, the 64th byte (byte 63, 0x3f) is the NE corner, etc.
  The blocks in the automap grid are 128 by 128, so four flats will fit
in each block. Note that there is no way to offset the placement of flats,
as can be done with wall textures. They are pasted according to grid lines
64 apart, reckoned from the coordinate (0,0). This allows flats to flow
smoothly even across jagged boundaries between sectors with the same
floor or ceiling height.

  As discussed in chapter [5], replacement and/or new-name flats don't
work right from pwad files unless they are all in the same wad.
  Theoretically, you can change all the flats want by constructing a
new DOOM.WAD or ALLFLATS.WAD pwad, but you have to make sure no floor
or ceiling uses an entry name which isn't in your F_ section. And you
have to include these four entries for DOOM 1 use, although you can
change their actual contents (pictures): FLOOR4_8, SFLR6_1, MFLR8_4,
and FLOOR7_2. The first three are needed as backgrounds for the episode
end texts. The last is what is shown "outside" the border of the display
window if the display is not full-screen.


[6-1]: Animated Flats
---------------------

  See Chapter [8-4-1] for a discussion of how the animated walls and
flats work. Unfortunately, the fact that the flats all need to be
lumped together in one wad file means that its not possible to change
the animations via a pwad file, unless it contains ALL the flats, which
amounts to several hundred k. Plus it is illegal to distribute the
original data, so to pass around modifications, either all the flats
must be all-new, or a utility must be used to construct a FLATS.WAD
on an end-user's hard drive, using the original DOOM.WAD plus the
additions. (Note: Bernd Kreimeier, listed in [A-5], has written a
utility that does just this. It is called DMADDS11).
```
