# The DOOM.EXE File

```
Via pwads, a great many characteristics of the DOOM environment can
be changed: maps, pictures, sounds, etc. But there are also a lot of
neat things that can be done by patching the DOOM.EXE file itself.
There is a large collection of data at the end of the EXE file, and by
patching some bytes, we can turn literal values into variables. For
example, the player has a 16-unit "radius" which prevents him from
entering very small passageways. The player's radius can be made 1 and
his "height" 1, so he can enter mouse-sized crawlspaces. There are a
lot more exciting examples, such as invisible monsters, cyber-demons
that look like players, super-fast shotguns, and a hundred others, but
I won't describe all of that here. See appendix [A-4] for some EXE
utilities and documents. Here I will simply give the data that has
been figured out to date.
  I freely mix hex and decimal numbers below. Hopefully you can tell from
the context. All of the stuff below applies to registered version 1.2,
and some of it applies to version 1.666 also. This chapter has not yet
been completely updated for 1.666, but it soon will be.
```
