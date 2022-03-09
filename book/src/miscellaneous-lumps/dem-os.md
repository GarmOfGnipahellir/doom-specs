# DEMOs

```
If you start DOOM and do nothing, after a few seconds, it automatically
shows a demo of play on some level. Also, external demos can be recorded
and played back by using the command line parameters explained in the
README and/or the DOOM FAQ. All external demos have a .LMP extension
which the DOOM OS attaches; you only type the [demoname] without the
.LMP extension.
  The DOOM.WAD lumps DEMO1, DEMO2, and DEMO3 are in exactly the same
format as these external .LMP files. Strictly speaking, the "demo"
format should not be called the "LMP" format, because any external
file without a wadfile header, i.e. it is just raw data, is a "lump"
and deserves the .LMP extension.

  A DOOM demo has three parts:

  (1) header - 7 or 13 bytes
  (2) data recording player moves - 4 bytes per player per gametic
  (3) quit byte - equals 128 (0x80)

(1) There are two different kinds of header depending on the version of
DOOM used to record the demo. Versions up to 1.2 use a 7-byte header:

  byte  range   purpose

0       0-4     skill level. 0="I'm too young to die", 4="Nightmare!"
1       1-3     episode.
2       1-9     mission/map.
3       0-1     player 1 is present if this is 1.
4       0-1     player 2.
5       0-1     player 3.
6       0-1     player 4.

  Versions after 1.2 use a 13-byte header:

byte    range   purpose

0       104-106 version. 104=1.4 beta, 105=1.5 beta, 106=1.6 beta or 1.666
1       0-4     skill level. 0="I'm too young to die", 4="Nightmare!"
2       1-3     episode. In DOOM 2 this is always 1.
3       1-32    mission/map/level. In DOOM 1, it's 1-9. In DOOM 2, it's 1-32.
4       0-2     mode. 0=single or cooperative, 1=deathmatch, 2=altdeath
5       0-      respawn. 0=no respawn parameter, (any other value)=respawn.
6       0-      fast. 0=no fast parameter, (any other value)=fast.
7       0-      nomonsters. 0=monsters exist, (any other value)=nomonsters.
8       0-3     viewpoint. 0=player 1's status bar, ..., 3=player 4.
9       0-1     player 1 is present if this is 1.
10 0x0a 0-1     player 2.
11 0x0b 0-1     player 3.
12 0x0c 0-1     player 4.

(2) The player-move data is recorded in 4-byte chunks. Every 1/35 of a
second is a gametic, and for every gametic, there is one 4-byte chunk
per player. So the time duration of a demo (in seconds) is approximately
equal to its length in bytes divided by (140 * number_of_players).

  The four bytes recording each player's actions are:

  (a) Forward/Backward Movement.
  (b) Strafe Right/Left Movement.
  (c) Turn Left/Right.
  (d) other actions - use/activate, fire, change weapons.

  The first three are signed bytes (i.e. of type <char>).

  (a) Ranges from -127 to 127, negative numbers are backward movement,
      positive numbers are forward movement. Without the -turbo option
      above 100, values outside -50..50 cannot be achieved. With a
      keyboard or joystick, these are the regular values:

      Move forward:   25 (0x19)   with Speed on:  50 (0x32)
      Move backward: -25 (0xE7)   with Speed on: -50 (0xCE)

      Fancy mouse use can achieve any number in the range.

  (b) Ranges from -127 to 127, negative numbers are left-strafe movement,
      positive numbers are right-strafe movement. The keyboard values are:

      Strafe right: 24  (0x18)    with Speed on:  50 (0x32)
      Strafe left: -24  (0xE8)    with Speed on: -50 (0xCE)

  (c) Ranges from -127 to 127, negative numbers are right turns, positive
      numbers are left turns. The keyboard values vary from version to
      version, but are all in the range -5..5, and that's with Speed on.

      Using the mouse can achieve much higher numbers here. I doubt if
      the maximums of 127 and -127 can actually be achieved in play,
      though.

  (d) the bits of this byte indicate what actions the player is engaged in:

      bit 0     Fire current weapon
      bit 1     Use (a switch, open a door, etc.)
      bit 2     Change weapon to the one indicated in bits 3-5:

      bits 5-3 = 000 Fist or Chainsaw
                 001 Pistol
                 010 Shotgun
                 011 Chaingun
                 100 Rocket Launcher
                 101 Plasma Rifle
                 110 BFG 9000
                 111 Super Shotgun (DOOM 2 only)

      bit 6     unused
      bit 7     indicates a special action which alters the meanings
                  of the other bits:

                bits 1-0 = 01 pause or unpause
                         = 10 save game in slot # recorded in bits 4 to 2
                                (slot number can thus be 0 to 7 but
                                 should NOT be 6 or 7 or else!)

  There might be other special actions. The save game action happens
during replay of the demo, so be careful when playing demos if you
have important savegames! One or more of them could conceivably get
overwritten.

(3) The last byte of a demo has the value 128 (0x80)
```
