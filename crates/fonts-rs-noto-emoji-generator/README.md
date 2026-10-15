# fonts-rs-noto-emoji-generator

[![crates.io](https://img.shields.io/crates/v/fonts-rs-noto-emoji-generator.svg)](https://crates.io/crates/fonts-rs-noto-emoji-generator)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Build-time code generation and glyph export for Noto Emoji.

## Overview

This crate provides the build-time pipeline specific to Noto Emoji, including
CLDR annotation parsing for keywords and categories. It is used as a
build-dependency by `fonts-rs-noto-emoji`.

## Usage

```toml
[build-dependencies]
fonts-rs-noto-emoji-generator = "0.5"
```

## License

MIT
