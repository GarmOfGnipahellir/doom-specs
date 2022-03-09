# PC Speaker Sound Effects

```
DP* entries in the directory refer to lumps that are sound data for
systems using the PC speaker.
  It's a quick and simple format. First is a <short> that's always 0,
then a <short> that's the number of bytes of sound data, then follow
that many bytes worth of sound data. That is, the lump's bytes will be
0, 0, N, 0, then N bytes of data. The DP* lumps range in size from around
10 bytes to around 150 bytes, and the data seem to range from 0 to 96
(0x00 to 0x60). The numbers obviously indicate frequency, but beyond
that I don't know the exact correlation in Hz, nor the time duration
of each byte worth of data. Feel free to figure this out and tell me.
```
