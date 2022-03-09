# Basics

```
The starting point is the concept of "WAD". It is not an acronym, it
just means a collection of data. Throughout this document, "WAD" or "wad"
will mean a file with a .WAD extension that contains data for the doom
engine to use.
  A WAD file has three parts:

  (1) a twelve-byte header
  (2) one or more "lumps"
  (3) a directory or "info table" that contains the names, offsets, and
        sizes of all the lumps in the WAD

  The header consists of three four-byte parts:

    (a) an ASCII string which must be either "IWAD" or "PWAD"
    (b) a 4-byte (long) integer which is the number of lumps in the wad
    (c) a long integer which is the file offset to the start of
          the directory

  The directory has one 16-byte entry for every lump. Each entry consists
of three parts:

    (a) a long integer, the file offset to the start of the lump
    (b) a long integer, the size of the lump in bytes
    (c) an 8-byte ASCII string, the name of the lump, padded with zeros.
          For example, the "DEMO1" entry in hexadecimal would be
          (44 45 4D 4F 31 00 00 00)

  A "lump" is just data, in one of several different formats. Some
contain sound data, some contain graphics data, some contain level
structure data, etc. These specs are mostly concerned with delineating
the formats of the various lump types. There are 10 different types of
map/level lump formats, each has a section in chapter [4] (sections 2-11).
There are 13 other types of lump formats, listed below with the section
where the format is explained, and the actual lump names in parentheses.
Also, Appendix [A-1] has definitions of the structures of all these
WAD elements.

  [8-1] palettes (PLAYPAL)
  [8-2] colormaps (COLORMAP)
  [8-3] dos exit text (ENDOOM)
  [8-6] demos (DEMO1, DEMO2, and DEMO3)
  [8-4] texture composition list (TEXTURE1 and TEXTURE2)
  [8-5] wall patch "number for name" indexing list (PNAMES)
  [7-4] midi mapping (GENMIDI)
  [7-5] Gravis UltraSound patch mappings (DMXGUS)
  [7-1] PC speaker sound effects (DP*)
  [7-2] Soundcard sound effects (DS*)
  [7-3] songs (D_*)
  [6]   flats (lumpnames between F_START and F_END)
  [5]   all other graphics (all other lumps)

  The "marker" and "label" lump names like "S_START" and "E1M1" (or
"MAP01") do not actually refer to lumps - they have zero length. They
merely serve to mark the beginning or end of a set of related lumps.

  It is possible to include other directory entries and lumps in a wad
file, e.g. an entry called CLOWNS could point to a lump that includes the
level creator's name, date of completion, and the latitude and longitude
of the Holy Grail. None of these non-standard entries will be used by
DOOM, nor will they cause it problems.
```
