# Tags specification for Dalet v1.0-preview

## 0. Element

| Property | Description |
| -------- | ----------- |
| name     | el          |
| id       | 0           |
| body     | text, tags  |
| argument | no          |

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

## 1. Heading

| Property | Description         |
| -------- | ------------------- |
| name     | h                   |
| id       | 1                   |
| body     | text                |
| argument | int x; 1 <= x <= 10 |

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

## 2. Paragraph

| Property | Description |
| -------- | ----------- |
| name     | p           |
| id       | 2           |
| body     | text, tags  |
| argument | no          |

Paragraph is used for text formatting.

**Daleth example**:

```yaml
p: This is a paragraph
```

**Daletl example (json5 representation)**:

```json5
[2, "This is a paragraph"]
```

## 3. Line break

| Property | Description |
| -------- | ----------- |
| name     | br          |
| id       | 3           |
| body     | no          |
| argument | no          |

Line break is used to insert a line break into the text.

**Daleth example**:

```yaml
br
```

**Daletl example (json5 representation)**:

```json5
[3]
```

## 4. Unordered list

| Property | Description |
| -------- | ----------- |
| name     | ul          |
| id       | 4           |
| body     | tags        |
| argument | no          |

Unordered list is used to create a list.

**Daleth example**:

```txt
ul: {
  Item 1
  Item 2
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

## 5. Ordered list

| Property | Description |
| -------- | ----------- |
| name     | ol          |
| id       | 5           |
| body     | tags        |
| argument | no          |

Ordered list is used to create a list with increasing numbers.

**Daleth example**:

```txt
ol: {
  Item
  Item
  Item
}
```

**Daletl example (json5 representation)**:

```json5
[
  5,
  [
    [0, "Item"],
    [0, "Item"],
    [0, "Item"],
  ],
]
```

## 6. Row

| Property | Description                    |
| -------- | ------------------------------ |
| name     | row                            |
| id       | 6                              |
| body     | tags                           |
| argument | string (optional); center, end |

Splits the text into rows.

**Daleth example**:

```txt
row: {
  Left
  Right
}

row[center]: {
  Left
  Right
}
```

**Daletl example (json5 representation)**:

```json5
[
  6,
  [
    [0, "Left"],
    [0, "Right"],
  ],
]
```

## 7. Link

| Property | Description |
| -------- | ----------- |
| name     | link        |
| id       | 7           |
| body     | text, tags  |
| argument | string      |

Link to other sites. On click the link opens in new tab.

**Daleth example**:

```yaml
link[https://example.com]: I am Link
```

**Daletl example (json5 representation)**:

```json5
[7, "I am Link", "https://example.com"]
```

## 8. Navlink

| Property | Description |
| -------- | ----------- |
| name     | navlink     |
| id       | 8           |
| body     | text, tags  |
| argument | string      |

Link to the same site. On click the link opens in current tab.

**Daleth example**:

```yaml
navlink[/specification]: I am Navlink
```

**Daletl example (json5 representation)**:

```json5
[8, "I am Navlink", "/specification"]
```

## 9. Button

| Property | Description |
| -------- | ----------- |
| name     | btn         |
| id       | 9           |
| body     | text,tags   |
| argument | string      |

Same as link, but with button style.

**Daleth example**:

```yaml
btn[https://example.com]: I am Button
```

**Daletl example (json5 representation)**:

```json5
[9, "I am Button", "https://example.com"]
```

## 10. NavButton

| Property | Description |
| -------- | ----------- |
| name     | navbtn      |
| id       | 9           |
| body     | text,tags   |
| argument | string      |

Same as navlink, but with button style.

**Daleth example**:

```yaml
navbtn[https://example.com]: I am NavButton
```

**Daletl example (json5 representation)**:

```json5
[10, "I am NavButton", "https://example.com"]
```

## 11. Image

| Property | Description |
| -------- | ----------- |
| name     | img         |
| id       | 11          |
| body     | no          |
| argument | string      |

Displays an image.

**Daleth example**:

```yaml
img[/dalet.png]
```

**Daletl example (json5 representation)**:

```json5
[11, null, "/dalet.png"]
```

## 12. Table

| Property | Description                       |
| -------- | --------------------------------- |
| name     | table                             |
| id       | 12                                |
| body     | (tcol or tpcol)[] or table-string |
| argument | no                                |

Creates a table.

**Daleth example**:

```txt
table: {
  Name | Age
  Elon | 53,
}
```

**Daletl example (json5 representation)**:

```json5
[
  12,
  [
    [
      13,
      [
        [0, "Name"],
        [0, "Age"],
      ],
    ],
    [
      13,
      [
        [0, "Elon"],
        [0, "53"],
      ],
    ],
  ],
]
```

## 13. Table Column

| Property | Description |
| -------- | ----------- |
| name     | tcol        |
| id       | 13          |
| body     | tags        |
| argument | no          |

Creates a table column.

**Daleth example**:

```txt
tcol: {
  Name
  Age
}
```

**Daletl example (json5 representation)**:

```json5
[
  13,
  [
    [0, "Name"],
    [0, "Age"],
  ],
]
```

## 14. Table Primary Column

| Property | Description |
| -------- | ----------- |
| name     | tpcol       |
| id       | 14          |
| body     | tags        |
| argument | no          |

Like table column, but with primary background.

**Daleth example**:

```txt
tpcol: {
  Name
  Age
}
```

**Daletl example (json5 representation)**:

```json5
[
  14,
  [
    [0, "Name"],
    [0, "Age"],
  ],
]
```
