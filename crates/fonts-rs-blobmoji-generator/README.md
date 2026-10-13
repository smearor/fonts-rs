# fonts-rs-blobmoji-generator

Build-time code generation and glyph export for Blobmoji.

## Overview

This crate provides the build-time pipeline specific to Blobmoji, including
CLDR annotation parsing for keywords and categories. It is used as a
build-dependency by the `fonts-rs-blobmoji-*` variant crates.

## Usage

```toml
[build-dependencies]
fonts-rs-blobmoji-generator = "0.1"
```

## License

MIT
