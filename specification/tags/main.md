# Tags specification for Dalet v1.0-preview

## Element

| Property | Description |
| -------- | ----------- |
| id       | 0           |
| name     | element     |
| argument | no          |
| body     | any         |

Most primitive tag. Also used if no tag is specified.

Example:

```yaml
element: Dalet
element: { h[1]: Daleth }
Daletl
```

Daletl json5 data representation:

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

Example:

```yaml
h[1]: Dalet
  h[2]: Daleth
    h[3]: High level
  h[2]: Daletl
    h[3]: Low level
```

Daletl json5 representation:

```json5
[1, "Dalet", 1]
```
