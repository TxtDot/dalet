<div align="center">

![dalet](./imgs/dalet.png)

# dalet

</div>

Simple markup language сombining small file size, big number of possibilities for describing the interface and readability.

Specification is coming soon.

## Concept

```yaml
h1: TxtDot revolution
p: TxtDot is a cool project

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
    }

    {
      h3: Production

      code: {
        npm install
        npm run build
        npm run start
      }
    }

    {
      h3: Docker

      code: docker compose up -d
    }

  }
}
```
