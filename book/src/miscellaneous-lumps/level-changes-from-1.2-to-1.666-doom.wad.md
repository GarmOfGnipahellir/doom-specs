# Level changes from 1.2 to 1.666 DOOM.WAD

```
Many people have experienced the error "Demo from a different game
version", because DOOM versions will only play back demos that were
recorded with the same version number. Theoretically, though, ANY
version can be converted to ANY other version. Versions up to 1.2
don't even require any modification, and versions 1.4 and up just
require that their first byte be changed. To change between the two
families (pre-1.4 and post-1.2) would just require extra header bytes
be added/shaved.
  But there are serious complications to converting demos. There have
been some minor changes and bug-fixes to the levels from version to
version. Since the demos only record player actions, they have NOTHING
about the level in them, so they depend on the level used for playback
to be the same as the level used for recording. Some kinds of differences
in the playback level can cause the demo to become "unsynchronized"
with the level, and players will seem to have gone crazy. For example,
if a deathmatch start-position is at a different location, when a
demo-player is spawned there, they will try to open doors that don't
exist at the new location, shoot at people who aren't there, etc.
The entire playback is ruined from that point on. Some examples of
changes that don't matter are different floor and wall textures, light
levels, and linedef "unpegged" flags. But most changes DO matter.
  Note that changes like (nomonsters) vs. (monsters), (respawn) vs.
(regular), (altdeath) vs. (regular deathmatch) will very quickly lead
to unsynchronized goofball players.
```
