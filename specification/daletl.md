# Daletl specification for Dalet v1.0-preview

## Data format

Daletl must be serialized as [MessagePack](https://github.com/msgpack/msgpack/blob/master/spec.md). All data transfer between server and client is done in this format.

## Data representation

### Root

Daletl root is array of tags. For convenience, we will use the json5 representation of the data.

```json5
[]
```

### Tag

All tags specification is in [Tags](./tags.md).

Each tag is array of 1-3 elements.

1. Tag id
2. Tag body (optional)
3. Tag argument (optional)

Tag id is integer number.

Body can be only a string, null, or an array of tags.

Argument can be number or string.

#### Heading example

```json5
[1, "This is heading", 1]
```

#### Unordered list example

```json5
[
  4,
  [
    [0, "Item 1"],
    [0, "Item 2"],
  ],
]
```
