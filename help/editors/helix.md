1. Install daleth_lsp `cargo install daleth_lsp`
2. Add this to languages.toml:

```toml
[[language]]
name = "daleth"
language-servers = ["daleth-lsp"]
auto-format = true
scope = "source.daleth"
file-types = ["dlth"]

[language-server.daleth-lsp]
command = "daleth_lsp"

[[grammar]]
name = "daleth"

[grammar.source]
git = "https://github.com/TxtDot/tree-sitter-daleth"
rev = "HEAD"
```
