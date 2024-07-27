# DaletPack specification for Dalet v1.0-preview

DaletPack is an binary data format for Dalet, that is used for minimizing the size of transmitted daletl data. DaletPack is designed specifically to transfer as little data as possible, it compresses the declaration of tag types into the smallest possible volume. Nothing unnecessary is transferred.

All data must be compressed in [Zstandard](https://datatracker.ietf.org/doc/html/rfc8878) format.

## Types (12)

- **Null** (1)
- **Integer** (1)
- **String** (6)
- **Array** (3)
- **Tags** (3)
  - **Tag only with id** (1)
  - **Tag with id and body** (1)
  - **Tag with id, body and argument** (1)

## Limitations

- a value of integer (5 bits) must be between -15 and 15
- maximum byte size of a String object is (2^32)-1
- string must be encoded in UTF-8
- maximum number of elements of an Array object is (2^32)-1

## Formats

### Overview

| name                     | id  | id-bits |
| ------------------------ | --- | ------- |
| null                     | 0   | 0000    |
| int                      | 1   | 0001    |
| str 3                    | 2   | 0010    |
| str 4                    | 3   | 0011    |
| str 6                    | 4   | 0100    |
| str 8                    | 5   | 0101    |
| str 16                   | 6   | 0110    |
| str 32                   | 7   | 0111    |
| arr 3                    | 8   | 1000    |
| arr 4                    | 9   | 1001    |
| arr 8                    | 10  | 1010    |
| arr 16                   | 11  | 1011    |
| arr 32                   | 12  | 1100    |
| tag (id)                 | 13  | 1101    |
| tag (id, body)           | 14  | 1110    |
| tag (id, body, argument) | 15  | 1111    |

### Notation in diagrams

```txt
block of bits (max 8 bits):
+--------+
|        |
+--------+

a variable number of bytes:
+========+
|        |
+========+

variable number of objects stored in DaletPack format:
+~~~~~~~~~~~~~~~~~+
|                 |
+~~~~~~~~~~~~~~~~~+
```

### Null format

```txt
+--------+
|  0000  |
+--------+
```

### Integer format

```txt
Positive integer:
+--------+-------+
|  0001  | 0XXXX |
+--------+-------+

Negative integer:
+--------+-------+
|  0001  | 1XXXX |
+--------+-------+
```

### String format

```txt
str 3 (up to 7 bytes):
+--------+-----+========+
|  0010  | XXX |  data  |
+--------+-----+========+

str 4 (up to 15 bytes):
+--------+------+========+
|  0011  | XXXX |  data  |
+--------+------+========+

str 6 (up to 63 bytes):
+--------+--------+========+
|  0100  | XXXXXX |  data  |
+--------+--------+========+

str 8 (up to 255 bytes):
+--------+----------+========+
|  0101  | XXXXXXXX |  data  |
+--------+----------+========+

str 16 (up to 2^16-1 bytes):
+--------+----------+----------+========+
|  0110  | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+========+

str 32 (up to 2^32-1 bytes):
+--------+----------+----------+----------+----------+========+
|  0111  | XXXXXXXX | XXXXXXXX | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+----------+----------+========+
```

### Array format

```txt
arr 3 (up to 7 elements):
+--------+-----+~~~~~~~~+
|  1000  | XXX |  data  |
+--------+-----+~~~~~~~~+

arr 4 (up to 15 elements):
+--------+------+~~~~~~~~+
|  1001  | XXXX |  data  |
+--------+------+~~~~~~~~+

arr 8 (up to 255 elements):
+--------+----------+~~~~~~~~+
|  1010  | XXXXXXXX |  data  |
+--------+----------+~~~~~~~~+

arr 16 (up to 2^16-1 elements):
+--------+----------+----------+~~~~~~~~+
|  1011  | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+~~~~~~~~+

arr 32 (up to 2^32-1 elements):
+--------+----------+----------+----------+----------+~~~~~~~~+
|  1100  | XXXXXXXX | XXXXXXXX | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+----------+----------+~~~~~~~~+
```

### Tag format

```txt

Y = tag_id = XXXXX (5 bits) (can change)

tag (id):
+--------+---+
|  1101  | Y |
+--------+---+

tag (id, body):
+--------+---+~~~~~~~~+
|  1110  | Y |  data  |
+--------+---+~~~~~~~~+

tag (id, body, argument):
+--------+---+~~~~~~~~+~~~~~~~~+
|  1111  | Y |  data  |  data  |
+--------+---+~~~~~~~~+~~~~~~~~+
```
