# fonts-rs-eva

[![crates.io](https://img.shields.io/crates/v/fonts-rs-eva.svg)](https://crates.io/crates/fonts-rs-eva)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Eva Icons font integration for GTK 4 and pixel-drawing.

## Overview

[Eva Icons](https://akveo.github.io/eva-icons/) is an open-source pack
of ~490 delightful icons, available in `fill` (default) and `outline`
styles. This crate integrates Eva Icons into the `fonts-rs` framework
using the **hybrid approach**:

- **GResource SVGs** - pre-rendered source SVGs, renamed to
  `eva-{name}-symbolic` names under the `scalable/{context}` layout.
  Rendered via `librsvg` in GTK 4's `GtkIconTheme`.
- **TTF for `pixel-drawing`** - the bundled `Eva-Icons.ttf` webfont for
  `ab_glyph` rendering (monochrome silhouettes).

Both variants share a single TTF where fill and outline glyphs occupy
different codepoints; the `fill`/`outline` feature selects the codepoint
set and `resources/svg/{variant}` artwork. Glyph names stay
variant-independent (`eva-{name}-symbolic`) - the upstream `-outline`
suffix is stripped by the name resolver. Codepoints are parsed from the
upstream stylesheet `eva-icons.css`; search tags come from
`icons-tags.json` (the keyword data powering the official eva-icons.com
cheat sheet).

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Icon keywords for search (no categories or aliases
  exist upstream)

### Variant features

Each feature selects one variant. If neither is active, `fill` is
the default.

- **`fill`** - filled icon set + codepoints (default)
- **`outline`** - outline icon set + codepoints

## Usage

```toml
[dependencies]
fonts-rs-eva = "0.5"
# Outline variant with metadata for search:
fonts-rs-eva = { version = "0.5", features = ["outline", "metadata"] }
```

```rust
use fonts_rs_eva::Eva;

fn main() {
    Eva::install().unwrap();
}
```

## Examples

```sh
cargo run -p fonts-rs-eva --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-eva --example svg_gresource --features gtk
cargo run -p fonts-rs-eva --example embed_fonts_render --features gtk,render,embed-fonts
```

A GTK 4 cheat sheet is available as `fonts-rs-eva-cheat-sheet`:

```sh
cargo run -p fonts-rs-eva-cheat-sheet
```

## Font License

Eva Icons are licensed under the MIT License. The crate code is MIT.
