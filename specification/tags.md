# Tags specification for Dalet v1.0-preview

## Element

| Property | Description |
| -------- | ----------- |
| id       | 0           |
| name     | el          |
| argument | no          |
| body     | text, tags  |

Most primitive tag. Also used if no tag is specified.

**Daleth example**:

```yaml
el: I am Element
el: { h[1]: I am first level heading }
Element also used if no tag is specified.
```

**Daletl example (json5 representation)**:

```json5
[0, "Dalet"]
```

## Heading

| Property | Description         |
| -------- | ------------------- |
| id       | 1                   |
| name     | h                   |
| argument | int x; 1 <= x <= 10 |
| body     | text                |

Heading is used for text formatting.

**Daleth example**:

```yaml
h[1]: Dalet
  h[2]: Daleth
    h[3]: High level
  h[2]: Daletl
    h[3]: Low level
```

**Daletl example (json5 representation)**:

```json5
[1, "Dalet", 1]
```

## Paragraph

| Property | Description |
| -------- | ----------- |
| id       | 2           |
| name     | p           |
| argument | no          |
| body     | text, tags  |

Paragraph is used for text formatting.

**Daleth example**:

```yaml
p: This is a paragraph
```

**Daletl example (json5 representation)**:

```json5
[2, "This is a paragraph"]
```

## Line break

| Property | Description |
| -------- | ----------- |
| id       | 3           |
| name     | br          |
| argument | no          |
| body     | no          |

Line break is used to insert a line break into the text.

**Daleth example**:

```yaml
br
```

**Daletl example (json5 representation)**:

```json5
[3]
```

## Unordered list

| Property | Description |
| -------- | ----------- |
| id       | 4           |
| name     | ul          |
| argument | no          |
| body     | tags        |

Unordered list is used to create a list. Each list item is a tag.

**Daleth example**:

```yaml
ul: {
    Item 1
    Item 2,
  }
```

**Daletl example (json5 representation)**:

```json5
[
  4,
  [
    [0, "Item 1"],
    [0, "Item 2"],
  ],
]
```
