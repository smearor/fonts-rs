# fonts-rs-redacted

[![crates.io](https://img.shields.io/crates/v/fonts-rs-redacted.svg)](https://crates.io/crates/fonts-rs-redacted)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: OFL-1.1](https://img.shields.io/badge/License-OFL--1.1-blue.svg)](https://openfontlicense.org)

Redacted placeholder text font integration for GTK 4 and pixel-drawing.

## Overview

[Redacted](https://github.com/christiannaths/Redacted-Font) is a font family
designed to look like real text but remain unreadable, useful for wireframes
and mockups. This crate integrates Redacted into the `fonts-rs` framework.

![Redacted Wireframe](resources/newspaper-redacted-wireframe.png)

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`

## Usage

```toml
[dependencies]
fonts-rs-redacted = "0.5"
```

```rust
use fonts_rs_redacted::Redacted;

fn main() {
    Redacted::install().unwrap();
}
```

## Font License

Redacted is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
