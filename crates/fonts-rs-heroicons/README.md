# fonts-rs-heroicons

Heroicons integration for GTK 4 - a **fontless** SVG-only icon set.

## Overview

[Heroicons](https://heroicons.com) is a set of ~320 hand-crafted SVG
icons by the makers of Tailwind CSS, shipped in four variants: outline
(24×24 stroke-based), solid (24×24 filled), mini (20×20 filled) and
micro (16×16 filled). This crate integrates Heroicons into the
`fonts-rs` framework using the **SVG-only approach**:

- **GResource SVGs** - pre-rendered source SVGs, renamed to
  `heroicons-{name}-symbolic` names under the `scalable/{context}`
  layout. Rendered via `librsvg` in GTK 4's `GtkIconTheme`;
  `currentColor` plus the `SymbolicClasses` post-processor make every
  icon symbolic-recolorable (stroke details survive GTK's symbolic
  recoloring instead of becoming filled blobs).
- **No TTF** - Heroicons ships no webfont, so there is no `render`
  (`ab_glyph`), `embed-fonts`, or `pango` capability. Codepoints are
  stable hash-of-name PUA allocations (`HashedPua`) purely for the
  name ↔ codepoint API contract; they carry no font semantics.
- **Typed HTML export** - `Heroicons::html()` produces a self-contained
  page of inline `<svg>` elements (`HtmlSvgExport`); `render_font_face()`
  is unavailable since there is no font to embed.

## Variants

The icon variant is selected by Cargo feature; icon names are
variant-independent (`heroicons-{name}-symbolic`) - the variant only
selects which SVG set ships:

| Feature   | Upstream directory | Size  | Style    |
|-----------|--------------------|-------|----------|
| `outline` (default) | `optimized/24/outline` | 24×24 | stroke   |
| `solid`   | `optimized/24/solid` | 24×24 | filled   |
| `mini`    | `optimized/20/solid` | 20×20 | filled   |
| `micro`   | `optimized/16/solid` | 16×16 | filled   |

## Metadata

`metadata` (keywords + aliases) is vendored under
`resources/metadata/`:

- **`keywords.json`** - the search keywords used by heroicons.com
  (extracted from the site's JS bundle; e.g. `sparkles` carries
  `["glitter", "stars"]`, so searching "star" finds it).
- **`aliases.json`** - Heroicons v1 → v2 name mappings derived from the
  iconify `heroicons` set's alias table (e.g. `exclaimation-circle` →
  `exclamation-circle`).

Heroicons ships no categories.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`metadata`** - Keywords and v1→v2 aliases for search
- **`outline` / `solid` / `mini` / `micro`** - icon variant selection

## Usage

```toml
[dependencies]
fonts-rs-heroicons = "0.5"
# Outline variant with keywords/aliases for search:
fonts-rs-heroicons = { version = "0.5", features = ["outline", "metadata"] }
```

```rust
use fonts_rs_heroicons::Heroicons;

fn main() {
    Heroicons::install().unwrap();
}
```

## Examples

```sh
cargo run -p fonts-rs-heroicons --example gtk_icon_theme --features gtk
cargo run -p fonts-rs-heroicons --example svg_gresource --features gtk
cargo run -p fonts-rs-heroicons --example svg_html --features gtk
cargo run -p fonts-rs-heroicons --example composed_svg_text --features gtk
```

A GTK 4 cheat sheet is available as `fonts-rs-heroicons-cheat-sheet`:

```sh
cargo run -p fonts-rs-heroicons-cheat-sheet
```

## Font License

Heroicons are licensed under the MIT License - see
`resources/LICENSE`. The crate code is MIT.
