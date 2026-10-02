# fonts-rs-line-awesome-cheat-sheet

GTK 4 cheat sheet application for Line Awesome.

## Overview

Browse all 1,544 Line Awesome icons (1,393 unique codepoints) in a
searchable grid, grouped by the Font Awesome 5 style categories
`regular`/`solid`/`brands`. Grid cells render via `GtkIconTheme` — the
bundled SVGs are authoritative — while the three bundled
`la-{regular-400,solid-900,brands-400}.ttf` webfonts back `la-icon`
text labels via Pango, so PUA codepoints never fall back to a random
system font. Click an icon to open a detail sidebar with the enlarged
glyph, canonical icon name, codepoint, keywords, and alias info.

With the `metadata` feature (default), search also covers
filename-derived keywords (e.g. "address book" finds
`la-address-book-symbolic`), the style categories, and the vendored
Font Awesome aliases (e.g. "address-book-o" or "fa-address-book" find
`la-address-book-symbolic`).

## Features

- **`gtk`** (default) — GTK4 icon-theme rendering of grid cells
- **`metadata`** (default) — filename keywords + style categories +
  Font Awesome aliases for search

## Usage

```sh
cargo run -p fonts-rs-line-awesome-cheat-sheet
```

## License

MIT for this crate. Line Awesome is dual-licensed MIT OR "Good Boy
License" — see `fonts-rs-line-awesome`.
