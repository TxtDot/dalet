# Daletl specification for Dalet v1.0-preview

## Data format

Daletl must be serialized as [DaletPack](./daletpack.md). All data transfer between server and client is done in this format.

### Root

Daletl root is array of tags. For convenience, we will use the json5 representation of the data.

```json5
[]
```

### Tag

All tags specification is in [Tags](./tags.md).

Each tag may be one of four types:

#### Data Representation

##### As array of 1-3 elements

1. Tag id
2. Tag body (optional if argument is null)
3. Tag argument (optional)

Tag id is integer number.

Body can be only a string, null, array of tags or tag (equals to array of tags with 1 tag).

Argument can be number or string.

###### Heading example

```json5
[1, "This is heading", 1]
```

###### Unordered list example

```json5
[
  4,
  [
    [0, "Item 1"],
    [0, "Item 2"],
  ],
]
```

##### As string

String becomes element tag.

```json5
"Element"
```

equals to

```json5
[0, "Element"]
```

##### As array of tags

If array not started with a number. The implication is that this turns into an “element” tag

```json5
["Element", [1, "Heading"]]
```

equals to

```json5
[
  0,
  [
    [0, "Element"],
    [1, "Heading"],
  ],
]
```
