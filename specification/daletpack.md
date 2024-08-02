# DaletPack specification for Dalet v1.0-preview

DaletPack is an binary data format for Dalet, that is used for minimizing the size of transmitted data. DaletPack is designed specifically to transfer as little data as possible, it compresses the declaration of tag types into the optimal possible volume.

All apps that supports Dalet must use this format when transmitting data between hosts.

All data must be compressed with [zstd](https://datatracker.ietf.org/doc/html/rfc8878).

Root data format is array of tags (see [daletl specification](./daletl.md)), each element reads sequentially. Type definition for root is not needed.

Mime type: `application/dalet-pack`

## Types

- **Integer**
- **String** (3)
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

| name                     | id  |
| ------------------------ | --- |
| int 8                    | 1   |
| str 8                    | 4   |
| str 16                   | 5   |
| str 32                   | 6   |
| tag array                | 7   |
| tag array end            | 8   |
| tag (id)                 | 12  |
| tag (id, body)           | 13  |
| tag (id, argument)       | 14  |
| tag (id, body, argument) | 15  |

### Notation in diagrams

```txt
byte:
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
+--------+----------+
|     1  | XXXXXXXX |
+--------+----------+
```

### String format

```txt

str 8 (up to 256 bytes):
+--------+----------+=========+
|     4  | XXXXXXXX |  utf-8  |
+--------+----------+=========+

str 16 (up to 2^16 bytes):
+--------+----------+----------+=========+
|     5  | XXXXXXXX | XXXXXXXX |  utf-8  |
+--------+----------+----------+=========+

str 32 (up to 2^32 bytes):
+--------+----------+----------+----------+----------+=========+
|     6  | XXXXXXXX | XXXXXXXX | XXXXXXXX | XXXXXXXX |  utf-8  |
+--------+----------+----------+----------+----------+=========+
```

### Tag array format

```txt
tag array:
+--------+~~~~~~~~~~~~+------+
|     7  |  elements  |   8  |
+--------+~~~~~~~~~~~~+------+
```

### Tag format

```txt

id = XXXXX (5 bits) (can change before release)

tag (id):
+--------+----+
|    12  | id |
+--------+----+

tag (id, body):
+--------+----+~~~~~~~~+
|    13  | id |  body  |
+--------+----+~~~~~~~~+

tag (id, argument):
+--------+----+~~~~~~~~~~~~+
|    14  | id |  argument  |
+--------+----+~~~~~~~~~~~~+

tag (id, body, argument):
+--------+----+~~~~~~~~+~~~~~~~~~~~~+
|    15  | id |  body  |  argument  |
+--------+----+~~~~~~~~+~~~~~~~~~~~~+
```
