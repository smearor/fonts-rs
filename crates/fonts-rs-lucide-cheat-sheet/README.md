# fonts-rs-lucide-cheat-sheet

GTK 4 cheat sheet application for Lucide Icons.

## Overview

Browse all ~1,850 Lucide icons in a searchable grid. Grid cells render
via `GtkIconTheme` (Lucide is a fontless SVG-only set - no TTF, no PUA
font fallback). Click an icon to open a detail sidebar with the
enlarged glyph, canonical icon name, codepoint, keywords, and
categories.

With the `metadata` feature (default), search also covers upstream
tags and categories from `icons.json` - e.g. "font size" finds
`lucide-a-arrow-down-symbolic`, and category chips (the first family
with real icon-set categories) filter by slugs like `arrows`, `devices`,
or `text`.

## Features

- **`gtk`** (default) - GTK4 icon-theme rendering of grid cells
- **`metadata`** (default) - Upstream tags + categories for search and chips

## Usage

```sh
cargo run -p fonts-rs-lucide-cheat-sheet
```

## License

MIT. Lucide Icons artwork is ISC/MIT - see `fonts-rs-lucide`.
