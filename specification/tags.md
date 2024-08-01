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

## 6. Row

| Property | Description                  |
| -------- | ---------------------------- |
| name     | row                          |
| id       | 6                            |
| body     | tags                         |
| argument | optional; start, center, end |

Splits the text into rows. The alignment argument specifies how the items inside the row are aligned. By default it is aligned to the start.
m
**Daleth example**:

```txt
row: {
  Left
  Right
}

row[start]: {
  Left
  Right
}
```

Argument converts to numbers in daletl.

start -> 0
end -> 1

## 7. Link

| Property | Description |
| -------- | ----------- |
| name     | link        |
| id       | 7           |
| body     | any         |
| argument | text        |

Link to other sites. On click the link opens in new tab.

**Daleth example**:

```yaml
link[https://example.com]: I am Link
```

## 8. Navlink

| Property | Description |
| -------- | ----------- |
| name     | navlink     |
| id       | 8           |
| body     | any         |
| argument | text        |

Link to the same site. On click the link opens in current tab.

**Daleth example**:

```yaml
navlink[/specification]: I am Navlink
```

## 9. Button

| Property | Description |
| -------- | ----------- |
| name     | btn         |
| id       | 9           |
| body     | any         |
| argument | text        |

Same as link, but with button style.

**Daleth example**:

```yaml
btn[https://example.com]: I am Button
```

## 10. NavButton

| Property | Description |
| -------- | ----------- |
| name     | navbtn      |
| id       | 9           |
| body     | any         |
| argument | text        |

Same as navlink, but with button style.

**Daleth example**:

```yaml
navbtn[https://example.com]: I am NavButton
```

## 11. Image

| Property | Description |
| -------- | ----------- |
| name     | img         |
| id       | 11          |
| body     | no          |
| argument | text        |

Displays an image.

**Daleth example**:

```yaml
img[/dalet.png]
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

## 19. Footnote Link

| Property | Description    |
| -------- | -------------- |
| name     | footlnk        |
| id       | 19             |
| body     | no             |
| argument | text  , number |

Link to footnote.

**Daleth example**:

```yaml
footlnk[1]
```

## 20. Footnote

| Property | Description    |
| -------- | -------------- |
| name     | footn          |
| id       | 20             |
| body     | text           |
| argument | text  , number |

Creates footnote.

**Daleth example**:

```yaml
footn[1]: I am Footnote
```

## 21. Anchor

| Property | Description    |
| -------- | -------------- |
| name     | a              |
| id       | 21             |
| body     | no             |
| argument | text  , number |

Creates anchor. Like `<a href="#argument"></a>` in HTML.

**Daleth example**:

```yaml
a[0]
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

## 25. Disclosure

| Property | Description |
| -------- | ----------- |
| name     | disc        |
| id       | 25          |
| body     | text, tags  |
| argument | text        |

Creates disclosure element.

**Daleth example**:

```yaml
disc[Click to expand]: I am Disclosure
```

## 26. Block

| Property | Description                   |
| -------- | ----------------------------- |
| name     | bl                            |
| id       | 26                            |
| body     | text, tags                    |
| argument | optional; start, center, end  |

Creates block element. Can be used for aligning. By default aligned to center.

**Daleth example**:

```yaml
bl: I am Block
```

Argument converts to numbers in daletl.

start -> 0
end -> 1

## 27. Carousel

| Property | Description |
| -------- | ----------- |
| name     | carousel    |
| id       | 27          |
| body     | tags        |
| argument | no          |

Creates carousel.

**Daleth example**:

```txt
carousel: {
  Example 1
  Example 2,
}
```
