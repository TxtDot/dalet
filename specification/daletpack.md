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
| arr 2                    | 8   | 1000    |
| arr 4                    | 9   | 1001    |
| arr 8                    | 10  | 1010    |
| arr 16                   | 11  | 1011    |
| arr 32                   | 12  | 1100    |
| tag (id)                 | 13  | 1101    |
| tag (id, body)           | 14  | 1110    |
| tag (id, body, argument) | 15  | 1111    |
