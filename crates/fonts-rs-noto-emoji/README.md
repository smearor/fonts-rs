# fonts-rs-noto-emoji

Noto Emoji font integration for GTK 4 and pixel-drawing.

## Overview

[Noto Emoji](https://fonts.google.com/noto/specimen/Noto+Emoji) is Google's
emoji font. This crate integrates Noto Emoji into the `fonts-rs` framework,
with CLDR-based keyword and category metadata for search functionality.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Emoji keywords and categories for search

## Usage

```toml
[dependencies]
fonts-rs-noto-emoji = "0.1"
# With metadata for search:
fonts-rs-noto-emoji = { version = "0.1", features = ["metadata"] }
```

```rust
use fonts_rs_noto_emoji::register_glyphs;

fn main() {
    register_glyphs().unwrap();
}
```

## Font License

Noto Emoji is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
