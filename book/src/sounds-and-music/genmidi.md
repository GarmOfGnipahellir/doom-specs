# GENMIDI

```
This has something to do with General MIDI, obviously. This lump
has 3 sections: an 8-byte header (the ASCII text "#OPL_II#"), then
150 36-byte records (1 for each instrument), then 150 32-byte strings
(names of instruments, padded with zeros). Perhaps the 36 bytes contain
waveform information for the General MIDI standard instruments (this
guess is based on exactly one glance at a dump of the byte values,
so don't put too much faith in it).
```
