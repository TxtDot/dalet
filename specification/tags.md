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
3
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

## 15. Horizontal rule

| Property | Description |
| -------- | ----------- |
| name     | hr          |
| id       | 15          |
| body     | no          |
| argument | no          |

Creates a horizontal rule.

**Daleth example**:

```yaml
hr
```

**Daletl example (json5 representation)**:

```json5
15
```

## 16. Bold

| Property | Description |
| -------- | ----------- |
| name     | b           |
| id       | 16          |
| body     | text        |
| argument | no          |

Creates **bold** text.

**Daleth example**:

```yaml
b: I am Bold
```

**Daletl example (json5 representation)**:

```json5
[16, "I am Bold"]
```

## 17. Italic

| Property | Description |
| -------- | ----------- |
| name     | i           |
| id       | 17          |
| body     | text        |
| argument | no          |

Creates _italic_ text.

**Daleth example**:

```yaml
i: I am Italic
```

**Daletl example (json5 representation)**:

```json5
[17, "I am Italic"]
```

## 18. Blockquote

| Property | Description |
| -------- | ----------- |
| name     | bq          |
| id       | 18          |
| body     | text, tags  |
| argument | no          |

Creates a blockquote.

**Daleth example**:

```yaml
bq: I am Blockquote
```

**Daletl example (json5 representation)**:

```json5
[18, "I am Blockquote"]
```

## 19. Footnote Link

| Property | Description    |
| -------- | -------------- |
| name     | footlnk        |
| id       | 19             |
| body     | no             |
| argument | string, number |

Link to footnote.

**Daleth example**:

```yaml
footlnk[1]
```

**Daletl example (json5 representation)**:

```json5
[19, null, 1]
```

## 20. Footnote

| Property | Description    |
| -------- | -------------- |
| name     | footn          |
| id       | 20             |
| body     | text           |
| argument | string, number |

Creates footnote.

**Daleth example**:

```yaml
footn[1]: I am Footnote
```

**Daletl example (json5 representation)**:

```json5
[20, "I am Footnote", 1]
```

## 21. Anchor

| Property | Description    |
| -------- | -------------- |
| name     | a              |
| id       | 21             |
| body     | no             |
| argument | string, number |

Creates anchor. Like `<a href="#argument"></a>` in HTML.

**Daleth example**:

```yaml
a[0]
```

**Daletl example (json5 representation)**:

```json5
[21, null, 0]
```

## 22. Strikethrough

| Property | Description |
| -------- | ----------- |
| name     | s           |
| id       | 22          |
| body     | text        |
| argument | no          |

Creates ~~strikethrough~~ text.

**Daleth example**:

```yaml
s: I am Strikethrough
```

**Daletl example (json5 representation)**:

```json5
[22, "I am Strikethrough"]
```

## 23. Superscript

| Property | Description |
| -------- | ----------- |
| name     | sup         |
| id       | 23          |
| body     | text        |
| argument | no          |

Creates ^superscript^ text.

**Daleth example**:

```yaml
sup: I am Superscript
```

**Daletl example (json5 representation)**:

```json5
[23, "I am Superscript"]
```

## 24. Subscript

| Property | Description |
| -------- | ----------- |
| name     | sub         |
| id       | 24          |
| body     | text        |
| argument | no          |

Creates ~subscript~ text.

**Daleth example**:

```yaml
sub: I am Subscript
```

**Daletl example (json5 representation)**:

```json5
[24, "I am Subscript"]
```

## 25. Disclosure

| Property | Description |
| -------- | ----------- |
| name     | disc        |
| id       | 25          |
| body     | text, tags  |
| argument | string      |

Creates disclosure element.

**Daleth example**:

```yaml
disc[Click to expand]: I am Disclosure
```

**Daletl example (json5 representation)**:

```json5
[25, "I am Disclosure", "Click to expand"]
```
