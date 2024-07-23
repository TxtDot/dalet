# Dalet

> [!WARNING]
> Specification is not complete.

**Dalet** is a system through which it is possible to combine human readable markup language and small size of transmitted data. The system is divided into two stages and two languages.

## Languages

### Daleth

**Daleth** is a high-level language for humans. It can be translated into daletl (to be explained later), but daletl cannot be translated into daleth[^1]. It is only used for page creation by humans.

### Daletl

**Daletl** is a low-level language for machines. It is used in data transmission, processing and generation. It is specifically optimized to transfer as little data as possible using json, messagepack or daletpack.

## Stages

### Stage 1 (optional)

In the first stage, the daleth language is parsed and converted to daletl. All tag names are replaced by the smallest possible names, so that they take up less space in json or messagepack, for example.

### Stage 2

At this stage, only daletl is used. This stage is used by programs for generation, modification and rendering. All information is transferred between the server and the client in this stage.

[^1]: In the first versions only. Detranslation is planned for the future.
