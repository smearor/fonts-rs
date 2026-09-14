# fonts-rs-noto-emoji-generator

Build-time code generation and glyph export for Noto Emoji.

## Overview

This crate provides the build-time pipeline specific to Noto Emoji, including
CLDR annotation parsing for keywords and categories. It is used as a
build-dependency by `fonts-rs-noto-emoji`.

## Usage

```toml
[build-dependencies]
fonts-rs-noto-emoji-generator = "0.1"
```

## License

MIT
