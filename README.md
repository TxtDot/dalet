<div align="center">

![dalet](./imgs/dalet.png)

# dalet

</div>

Markup language ecosystem сombining small file size, big number of possibilities for describing the interface and readability.

[Specification](./specification/main.md).

## Concept

This is daleth (high level language that translates to daletl).

```yaml
# tag: body
# each line is a new tag (if not {}, () is used)
h1: TxtDot revolution
p: TxtDot is a cool project

# If no tag is specified, then the 'element' tag, the most primitive tag, is placed
(
Check dalet too
This is one paragraph
)

This is another paragraph

# [ ] for arguments
row[center]: {
  link[https://github.com/txtdot/txtdot]: Homepage
  button[https://example.com/donate]: {
    # tag without body
    img[https://example.com/donate.png]
    Donate
  }
}

# {} for multiple objects
row: {
  {
    h2: Features

    ul: {
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
    h2: Running

    {
      h3: Dev

      # () for multiline strings, indent is automatically trimmed
      code: (
        npm install
        npm run dev
      )

      # (~n Text) n is number of minimum spaces
      code[markdown]: (~4
        this is codeblock
      )

      # (# Text) Text after "(# " not modified
      code[markdown]: (#     this is codeblock)
    }

    {
      h3: Production

      code: (
        npm install
        npm run build
        npm run start
      )
    }

    {
      h3: Docker

      code: docker compose up -d
    }

  }
}
```
