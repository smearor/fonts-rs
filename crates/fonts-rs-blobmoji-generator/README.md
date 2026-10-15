# fonts-rs-blobmoji-generator

[![crates.io](https://img.shields.io/crates/v/fonts-rs-blobmoji-generator.svg)](https://crates.io/crates/fonts-rs-blobmoji-generator)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Build-time code generation and glyph export for Blobmoji.

## Overview

This crate provides the build-time pipeline specific to Blobmoji, including
CLDR annotation parsing for keywords and categories. It is used as a
build-dependency by the `fonts-rs-blobmoji-*` variant crates.

## Usage

```toml
[build-dependencies]
fonts-rs-blobmoji-generator = "0.5"
```

## License

MIT
