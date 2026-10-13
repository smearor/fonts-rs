# fonts-rs-blobmoji

Blobmoji font integration for GTK 4 and pixel-drawing.

## Overview

[Blobmoji](https://github.com/DavidBerdik/blobmoji2) is a
community-maintained emoji font recreating Google's classic "blob" emoji
style - the amorphous, expressive characters that shipped with Android
4.4 through 8.0. This crate integrates Blobmoji 17.1 (Unicode 17
coverage) into the `fonts-rs` framework using the **hybrid approach**:

- **GResource SVGs** - pre-rendered full-color source SVGs from
  `blobmoji2/svg/` (~4000 files, vendored via `fonts-rs-blobmoji-glyphs`
  and `fonts-rs-blobmoji-sequences`), renamed to CLDR semantic names
  (`grinning-face`, `thumbs-up`, ...). Rendered via `librsvg` in GTK 4's
  `GtkIconTheme` - full color, original artwork, no COLRv1 extraction.
- **TTF for `pixel-drawing`** - the selected variant's TTF is bundled
  for `ab_glyph` rendering (CBDT bitmap strikes or COLRv1 outlines).

Glyph names come from CLDR annotation data, identical to the Noto Emoji
approach. Multi-codepoint sequences (ZWJ, keycap, flag) are covered by
the pre-rendered SVG set and allocated PUA codepoints.

## Variants

This is an umbrella crate. Exactly one TTF variant is selected by Cargo
feature; the active variant crate is re-exported 1:1.

| Feature | Crate | Font | `RENDERABLE` |
|---------|-------|------|--------------|
| `color-flags` (default) | `fonts-rs-blobmoji-color-flags` | `Noto-COLRv1.ttf` | `false` |
| `color-noflags` | `fonts-rs-blobmoji-color-noflags` | `Noto-COLRv1-noflags.ttf` | `false` |
| `bitmap-flags` | `fonts-rs-blobmoji-bitmap-flags` | `NotoColorEmoji.ttf` | `true` |
| `bitmap-noflags` | `fonts-rs-blobmoji-bitmap-noflags` | `NotoColorEmoji-noflags.ttf` | `true` |

Variant crates can also be used directly for smaller dependency trees.

## Features

- **Variant selection** - exactly one of `color-flags` (default),
  `color-noflags`, `bitmap-flags`, `bitmap-noflags`; enabling several
  is a compile error
- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
  (only `bitmap-*` produces pixels - see `RENDERABLE`)
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Emoji keywords, categories, picker ordering,
  shortcode aliases and tone groups (`emoji_17_0_ordering.json`) via
  `BlobmojiMetadata` (`GlyphMetadata` + `GlyphOrdering`)

The GResource SVGs are always full-color regardless of variant;
`*-noflags` additionally drops flag emoji SVGs.

## Usage

```toml
[dependencies]
# Default: COLRv1 with flag emoji
fonts-rs-blobmoji = "0.5"

# Another variant: disable defaults, then pick exactly one
fonts-rs-blobmoji = { version = "0.5", default-features = false, features = ["gtk", "bitmap-flags"] }

# With metadata for search:
fonts-rs-blobmoji = { version = "0.5", features = ["metadata"] }
```

```rust
use fonts_rs_blobmoji::Blobmoji;

fn main() {
    Blobmoji::install().unwrap();
}
```

## Emoji ordering & metadata

With the **`metadata`** feature, `BlobmojiMetadata` implements
`GlyphMetadata` + `GlyphOrdering` backed by build-time generated phf
tables:

- **Keywords & categories** - CLDR annotations and `emoji-test.txt`;
  `search()`/`keywords()`/`categories()` power filtering beyond name
  matching.
- **Picker ordering** - `groups()`/`order_index()`/`group()` follow
  `googlefonts/emoji-metadata` (`emoji_17_0_ordering.json`): Smileys
  first, canonical group layout.
- **Variants** - `alternates()`/`base_of()` link skin-tone, gender and
  ZWJ sequences to their base emoji; `tone_group()` shares the variant
  shape across bases for default-tone picking.
- **Shortcode aliases** - `:smile:` and emoticons resolve via
  `resolve_alias()` and hit `search()`; `shortcodes(name)` lists them.

The cheat sheet consumes all of this via `CheatSheetFamily::ordering()`
- picker-order grid, group jump-list, collapsed variants, tone picking.

## Examples

Each variant crate ships the standard example set; the renderable
`bitmap-*` crates additionally provide `embed_fonts_render`:

```sh
cargo run -p fonts-rs-blobmoji-color-flags --example gtk_icon_theme
cargo run -p fonts-rs-blobmoji-color-flags --example svg_gresource
cargo run -p fonts-rs-blobmoji-bitmap-flags --example embed_fonts_render --features gtk,render,embed-fonts
```

A GTK 4 cheat sheet is available as `fonts-rs-blobmoji-cheat-sheet`:

```sh
cargo run -p fonts-rs-blobmoji-cheat-sheet
```

## Font License

Blobmoji is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
