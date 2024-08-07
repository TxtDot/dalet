<div align="center">

![Dalet](https://github.com/TxtDot/.github/blob/main/imgs/dalet.png?raw=true)

# Dalet

</div>

Markup language ecosystem сombining small file size, big number of possibilities for describing the interface and readability.

[Specification](./specification/main.md).
[Rust implementation](https://github.com/TxtDot/dalet-rs)

> [!WARNING]
> Specification is not complete and very unstable.

## Concept

This is Daleth (high level language that compiles to Daletl).

```yaml
# multilines
#
# (text) - input is trimmed with indent
#
# (~n text) - n is number of minimum spaces to add after trimming with indent
# for each line
#
# (# text) - input not modified
#
# tag syntax
#
# tag: text body
# tag (multiline text body)
# body text always trimmed
#
# tag { multiple tags body }
#
# Arguments
# tag[argument]
#
# Tags without body and argument also supported

meta[title]: Daleth syntax concept
meta[description]: This document describes Daleth syntax and some tags

h[1]: TxtDot revolution
p: TxtDot is a cool project

# If no tag is specified, then the 'el' tag is placed
This is element
br

# if no tag is specified but a '()' is present, then the 'p' tag is placed
# '\n' is deleted only in this format. If a break line is needed in a paragraph, use '  \n'.
(
  Check Dalet too
  This is one paragraph
)

( This is another paragraph )

# [ ] for argument
row[center] {
  link[https://github.com/txtdot/txtdot]: Homepage
  btn[https://example.com/donate] {
    # tag without body
    img[https://example.com/donate.png]
    Donate
  }
}

# {} for multiple objects
row {
  {
    h[2]: Features

    ul {
      Server-side page simplification
      Media proxy
      Image compression with Sharp
      Rendering client-side apps (Vanilla, React, Vue, etc) with webder
      Search with SearXNG
      Handy API endpoints
      No client JavaScript
      Some kind of Material Design 3
      Customization with plugins, see @txtdot/sdk and @txtdot/plugins
    }

  }

  {
    h[2]: Running

    {
      h[3]: Dev

      # () for multiline strings, indent is automatically trimmed
      code (
        npm install
        npm run dev
      )

      # (~n Text) n is number of minimum spaces
      code[markdown] (~4
        this is codeblock
      )

      # (# Text) Text after "(# " not modified
      code[markdown] (#     this is codeblock)
    }

    {
      h[3]: Production

      code (
        npm install
        npm run build
        npm run start
      )
    }

    {
      h[3]: Docker

      code: docker compose up -d
    }

  }
}

# Table has custom format if text used
# +| cells | - primary column
#  | cells | - secondary column
#  | Element | Description | - converts to
#  tcol {
#    Element
#    Description
#  }
table (
  +| Tag      | Description     |
   | h        | Heading         |
   | p        | Paragraph       |
   | img      | Image           |
   | link     | Link            |
   | btn      | Button          |
   | ul       | Unordered list  |
   | br       | Line break      |
  +| quantity | 7               |
)
```
