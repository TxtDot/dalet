# Tags specification for Dalet v1.0-preview

## 0. Element

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

## 1. Heading

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

## 2. Paragraph

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

## 3. Line break

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

## 4. Unordered list

| Property | Description |
| -------- | ----------- |
| id       | 4           |
| name     | ul          |
| argument | no          |
| body     | tags        |

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
| id       | 5           |
| name     | ol          |
| argument | no          |
| body     | tags        |

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
| id       | 6                              |
| name     | row                            |
| argument | string (optional); center, end |
| body     | tags                           |

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
| id       | 7           |
| name     | link        |
| argument | string      |
| body     | text, tags  |

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
| id       | 8           |
| name     | navlink     |
| argument | string      |
| body     | text, tags  |

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
| id       | 9           |
| name     | btn         |
| argument | string      |
| body     | text,tags   |

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
| id       | 9           |
| name     | navbtn      |
| argument | string      |
| body     | text,tags   |

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
| id       | 11          |
| name     | img         |
| argument | string      |
| body     | no          |

Displays an image.

**Daleth example**:

```yaml
img[/dalet.png]
```

**Daletl example (json5 representation)**:

```json5
[11, null, "/dalet.png"]
```
