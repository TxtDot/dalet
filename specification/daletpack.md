# DaletPack specification for Dalet v1.0-preview

DaletPack is an binary data format for Dalet, that is used for minimizing the size of transmitted daletl data. DaletPack is designed specifically to transfer as little data as possible, it compresses the declaration of tag types into the smallest possible volume. Nothing unnecessary is transferred.

All data must be compressed with [zstd](https://datatracker.ietf.org/doc/html/rfc8878).

Root data format is array of tags (see [daletl specification](./daletl.md)), each element reads sequentially. Type definition for root is not needed.

Mime type: `application/dalet-pack`

## Types

- **Integer** (2)
- **String** (5)
- **Tag array**
- **Tags** (4)
  - **Tag (id)**
  - **Tag (id, body)**
  - **Tag (id, argument)**
  - **Tag (id, body, argument)**

## Limitations

- a value of integer must be between 0 and 255
- maximum byte size of a String object is (2^32)
- string must be encoded in UTF-8
- maximum number of elements of a tag array object is (2^32)

## Formats

### Overview

| name                     | id  | id-bits |
| ------------------------ | --- | ------- |
| int 4                    | 0   | 0000    |
| int 8                    | 1   | 0001    |
| str 3                    | 2   | 0010    |
| str 4                    | 3   | 0011    |
| str 8                    | 4   | 0100    |
| str 16                   | 5   | 0101    |
| str 32                   | 6   | 0110    |
| tag array                | 7   | 0111    |
| tag array end            | no  | 10      |
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

X - unknown bit
```

### Integer format

```txt
+--------+------+
|  0000  | XXXX |
+--------+------+

+--------+----------+
|  0001  | XXXXXXXX |
+--------+----------+
```

### String format

```txt
str 3 (up to 8 bytes):
+--------+-----+=========+
|  0010  | XXX |  utf-8  |
+--------+-----+=========+

str 4 (up to 16 bytes):
+--------+------+=========+
|  0011  | XXXX |  utf-8  |
+--------+------+=========+

str 8 (up to 256 bytes):
+--------+----------+=========+
|  0100  | XXXXXXXX |  utf-8  |
+--------+----------+=========+

str 16 (up to 2^16 bytes):
+--------+----------+----------+=========+
|  0101  | XXXXXXXX | XXXXXXXX |  utf-8  |
+--------+----------+----------+=========+

str 32 (up to 2^32 bytes):
+--------+----------+----------+----------+----------+=========+
|  0110  | XXXXXXXX | XXXXXXXX | XXXXXXXX | XXXXXXXX |  utf-8  |
+--------+----------+----------+----------+----------+=========+
```

### Tag array format

```txt
tag array:
+--------+~~~~~~~~~~~~+------+
|  0111  |  elements  |  10  |
+--------+~~~~~~~~~~~~+------+
```

### Tag format

```txt

id = XXXXX (5 bits) (can change before release)

tag (id):
+--------+----+
|  1100  | id |
+--------+----+

tag (id, body):
+--------+----+~~~~~~~~+
|  1101  | id |  body  |
+--------+----+~~~~~~~~+

tag (id, argument):
+--------+----+~~~~~~~~~~~~+
|  1110  | id |  argument  |
+--------+----+~~~~~~~~~~~~+

tag (id, body, argument):
+--------+----+~~~~~~~~+~~~~~~~~~~~~+
|  1111  | id |  body  |  argument  |
+--------+----+~~~~~~~~+~~~~~~~~~~~~+
```
