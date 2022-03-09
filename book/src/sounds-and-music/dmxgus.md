# DMXGUS

```
This lump is used to do instrument patch mappings on the Gravis
Ultra-Sound soundcard. It's in a very simple format - ASCII text!
Here's the start and end of the DMXGUS lump from DOOM 1 version 1.2,
which is 200 lines, of which the first 10 are comments:

#Purpose: Different size patch libraries for different memory sizes.
#         The libraries are built in such a way as to leave 8K+32bytes
#         after the patches are loaded for digital audio.
#
#Revision History: 06/22/93 - Fixed problem with 512K patch library
#                  07/26/93 - patch names changed in various releases
#
#
#Explanation of Columns: Patch #  256K  512K  768K  1024K  Patch Name
#
0, 2, 1, 1, 1, acpiano
1, 2, 1, 1, 1, britepno
2, 2, 1, 1, 1, synpiano
.
.
.
213, 128, 128, 128, 128, castinet
214, 128, 128, 128, 128, surdo1
215, 128, 128, 128, 128, surdo2
```
