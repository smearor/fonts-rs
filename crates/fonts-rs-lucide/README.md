# fonts-rs-lucide

Lucide Icons integration for GTK 4 - a **fontless** SVG-only icon set.

## Overview

[Lucide](https://lucide.dev) is an open-source icon library with ~1,850
icons on a 24×24 grid (`stroke="currentColor"`, `fill="none"`, 2px
stroke - a community-driven continuation of Feather). This crate
integrates Lucide into the `fonts-rs` framework using the
**SVG-only approach**:

- **GResource SVGs** - pre-rendered source SVGs, renamed to
  `lucide-{name}-symbolic` names under the `scalable/{context}` layout.
  Rendered via `librsvg` in GTK 4's `GtkIconTheme`; `currentColor`
  makes every icon symbolic-recolorable.
- **No TTF** - Lucide ships no webfont, so there is no `render`
  (`ab_glyph`), `embed-fonts`, or `pango` capability. Codepoints are
  stable hash-of-name PUA allocations (`HashedPua`) purely for the
  name ↔ codepoint API contract; they carry no font semantics.
- **Typed HTML export** - `Lucide::html()` produces a self-contained
  page of inline `<svg>` elements (`HtmlSvgExport`); `render_font_face()`
  is unavailable since there is no font to embed.

Metadata comes from upstream per-icon JSON files (`icons/{name}.json`
with `tags` + `categories`), vendored merged as
`resources/metadata/icons.json`. Category titles live in
`resources/metadata/categories.json` - Lucide is the first family with
real icon-set categories (42 upstream slugs like `arrows`, `devices`,
`text`). Lucide ships no aliases.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`metadata`** - Icon keywords (tags) and categories for search
  (no aliases exist upstream)

## Usage

```toml
[dependencies]
fonts-rs-lucide = "0.3"
# With upstream tags/categories for search:
fonts-rs-lucide = { version = "0.3", features = ["metadata"] }
```

```rust
use fonts_rs_lucide::Lucide;

fn main() {
    Lucide::install().unwrap();
}
```

## Examples

```sh
cargo run -p fonts-rs-lucide --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-lucide --example svg_gresource --features gtk
cargo run -p fonts-rs-lucide --example svg_html --features gtk
cargo run -p fonts-rs-lucide --example composed_svg_text --features gtk
```

A GTK 4 cheat sheet is available as `fonts-rs-lucide-cheat-sheet`:

```sh
cargo run -p fonts-rs-lucide-cheat-sheet
```

## Font License

Lucide Icons are licensed under the ISC License; icons derived from
Feather remain under MIT - see `resources/LICENSE`. The crate code
is MIT.
