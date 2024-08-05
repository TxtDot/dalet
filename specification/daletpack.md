# DaletPack specification for Dalet v1.0-preview

DaletPack is an binary data format for Dalet, that is used for minimizing the size of transmitted data. DaletPack is designed specifically to transfer as little data as possible, it compresses the declaration of tag types into the optimal possible volume.

All apps that supports Dalet must use this format when transmitting data between hosts.

All data must be compressed with [zstd](https://datatracker.ietf.org/doc/html/rfc8878).

Page data format is array of tags (see [daletl specification](./daletl.md)), each element reads sequentially. Type definition for page is not needed.

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
| str end                  | 0   |
| str                      | 1   |
| int                      | 2   |
| tag array                | 3   |
| tag array end            | 4   |
| tag (id)                 | 5   |
| tag (id, body)           | 6   |
| tag (id, argument)       | 7   |
| tag (id, body, argument) | 8   |

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

### Str format

```txt

str:
+--------+=========+--------+
|     1  |  utf-8  | 0      |
+--------+=========+--------+
```

### Int format

```txt
+--------+----------+
|     1  | XXXXXXXX |
+--------+----------+
```

### Tag array format

```txt
tag array:
+--------+~~~~~~~~~~~~+------+
|     3  |  elements  |   4  |
+--------+~~~~~~~~~~~~+------+
```

### Tag format

```txt

id = XXXXX (5 bits) (can change before release)

tag (id):
+--------+----+
|     5  | id |
+--------+----+

tag (id, body):
+--------+----+~~~~~~~~+
|     6  | id |  body  |
+--------+----+~~~~~~~~+

tag (id, argument):
+--------+----+~~~~~~~~~~~~+
|     7  | id |  argument  |
+--------+----+~~~~~~~~~~~~+

tag (id, body, argument):
+--------+----+~~~~~~~~+~~~~~~~~~~~~+
|     8  | id |  body  |  argument  |
+--------+----+~~~~~~~~+~~~~~~~~~~~~+
```
