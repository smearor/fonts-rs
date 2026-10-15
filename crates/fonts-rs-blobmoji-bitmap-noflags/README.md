# fonts-rs-blobmoji-bitmap-noflags

[![crates.io](https://img.shields.io/crates/v/fonts-rs-blobmoji-bitmap-noflags.svg)](https://crates.io/crates/fonts-rs-blobmoji-bitmap-noflags)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: OFL-1.1](https://img.shields.io/badge/License-OFL--1.1-blue.svg)](https://openfontlicense.org)

Blobmoji `bitmap-noflags` variant (`NotoColorEmoji-noflags`, CBDT bitmap
strikes, flag emoji filtered out) integration for GTK 4 and
pixel-drawing.

Provides glyph name resolution with CLDR-based semantic names, GResource
registration with full-color pre-rendered SVG artwork, and keyword/
category metadata for search. The bundled CBDT TTF produces colored
`ab_glyph` output for `pixel-drawing` (`RENDERABLE` is `true`).

Part of the Blobmoji variant matrix - see `fonts-rs-blobmoji` for
the feature-selected umbrella crate.

## Quick Start

```rust,ignore
// Register GResource + font (gtk feature)
fonts_rs_blobmoji_bitmap_noflags::Blobmoji::install()?;

// Resolve a codepoint
let cp = fonts_rs_blobmoji_bitmap_noflags::Blobmoji::codepoint("grinning-face");
```

## Features

- `gtk` (default) - GTK 4 icon theme + GResource integration
- `render` - `ab_glyph` font loading (CBDT strikes render in color)
- `embed-fonts` - embed the TTF via `include_bytes!`
- `metadata` - CLDR keywords, categories, shortcodes and picker ordering

## License

The crate code is MIT-licensed; the bundled font is licensed under the
SIL Open Font License 1.1 (`resources/OFL.txt`).
