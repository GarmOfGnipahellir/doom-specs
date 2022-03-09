# PNAMES

```
This is a list of all the names of lumps that are going to be used
as wall patches. DOOM assigns numbers to these names, in the order
that they are listed. The numbers are then used in TEXTURE1 and TEXTURE2
entries to refer to wall patch lumps. In a texture composition entry,
0 means the first pname, 1 is the second, ...

  The first four bytes of the PNAMES lump is a long integer N.
  Then follow N pnames, each of which occupies 8 bytes. Pnames less than
8 bytes long are padded with zeros. These names duplicate an entry in
the directory (but are not case sensitive - lowercase letters will match
uppercase letters and vice versa). Unlike sprites and floors, wall
patches can be loaded from external pwads. Whichever pwad was listed
last on the command line and contains a lump with the same name as the
one in the PNAMES lump (which itself could be from a pwad) is the pwad
that is used to get the picture data for that wall patch.
```
