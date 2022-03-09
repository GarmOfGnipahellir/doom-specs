# Terminology conventions

```
Throughout this document, I will use the following conventions for
numbers and variable types:

(1) Most numbers will be decimal. Hexadecimal numbers will usually be
        labeled thus: 0xffff or $ffff. But sometimes I'll say "hex ...".
        And in tablature form, a column heading "HEX" indicates all the
        numbers in that column are hexadecimal.
(2) "byte" is of course the generic, 8 bits. It will usually mean one
        8-bit component of a larger data type, or an 8-bit ASCII
        character, or some such. As a number, it is an unsigned 8-bit
        integer (0..255).
(3) "short" is a signed 16-bit integer (-32768..32767), stored in
        lo-hi format.
(4) "ushort" or "unsigned short" is an unsigned 16-bit integer (0..65535).
(5) "integer" or "long" is a signed 32-bit integer. If you don't read
        this first, my use of the word "integer" might not be immediately
        apparent.
(6) "string8" or "8-byte string" is an ASCII string with length between
        1 and 8 characters inclusive. If its length is less than 8, the
        remaining bytes are zeros.
(7) The first byte of a file or any data structure, for addressing and
        offset purposes, is byte #0, not byte #1.
(8) Some abbreviations I use: E1, E2, and E3 refer to episodes 1, 2, and
        3 respectively. "The EXE" means the file DOOM.EXE.
(666) Any reference to this number is purely intentional.
```
