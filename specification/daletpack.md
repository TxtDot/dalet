# DaletPack specification for Dalet v1.0-preview

DaletPack is an binary data format for Dalet, that is used for minimizing the size of transmitted daletl data. DaletPack is designed specifically to transfer as little data as possible, it compresses the declaration of tag types into the smallest possible volume. Nothing unnecessary is transferred.

All data must be compressed in [Zstandard](https://datatracker.ietf.org/doc/html/rfc8878) format.

Root data format is array of tags (see [Daletl specification](./daletl.md)), each element reads sequentially without type and size definition.

## Types (16)

- **Null**
- **Integer**
- **String** (6)
- **Array** (5)
- **Tags** (3)
  - **Tag only with id**
  - **Tag with id and body**
  - **Tag with id, body and argument**

## Limitations

- a value of integer (4 bits) must be between 0 and 15
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
| str 8                    | 4   | 0100    |
| str 16                   | 5   | 0101    |
| str 32                   | 6   | 0110    |
| arr 3                    | 7   | 0111    |
| arr 4                    | 8   | 1000    |
| arr 8                    | 9   | 1001    |
| arr 16                   | 10  | 1010    |
| arr 32                   | 11  | 1011    |
| tag (id)                 | 12  | 1100    |
| tag (id, body)           | 13  | 1101    |
| tag (id, argument)       | 14  | 1110    |
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
+--------+------+
|  0001  | XXXX |
+--------+------+
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

str 8 (up to 255 bytes):
+--------+----------+========+
|  0100  | XXXXXXXX |  data  |
+--------+----------+========+

str 16 (up to 2^16-1 bytes):
+--------+----------+----------+========+
|  0101  | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+========+

str 32 (up to 2^32-1 bytes):
+--------+----------+----------+----------+----------+========+
|  0110  | XXXXXXXX | XXXXXXXX | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+----------+----------+========+
```

### Array format

```txt
arr 3 (up to 7 elements):
+--------+-----+~~~~~~~~+
|  0111  | XXX |  data  |
+--------+-----+~~~~~~~~+

arr 4 (up to 15 elements):
+--------+------+~~~~~~~~+
|  1000  | XXXX |  data  |
+--------+------+~~~~~~~~+

arr 8 (up to 255 elements):
+--------+----------+~~~~~~~~+
|  1001  | XXXXXXXX |  data  |
+--------+----------+~~~~~~~~+

arr 16 (up to 2^16-1 elements):
+--------+----------+----------+~~~~~~~~+
|  1010  | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+~~~~~~~~+

arr 32 (up to 2^32-1 elements):
+--------+----------+----------+----------+----------+~~~~~~~~+
|  1011  | XXXXXXXX | XXXXXXXX | XXXXXXXX | XXXXXXXX |  data  |
+--------+----------+----------+----------+----------+~~~~~~~~+
```

### Tag format

```txt

Y = tag_id = XXXXX (5 bits) (can change before release)

tag (id):
+--------+---+
|  1100  | Y |
+--------+---+

tag (id, body):
+--------+---+~~~~~~~~+
|  1101  | Y |  body  |
+--------+---+~~~~~~~~+

tag (id, argument):
+--------+---+~~~~~~~~~~~~+
|  1101  | Y |  argument  |
+--------+---+~~~~~~~~~~~~+

tag (id, body, argument):
+--------+---+~~~~~~~~+~~~~~~~~~~~~+
|  1111  | Y |  data  |  argument  |
+--------+---+~~~~~~~~+~~~~~~~~~~~~+
```
