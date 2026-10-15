# fonts-rs-bravura

[![crates.io](https://img.shields.io/crates/v/fonts-rs-bravura.svg)](https://crates.io/crates/fonts-rs-bravura)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: OFL-1.1](https://img.shields.io/badge/License-OFL--1.1-blue.svg)](https://openfontlicense.org)

Bravura SMuFL music notation font integration for GTK 4 and pixel-drawing.

## Overview

[Bravura](https://github.com/steinbergmedia/bravura) is a SMuFL-compliant
music notation font. This crate integrates Bravura into the `fonts-rs`
framework, using the SMuFL `glyphnames.json` metadata for glyph name
resolution (since Bravura lacks PostScript glyph names).

![Bravura Music Sheet](resources/music-sheet.png)

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`

## Usage

```toml
[dependencies]
fonts-rs-bravura = "0.5"
```

```rust
use fonts_rs_bravura::Bravura;

fn main() {
    Bravura::install().unwrap();
}
```

## Font License

Bravura is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
