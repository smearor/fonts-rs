# fonts-rs-blobmoji-cheat-sheet

[![crates.io](https://img.shields.io/crates/v/fonts-rs-blobmoji-cheat-sheet.svg)](https://crates.io/crates/fonts-rs-blobmoji-cheat-sheet)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Blobmoji cheat sheet application.

## Overview

Browse all Blobmoji emoji in a searchable GTK 4 grid. Click an emoji to
open a detail sidebar with the enlarged glyph, canonical name,
codepoint, keywords, and categories. With the `metadata` feature
(default), search also covers CLDR tags and categories.

## Features

- **`gtk`** (default) - GTK4 icon-theme rendering of grid cells
- **`metadata`** (default) - CLDR tags + categories for search

## Usage

```sh
cargo run -p fonts-rs-blobmoji-cheat-sheet
```

## License

MIT. Blobmoji artwork is OFL-1.1 - see `fonts-rs-blobmoji`.
