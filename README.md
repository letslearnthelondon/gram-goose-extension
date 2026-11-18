# Goose language support for Zed / Gram

Zed/Gram extension providing Goose (`.goose`) language support:

- Tree-sitter grammar (sourced from [aardappel/goose](https://github.com/aardappel/goose), `tree-sitter/`)
- Syntax highlighting, brackets, indents, outline
- Language server via `goose lsp`

## Prerequisites

Install the Goose compiler so that `goose` is on your `PATH` (the extension
looks in `PATH`, then falls back to `$HOME/.local/bin/goose`):

- https://github.com/aardappel/goose

Verify with:

```sh
goose lsp --probe   # prints "goose lsp 3.x"
```

## Install (dev / local)

1. Clone this repo.
2. In Gram: command palette -> `gram: install extension from folder` -> pick this directory.
   In Zed: command palette -> `zed: install dev extension` -> pick this directory.
3. Open a `.goose` file; the status bar should read "Goose".

## Build

```sh
cargo build --release --target wasm32-wasip2
```

Pinned to Rust 1.92.0 via `rust-toolchain.toml` so the WASI ABI matches
Gram/Zed 3.x (0.2.6).
