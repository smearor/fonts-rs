# fonts-rs-heroicons-cheat-sheet

[![crates.io](https://img.shields.io/crates/v/fonts-rs-heroicons-cheat-sheet.svg)](https://crates.io/crates/fonts-rs-heroicons-cheat-sheet)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Heroicons cheat sheet application.

## Overview

Browse all Heroicons in a searchable GTK 4 grid. Click an icon to open
a detail sidebar with the enlarged glyph, canonical icon name,
codepoint, keywords, and categories. With the `metadata` feature
(default), search also covers upstream tags and categories.

## Features

- **`gtk`** (default) - GTK4 icon-theme rendering of grid cells
- **`metadata`** (default) - Upstream tags + categories for search

## Usage

```sh
cargo run -p fonts-rs-heroicons-cheat-sheet
```

## License

MIT. Heroicons artwork is MIT - see `fonts-rs-heroicons`.
