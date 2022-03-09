# Pwads

```
There are two types of wad files. The original DOOM.WAD and DOOM2.WAD
files are "IWAD"s, or "Internal wads", meaning they contain all of the
data necessary to play. The other type is the "PWAD" file, "Patch wad",
an external file which has the same structure, but with far fewer entries
in the directory. The data in a pwad is substituted for the original data
in the DOOM.WAD, thus allowing for much easier distribution of new levels.
Only those resources listed in the pwad's directory are changed,
everything else is loaded from the IWAD. All external wads should have
the "PWAD" indicator, as id has requested.
  A typical pwad might contain new data for a single level, in which
case it would contain the 10 lumps and 11 directory entries necessary
to define the level (as described in chapter [4]).
  A pwad file may contain more than one level or parts of levels, in
addition to replacement graphics, sounds, etc. (as of version 1.666,
sprites and flats do NOT work from pwads - see chapter [5] for more).
In fact, there is apparently no limit to how many entries may be in a
pwad. The original doom levels are pretty complicated, and they are
from 50-200 kilobytes each in size, uncompressed.
  Pwad files need to have the extension .WAD to work. Many of them have
descriptive names, e.g. if J.R.R. Tolkien made a new level, he might call
it GONDOLIN.WAD - to use this level, a player would type

  DOOM -FILE GONDOLIN.WAD

at the command line, along with any other parameters. More than one
external file can be added, thus in general:

  DOOM -FILE [pwad_filename] [pwad_filename] [pwad_filename] ...

  If there are duplicate entries amongst the directories of all the
wads being "added", the pwads listed LAST take precedence.
  When the game loads, a "modified game" message will appear if there
are any pwads involved, reminding the player that id Software will not
give technical support or answer questions regarding modified levels.
  With DOOM version 1.666, there is also the @responsefile option for
listing command line parameters and -file specifications. See the DOOM
README or the latest FAQ for more information. Also, there are numerous
"front-end" utilities that make it easier to play pwads, e.g. load several
external files at once, warp to certain levels, specify options, etc.
```
