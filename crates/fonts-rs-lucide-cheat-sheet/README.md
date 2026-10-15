# fonts-rs-lucide-cheat-sheet

[![crates.io](https://img.shields.io/crates/v/fonts-rs-lucide-cheat-sheet.svg)](https://crates.io/crates/fonts-rs-lucide-cheat-sheet)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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
