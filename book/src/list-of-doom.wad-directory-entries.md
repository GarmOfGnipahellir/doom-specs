# List of DOOM.WAD Directory Entries

```
There are over 2000 entries in the DOOM.WAD directory. Most of them
can be easily described in groups, and so are not explicitly mentioned
in this list. This includes the sprites (see [4-2-1] for sprite names
and [5] for the sprite lump naming system), the wall patches ([8-4] and
[8-5] have more info), the flats (chapter [6]), the sounds and songs
(chapter [7]), and the map data lumps (chapter [4]). All the others
are listed here.
  There have been several changes from version to version. The "Ver"
column indicates in which doom versions the lump exists:

___     no indication means it is in every version. Most are like this.
1.1     it was in 1.0 and 1.1, but not in 1.2 and later. It is obsolete.
1.2     it is not in 1.1 and earlier, only in 1.2 and up.
1.6     it is not in 1.2 and earlier, only in 1.666 and up.
r       it is only in the registered version, not the shareware.
1       it is only in DOOM 1, it is not in DOOM 2.
2       it is only in DOOM 2, it is not in DOOM 1.

  In the lump names, x (and y and e) indicates variable ASCII
characters, and * can be replaced by an ASCII string (up to the
8-byte lumpname limit).

LumpName  Ver   Description
--------  ---   -----------
PLAYPAL         fourteen 256 color palettes. See [8-1].
COLORMAP        maps colors in the palette down to darker ones. [8-2].
ENDOOM          text message displayed when you exit to DOS. [8-3].
DEMOx           x=1-3, are the demos. [8-6].
ExMy            subsequent entries define a single level's data. [4].
MAPxy     2     like ExMy, but for DOOM 2.
TEXTURE1        list of wall texture names and their composition data,
                  used in the SIDEDEF portion of each level. [8-4].
TEXTURE2  r     more wall texture compositions.
PNAMES          lists all lump names used as wall patches. [8-5].
GENMIDI         General Midi standard instrument data. [7-3].
DMXGUS          Gravis Ultra Sound instrument patches. [7-4].

D_ExMy          music for a doom 1 level. [7-2].
D_INTER         music played on the summary screen between levels.
D_INTRO         music played when the game starts.
D_INTROA  1.2   more introductory music.
D_VICTOR        music played on the victory text-screen after an episode.
D_BUNNY   r     music for while a certain rabbit has his story told...
D_*       2     music for a doom 2 level.

DP_*      vary  PC speaker sound effects. [7-1].
DS_*      vary  Soundcard sound effects. [7-1].

  All the remaining entries in the directory, except the flats between
F_START and F_END, and the "markers" like S_START, refer to lumps which
are pictures, in the doom/wad graphic format described in chapter [5].
The flats are also pictures, but in a format described in chapter [6].
  The next seven are full screen (320 by 200 pixel) pictures. After
that, ST* are status-bar pictures, WI* are for the screens between
levels, and M_* are for menus.

HELP1           Ad-screen says Register!, with some screen shots.
HELP2           Actual help, all the controls explained.
TITLEPIC        Maybe this is the title screen? Gee, I dunno...
CREDIT          People at id Software who created this great game.
VICTORY2  r     Screen shown after a victorious end to episode 2.
PFUB1     r     A nice little rabbit minding his own peas and queues...
PFUB2     r     ...a hint of what's waiting in Doom 2.

ENDx      r     x=0-6, big red "THE END" gets shot up.
AMMNUMx         x=0-9. Small grey digits for ammo count (15/200 etc).
STxBARy   1.1   x=M or A, y= L or R. Status bar used to be in pieces.
STCHAT    1.1   Status bar used to have a "chat" box.
STRSNUMx  1.1   x=0-9. Small red digits.
STWEAPx   1.1   x=0-5. COOL little weapon icons. Why'd they drop them?
STFRAGS   1.1   Tiny "FRAG" to be placed on top of part of status bar.
STBAR     1.2   Status Bar as used in deathmatches.
STGNUMx         x=0-9. Small grey digits used on the "Arms" panel.
STTNUMx         x=0-9. Big red digits used for Armor, Health, etc.
STTMINUS  1.6   Big red "-" used for negative frags.
STYSNUMx        x=0-9. Small yellow digits used on the "Arms" panel.
STTPRCNT        Big red % used in Armor and Health.
STKEYSx         x=0-5. Blue/Yellow/Red Keycards and Skullkeys.
STDISK          Disk, used at bottom right corner during disk accesses.
STCDROM   1.6   CD, used during CD-ROM accesses.
STARMS          "Arms" panel which replaces "Frags" in non-deathmatch.
STCFNxxx        xxx=033-095, also 121. Small red ASCII characters.
STFBx           x=0-3. Green/black/brown/red squares, for ST player faces.
STPBx           x=0-3. Squares with bottoms, for inter-level screens.
STFSTxy         x=0-4, y=0-2. Player face. x=0 is 100% health...x=4 is
                  very low health. y=0 is glancing right, y=2 left.
STFTLx0         x=0-4. Face looking left, player hurt from that direction.
STFTRx0         x=0-4. Face looking right.
STFOUCHx        x=0-4. Face looking surprised (hurt bad).
STFEVLx         x=0-4. Face with a grin (when pick up new weapons).
STFKILLx        x=0-4. Face with a grimace (when killing foes).
STFGOD0         Face with yellow eyes (invulnerable).
STFDEAD0        Dead face.
BRDR_*          Tiny pictures used as a border between a less-than-full
                  screen view and the "outside" marbleized zone. TL is
                  top left, BR bottom right, you can guess the rest.
WIBONUS   1.1   Medium sized red text "BONUS"
WISCORE   1.1   "SCORE"
WIMSTPx   1.1   x=0-3. Red text "ONE" to "FOUR".
WIMSTBx   1.1   x=0-3. Grey text "ONE" to "FOUR".
WIMINUS   1.6   Small red "-" used for negative frags.
WIMAPx          x=0-2. 320x200 maps used on inter-level screens for e1,2,3.
WIAe0x0y        patches used to animate inter-level maps.
WIURH0          "YOU ARE HERE" with an arrow pointing left.
WIURH1          "YOU ARE HERE" with an arrow pointing right.
WISPLAT         Splat mark that indicates a completed level.
WIOSTK          "KILLS"
WIOSTI          "ITEMS"
WIF             "FINISHED"
WIMSTT          "TOTAL"
WIOSTS          "SCRT"
WIOSTF          "F."
WITIME          "TIME"
WIPAR           "PAR"
WIMSTAR         "YOU"
WIPCNT          "%"
WINUMx          x=0-9. Medium sized red digits.
WICOLON         ":"
WISUCKS         "SUCKS"
WIFRGS          "FRAGS"
WILVxy          x=0-2, y=0-8. E(x+1)M(y+1) level names in grey/white letters.
WIPx            x=1-4. Red "P1" - "P4", for multiplayer summaries.
WIBPx           x=1-4. Grey "P1" - "P4"
WIKILRS         Small red "KILLERS" going sideways up, for deathmatches.
WIVCTMS         Small red "VICTIMS" for the top of the deathmatch chart.
WISCRT2         "SECRET"
WIENTER         "ENTERING"
M_DOOM          The DOOM logo
M_RDTHIS        Big red "Read This!"
M_OPTION        "Options"
M_QUITG         "Quit Game"
M_NGAME         "New Game"
M_SKULL1        The skull indicator with eyes lit.
M_SKULL2        The skull indicator with eyes unlit.
M_THERMO        The marker on e.g. the Sfx volume "thermometer".
M_THERMR        The right end of the thermometer.
M_THERML        The left end.
M_THERMM        The middle, repeated over and over.
M_ENDGAM        "End Game"
M_PAUSE         "Pause"
M_MESSG         "Messages:"
M_MSGON         "on"
M_MSGOFF        "off"
M_EPISOD        "Which Epsiode?"
M_EPI1          "Knee-Deep In The Dead"
M_EPI2          "The Shores Of Hell"
M_EPI3          "Inferno"
M_HURT          "Hurt me plenty."
M_JKILL         "I'm too young to die."
M_ROUGH         "Hey, not too rough."
M_SKILL         "Choose Skill Level:"
M_NEWG          "NEW GAME" (title of New Game menu)
M_ULTRA         "Ultra-Violence."
M_NMARE   1.2   "Nightmare!"
M_SVOL          "Sound Volume"
M_OPTTTL        "OPTIONS" (title of Options menu)
M_SAVEG         "Save Game"
M_LOADG         "Load Game"
M_DISP          "Display"
M_MSENS         "Mouse sensitivity"
M_GDHIGH        "high"
M_GDLOW         "low"
M_DETAIL        "Graphic Detail:"
M_DISOPT        "DISPLAY OPTIONS"
M_SCRNSZ        "Screen Size"
M_SGTTL         "SAVE GAME"
M_LGTTL         "LOAD GAME"
M_SFXVOL        "Sfx Volume"
M_MUSVOL        "Music Volume"
M_LSLEFT        Load/save box, left part
M_LSCNTR        Load/save box, center part (repeated)
M_LSRGHT        Load/save box, right part

  The following entries are markers that do not point to a lump; they
have zero size:

S_START         marks the start of the item/monster "sprite" section.
                  See chapter [5] for the naming convention used here.
S_END           is immediately after the last sprite.
P_START         marks the beginning of the wall patches.
P1_START          before the first of the shareware wall patches.
P1_END            after the last of the shareware wall patches.
P2_START  r       registered wall patches.
P2_END    r       registered wall patches.
P_END           marks the end of the wall patches.
F_START         marks the beginning of the flats (floor textures).
F1_START          shareware flats.
F1_END            shareware flats.
F2_START  r       registered flats.
F2_END    r       registered flats.
F_END           marks the end of the flats.
```
