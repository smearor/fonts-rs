# fonts-rs-blobmoji

Blobmoji font integration for GTK 4 and pixel-drawing.

## Overview

[Blobmoji](https://github.com/DavidBerdik/blobmoji2) is a
community-maintained emoji font recreating Google's classic "blob" emoji
style — the amorphous, expressive characters that shipped with Android
4.4 through 8.0. This crate integrates Blobmoji 17.1 (Unicode 17
coverage) into the `fonts-rs` framework using the **hybrid approach**:

- **GResource SVGs** — pre-rendered full-color source SVGs from
  `blobmoji2/svg/` (~4000 files), renamed to CLDR semantic names
  (`grinning-face`, `thumbs-up`, ...). Rendered via `librsvg` in GTK 4's
  `GtkIconTheme` — full color, original artwork, no COLRv1 extraction.
- **TTF for `pixel-drawing`** — the selected variant's TTF is bundled
  for `ab_glyph` rendering (CBDT bitmap strikes or COLRv1 outlines).

Glyph names come from CLDR annotation data, identical to the Noto Emoji
approach. Multi-codepoint sequences (ZWJ, keycap, flag) are covered by
the pre-rendered SVG set and allocated PUA codepoints.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Emoji keywords (CLDR) and categories (emoji-test.txt)
  for search

### Variant features (2×2 matrix: color × flags)

Each feature selects one TTF for `pixel-drawing`. The GResource SVGs are
always full-color regardless of variant; `*-noflags` additionally drops
flag emoji SVGs.

- **`color-flags`** (default) - `Noto-COLRv1.ttf`, all emoji
- **`color-noflags`** - `Noto-COLRv1-noflags.ttf`, no flag emoji
- **`bitmap-flags`** - `NotoColorEmoji.ttf` (CBDT), all emoji — required
  for colored `ab_glyph` output (`embed_fonts_render`)
- **`bitmap-noflags`** - `NotoColorEmoji-noflags.ttf` (CBDT), no flags

## Usage

```toml
[dependencies]
fonts-rs-blobmoji = "0.3"
# With metadata for search:
fonts-rs-blobmoji = { version = "0.3", features = ["metadata"] }
# Bitmap variant for colored pixel-drawing:
fonts-rs-blobmoji = { version = "0.3", features = ["bitmap-flags"] }
```

```rust
use fonts_rs_blobmoji::Blobmoji;

fn main() {
    Blobmoji::install().unwrap();
}
```

## Examples

```sh
cargo run -p fonts-rs-blobmoji --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-blobmoji --example svg_gresource --features gtk
cargo run -p fonts-rs-blobmoji --example embed_fonts_render --features gtk,render,embed-fonts,bitmap-flags
```

A GTK 4 cheat sheet is available as `fonts-rs-blobmoji-cheat-sheet`:

```sh
cargo run -p fonts-rs-blobmoji-cheat-sheet
```

## Font License

Blobmoji is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
