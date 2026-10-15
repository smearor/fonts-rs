# fonts-rs-tabler

[![crates.io](https://img.shields.io/crates/v/fonts-rs-tabler.svg)](https://crates.io/crates/fonts-rs-tabler)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Tabler Icons font integration for GTK 4 and pixel-drawing.

## Overview

[Tabler Icons](https://tabler.io/icons) is a free and open-source icon
set designed with attention to pixel-perfect detail - over 5000 icons in
`outline` (stroke-based) and `filled` styles. This crate integrates
Tabler Icons into the `fonts-rs` framework using the **hybrid approach**:

- **GResource SVGs** - pre-rendered source SVGs (strokes in
  `currentColor`), renamed to `tabler-{name}-symbolic` names under the
  `scalable/{context}` layout. Rendered via `librsvg` in GTK 4's
  `GtkIconTheme`.
- **TTF for `pixel-drawing`** - the selected variant's webfont TTF is
  bundled for `ab_glyph` rendering (monochrome silhouettes).

Glyph names are variant-independent (`tabler-{name}-symbolic`); the
`outline`/`filled` feature only selects which SVG set and TTF ship.
Codepoints and tags come from upstream `icons.json`; aliases from
`aliases.json`.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Icon keywords, categories, and aliases for search

### Variant features

Each feature selects one variant. If neither is active, `outline` is
the default.

- **`outline`** - `tabler-icons.ttf` + `svg/outline/` artwork (default)
- **`filled`** - `tabler-icons-filled.ttf` + `svg/filled/` artwork

## Usage

```toml
[dependencies]
fonts-rs-tabler = "0.5"
# Filled variant with metadata for search:
fonts-rs-tabler = { version = "0.5", features = ["filled", "metadata"] }
```

```rust
use fonts_rs_tabler::Tabler;

fn main() {
    Tabler::install().unwrap();
}
```

## Examples

```sh
cargo run -p fonts-rs-tabler --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-tabler --example svg_gresource --features gtk
cargo run -p fonts-rs-tabler --example embed_fonts_render --features gtk,render,embed-fonts
```

A GTK 4 cheat sheet is available as `fonts-rs-tabler-cheat-sheet`:

```sh
cargo run -p fonts-rs-tabler-cheat-sheet
```

## Font License

Tabler Icons are licensed under the MIT License. The crate code is MIT.
