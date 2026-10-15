# nerd-fonts-generator

[![crates.io](https://img.shields.io/crates/v/nerd-fonts-generator.svg)](https://crates.io/crates/nerd-fonts-generator)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Build-time code generation and icon export for Nerd Fonts.

## Overview

This crate provides the Nerd Fonts-specific build-time pipeline, including
icon export from the Symbols Nerd Font TTF file, codepoint map generation,
and the `export_icons` CLI binary.

## Features

- Glyph export from `SymbolsNerdFont-Regular.ttf` to SVG files
- `phf::Map` codepoint map generation
- `export_icons` CLI binary for manual export

## Usage

```toml
[build-dependencies]
nerd-fonts-generator = "0.5"
```

### CLI

```bash
cargo run -p nerd-fonts-generator --bin export_icons -- <font.ttf> -o <output_dir>/
```

## License

MIT
