# Daletl specification for Dalet v1.0-preview

## Daletl

Daletl is data representation of serialized/deserialized [DaletPack](./daletpack.md).

### Page

Daletl page is struct. For convenience, we will use the typescript notation.

```typescript
interface Page = {
  data: Tag[]
}
```

### Tag

All tags specification is in [Tags](./tags.md).

```typescript
export type Body = string | Tag[] | null;
export type Argument = string | number | null;

export interface Tag {
  id: number;
  body: Body;
  argument: Argument;
}
```

### Example

```typescript
const page: Page = [
  {
    id: 1,
    body: "I am Heading with level 1",
    argument: 1,
  },
];
```
