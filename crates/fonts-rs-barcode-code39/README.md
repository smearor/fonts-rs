# fonts-rs-barcode-code39

[![crates.io](https://img.shields.io/crates/v/fonts-rs-barcode-code39.svg)](https://crates.io/crates/fonts-rs-barcode-code39)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: OFL-1.1](https://img.shields.io/badge/License-OFL--1.1-blue.svg)](https://openfontlicense.org)

Libre Barcode Code 39 font integration for GTK 4 and pixel-drawing.

## Overview

[Libre Barcode 39](https://github.com/graphicore/librebarcode) is a barcode
font for the Code 39 symbology. This crate integrates it into the
`fonts-rs` framework.

![Barcode Code 39 Demo](resources/barcode-code39.png)

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`

## Usage

```toml
[dependencies]
fonts-rs-barcode-code39 = "0.5"
```

```rust
use fonts_rs_barcode_code39::Code39;

fn main() {
    Code39::install().unwrap();
}
```

## Font License

Libre Barcode is licensed under the SIL Open Font License (OFL-1.1). The
crate code is MIT.
