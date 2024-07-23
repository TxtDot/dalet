# Dalet v1.0-preview

**Dalet** is a system through which it is possible to combine human readable markup language and small size of transmitted data. The system is divided into two stages and two languages.

## Specification

- [Tags](./tags.md)
- [Daletl](./daletl.md)
- [Daleth](./daleth.md)

## Languages

### Daleth

**Daleth** is a high-level language for humans. It can be translated into daletl, but daletl cannot be translated into daleth[^1]. It is only used for page creation by humans.

### Daletl

**Daletl** is a low-level language for machines. It is used in data transmission, processing and generation. It is specifically optimized to transfer as little data as possible using MessagePack.

## Stages

### Stage 1 (optional)

In the first stage, the daleth language is parsed and converted to daletl. All tags becomes an array of properties `[tag_id, body, argument]`, so that they take up less space in json or messagepack, for example. **This stage is not for data transmission**.

### Stage 2

At this stage, only daletl is used. This stage is used by programs for generation, modification and rendering. The data is transferred between the server and the client only in this stage.

## References

[^1]: In the first versions only. Detranslation is planned for the future for the sake of devtools.
