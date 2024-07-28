# Daletl specification for Dalet v1.0-preview

## Daletl

Daletl is data representation for Dalet interfaces. [DaletPack](./daletpack.md) serializes/deserializes into Daletl.

Daletl must be serialized as [DaletPack](./daletpack.md). All data transfer between server and client is done in this format.

### Root

Daletl root is array of tags. For convenience, we will use the typescript notation.

```typescript
type Root = Tag[];
```

### Tag

All tags specification is in [Tags](./tags.md).

```typescript
interface Tag {
  id: number;
  body: string | Tag[] | null;
  argument: string | number | null;
}
```

### Example

```typescript
const root: Root = [
  {
    id: 1,
    body: "I am Heading with level 1",
    argument: 1,
  },
];
```
