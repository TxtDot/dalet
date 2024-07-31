# Dalet v1.0-preview

**Dalet** is a system through which it is possible to combine human readable markup language and small size of transmitted data. The system is divided into two stages and two languages.

## Specifications

- [Tags](./tags.md)
- [Daletl](./daletl.md)
- [Daleth](./daleth.md)
- [DaletPack](./daletpack.md)

## Stages

### Stage 1 (optional)

In the first stage, the daleth language is parsed and converted to daletl.  **This stage is not for data transmission**.

### Stage 2

At this stage, only daletl is used. This stage is used by programs for generation, modification and rendering. The data is transferred between the server and the client only in this stage.

## References

[^1]: In the first versions only. Decompilation is planned for the future for the sake of devtools.
