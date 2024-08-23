# DaletPack specification for Dalet v1.0-preview

DaletPack is an binary data format for Dalet, that is used for minimizing the size of transmitted data. DaletPack is designed specifically to transfer as little data as possible, it compresses the declaration of tag types into the optimal possible volume.

All apps that supports Dalet must use this format when transmitting data between hosts.

All data must be compressed with [zstd](https://datatracker.ietf.org/doc/html/rfc8878).

Page data format is array of tags (see [daletl specification](./daletl.md)), each element reads sequentially. Type definition for page is not needed.

Mime type: `application/dalet-pack`

## Types

Notation: `type_name (...data) [hex_id]`

- Tags with body
  - Tag (id, text) [**a0**]
  - Tag (id, tag) [**a1**]
  - Tag (id, tags) [**a2**]
- Tags with argument
  - Tag (id, text) [**b0**]
  - Tag (id, number) [**b1**]
- Complex tags
  - Tag (id, text, text) [**c0**]
  - Tag (id, tag, text) [**c1**]
  - Tag (id, tags, text) [**c2**]
  - Tag (id, text, number) [**c3**]
  - Tag (id, tag, number) [**c4**]
  - Tag (id, tags, number) [**c5**]
- Custom tags (most used, for better compression, backward compatible)
  - Tag(id) [**d0**]
  - El (text) [**d1**]
  - El (tag) [**d2**]
  - El (tags) [**d3**]
  - P (text) [**d4**]
  - P (tag) [**d5**]
  - P (tags) [**d6**]

  - Br [**d7**]
  - Hr [**d8**]

  - Img (text arg) [**d9**]

  - B (text) [**da**]
  - I (text) [**db**]

  - A (number arg) [**dc**]
  - A (text arg) [**dd**]

  - S (text) [**de**]
  - Sup (text) [**df**]
  - Sub (text) [**e0**]

  - Meta (text, text arg) [**e1**]

## Limitations

- a value of number must be between 0 and 255 and be integer
- maximum byte size of a text is (2^32)
- text must be encoded in UTF-8
- maximum number of elements of a tag array object is (2^32)

## Binary representation

### Special symbols

| name                     | hex_id  |
| ------------------------ | ------- |
| text end                 | 00      |
| tags end                 | 01      |

### Format

#### Notation

Byte - `+----+`
Variable length of bytes - `+====+`
Variable number of data objects - `+~~~~+`

#### Overview


##### Tag data

Look into types.
`type_name (...data) [hex_id]`

becomes

```txt
+--------+~~~~~~~~~+
| hex_id | ...data |
+--------+~~~~~~~~~+
```

##### Number data

```txt
+---------------+
| 8-bit integer |
+---------------+
```

##### Text data

```txt
+=============+----+
| utf-8 bytes | 00 |
+=============+----+
```

##### Tags data

```txt
+~~~~~~+----+
| tags | 01 |
+~~~~~~+----+
```
