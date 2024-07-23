# Tags specification for Dalet v1.0-preview

## Element

| Property    | Description |
| ----------- | ----------- |
| Daleth name | element     |
| Daletl name | e           |
| Argument    | no          |
| Body        | any         |

Most primitive tag. Also used if no tag is specified.

Example:

```yaml
element: Dalet
element: { h[1]: Daleth }
Daletl
```

Daletl json5 data representation:

```json5
{ e: "Dalet" }
```

## Heading

| Property    | Description         |
| ----------- | ------------------- |
| Daleth name | h                   |
| Daletl name | h                   |
| Argument    | int x; 1 <= x <= 10 |
| Body        | text                |

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
{ h: "Dalet", l: 1 }
```
