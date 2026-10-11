# fonts-rs-fluent-cheat-sheet

GTK 4 cheat sheet application for Fluent UI System Icons.

## Overview

Browse all 2,474 regular (2,508 filled) Fluent icons in a searchable
grid. Grid cells render via `GtkIconTheme` - the bundled SVGs are
authoritative - while the bundled `FluentSystemIcons-{Regular,Filled}.ttf`
webfont backs `fluent-icon` text labels via Pango, so PUA codepoints
never fall back to a random system font. Click an icon to open a
detail sidebar with the enlarged glyph, canonical icon name,
codepoint, keywords, and alias info.

With the `metadata` feature (default), search also covers
filename-derived keywords (e.g. "square down" finds
`fluent-arrow-square-down-symbolic`) and the iconify alias table
(e.g. "magic-wand" finds `fluent-wand-symbolic`). Deprecated upstream
icons (`flag-pride`, `text-color-accent`) are excluded from the set.

## Features

- **`gtk`** (default) - GTK4 icon-theme rendering of grid cells
- **`metadata`** (default) - filename keywords + iconify aliases for search

## Usage

```sh
cargo run -p fonts-rs-fluent-cheat-sheet
```

## License

MIT. Fluent UI System Icons artwork is MIT - see `fonts-rs-fluent`.
