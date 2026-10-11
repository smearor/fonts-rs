# fonts-rs-noto-emoji

Noto Emoji font integration for GTK 4 and pixel-drawing.

## Overview

[Noto Emoji](https://fonts.google.com/noto/specimen/Noto+Emoji) is Google's
emoji font. This crate integrates Noto Emoji into the `fonts-rs` framework,
with CLDR-based keyword and category metadata for search functionality.

![Noto Emoji Cheat Sheet](resources/cheat-sheet.png)

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Emoji keywords, categories, picker ordering,
  shortcode aliases and tone groups (`emoji_17_0_ordering.json`) via
  `NotoEmojiMetadata` (`GlyphMetadata` + `GlyphOrdering`)

## Usage

```toml
[dependencies]
fonts-rs-noto-emoji = "0.1"
# With metadata for search:
fonts-rs-noto-emoji = { version = "0.1", features = ["metadata"] }
```

```rust
use fonts_rs_noto_emoji::NotoEmoji;

fn main() {
    NotoEmoji::install().unwrap();
}
```

## Emoji ordering & metadata

With the **`metadata`** feature, `NotoEmojiMetadata` implements
`GlyphMetadata` + `GlyphOrdering` backed by build-time generated phf
tables (same sources as `fonts-rs-blobmoji`):

- **Keywords & categories** - CLDR annotations and `emoji-test.txt`.
- **Picker ordering** - `groups()`/`order_index()`/`group()` follow
  `googlefonts/emoji-metadata`.
- **Variants & tone picking** - `alternates()`/`base_of()` link skin-tone
  and ZWJ sequences to their base; `tone_group()` shares the variant
  shape for default-tone picking.
- **Shortcode aliases** - `:smile:`/emoticons resolve via
  `resolve_alias()` and hit `search()`.

## Font License

Noto Emoji is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
