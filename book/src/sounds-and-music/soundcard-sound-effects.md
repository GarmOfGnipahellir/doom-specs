# Soundcard Sound Effects

```
DS* entries in the directory refer to lumps that are sound data for
systems using soundcards.
  This data is in a RAW format for 8-bit 11 KHz mono sound - first is
an 8-byte header composed of 4 unsigned short integers:

(1) 3           (means what?)
(2) 11025       (the sample rate, samples per second)
(3) N           (the number of samples)
(4) 0

  Each sample is a single byte, since they are 8-bit samples. The
maximum number of samples is 65535, so at 11 KHz, a little less than
6 seconds is the longest possible sound effect.
```
