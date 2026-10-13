# fonts-rs-line-awesome

Line Awesome integration for GTK 4 and pixel-drawing.

## Overview

[Line Awesome](https://icons8.com/line-awesome) is Icons8's
Font-Awesome-5-compatible icon set - ~1,550 icons in `regular`,
`solid` and `brands` styles. This crate integrates it into the
`fonts-rs` framework using the **hybrid multi-TTF approach**:

- **GResource SVGs** - the vendored flat `resources/svg/` slice,
  renamed to `la-{name}-symbolic` / `la-{name}-solid-symbolic` under
  the `scalable/{context}` layout. Upstream paths carry no paint
  attribute at all, so the `CurrentColor::inject_missing`
  post-processor injects `fill="currentColor"` on the `<svg>` root.
- **Bundled TTFs** - all three upstream webfonts
  (`la-regular-400.ttf`, `la-solid-900.ttf`, `la-brands-400.ttf`) ship
  via the static `resources/font.gresource.xml` manifest for Pango
  text (`install_pango` registers all faces), `@font-face` HTML
  export, and `ab_glyph` software rendering with a
  `LineAwesomeStyle` variant selector.

Codepoints are **real upstream FA5 PUA codepoints** (U+F000-U+F8FF,
BMP) from the vendored stem-keyed `codepoints.json`. Regular and solid
style pairs share a codepoint per icon name - the `CodemapGenerator`
dedupes the forward map (canonical name wins) while the reverse map
keeps all names.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font files via `include_bytes!`
- **`metadata`** - Filename-derived keywords, style categories
  (`regular`/`solid`/`brands`) and vendored FA4/FA5 aliases via
  `metadata::LineAwesomeMetadata` (also implements `GlyphOrdering` for
  the cheat-sheet style dropdown and regular/solid variant collapsing)

## Usage

```toml
[dependencies]
fonts-rs-line-awesome = "0.5"
# With metadata for search:
fonts-rs-line-awesome = { version = "0.5", features = ["metadata"] }
```

```rust
use fonts_rs_line_awesome::LineAwesome;

fn main() {
    LineAwesome::install().unwrap();
}
```

Icons resolve as `la-{name}-symbolic` (regular/brands) or
`la-{name}-solid-symbolic` (solid) in `GtkIconTheme`. The TTFs expose
the same codepoints under the Pango families `la-regular-400`,
`la-solid-900` (CSS font family) and `la-brands-400`.

## Examples

```sh
cargo run -p fonts-rs-line-awesome --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-line-awesome --example svg_gresource --features gtk
cargo run -p fonts-rs-line-awesome --example svg_html --features gtk
cargo run -p fonts-rs-line-awesome --example composed_svg_text --features gtk
cargo run -p fonts-rs-line-awesome --example gtk_gresource --features gtk
cargo run -p fonts-rs-line-awesome --example system_font_fallback --features gtk
cargo run -p fonts-rs-line-awesome --example embed_fonts_render --features gtk,render,embed-fonts
cargo run -p fonts-rs-line-awesome --example css_class_html --features gtk
```

A GTK 4 cheat sheet is available as `fonts-rs-line-awesome-cheat-sheet`:

```sh
cargo run -p fonts-rs-line-awesome-cheat-sheet
```

## Font License

Line Awesome is dual-licensed MIT OR "Good Boy License"; vendored here
under MIT (`resources/LICENSE`). The crate code is MIT.
