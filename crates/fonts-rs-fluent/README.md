# fonts-rs-fluent

Fluent UI System Icons integration for GTK 4 and pixel-drawing.

## Overview

[Fluent UI System Icons](https://github.com/microsoft/fluentui-system-icons)
is Microsoft's icon set - over 2,400 icons per style in `regular` and
`filled` variants. This crate integrates Fluent into the `fonts-rs`
framework using the **hybrid approach**:

- **GResource SVGs** - the upstream 24px SVG slice (fills rewritten to
  `currentColor` by the `CurrentColor` post-processor), renamed to
  `fluent-{name}-symbolic` / `fluent-filled-{name}-symbolic` under the
  `scalable/{context}` layout. Rendered via `librsvg` in GTK 4's
  `GtkIconTheme` and recolored by the widget theme.
- **Bundled TTF** - `FluentSystemIcons-{Regular,Filled}.ttf` for Pango
  text (`install_pango`), `@font-face` HTML export, `ab_glyph` software
  rendering, and `embed-fonts` embedding.

Codepoints are **real upstream codepoints** from the vendored
`FluentSystemIcons-{Regular,Filled}.json` name↔codepoint maps - including
~26% glyphs in the Supplementary Private Use Area-A (>U+FFFF).

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- **`metadata`** - Icon keywords (filename-derived) and iconify aliases for search

### Variant features

Each feature selects one variant. If neither is active, `regular` is
the default.

- **`regular`** - `FluentSystemIcons-Regular.ttf` + regular SVG artwork (default)
- **`filled`** - `FluentSystemIcons-Filled.ttf` + filled SVG artwork

## Usage

```toml
[dependencies]
fonts-rs-fluent = "0.5"
# Filled variant with metadata for search:
fonts-rs-fluent = { version = "0.5", features = ["filled", "metadata"] }
```

```rust
use fonts_rs_fluent::Fluent;

fn main() {
    Fluent::install().unwrap();
}
```

Icons resolve as `fluent-{name}-symbolic` (regular) or
`fluent-filled-{name}-symbolic` (filled) in `GtkIconTheme`. The TTF
exposes the same codepoints under the Pango family
`FluentSystemIcons-Regular` / `FluentSystemIcons-Filled`.

## Examples

```sh
cargo run -p fonts-rs-fluent --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-fluent --example svg_gresource --features gtk
cargo run -p fonts-rs-fluent --example gtk_gresource --features gtk
cargo run -p fonts-rs-fluent --example embed_fonts_render --features gtk,render,embed-fonts
cargo run -p fonts-rs-fluent --example css_class_html --features gtk
```

A GTK 4 cheat sheet is available as `fonts-rs-fluent-cheat-sheet`:

```sh
cargo run -p fonts-rs-fluent-cheat-sheet
```

## Font License

Fluent UI System Icons are licensed under the MIT License
(`resources/LICENSE`). The crate code is MIT.
