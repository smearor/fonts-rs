# fonts-rs-cuernavaca-cheat-sheet

[![crates.io](https://img.shields.io/crates/v/fonts-rs-cuernavaca-cheat-sheet.svg)](https://crates.io/crates/fonts-rs-cuernavaca-cheat-sheet)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Chess Cuernavaca cheat sheet application.

## Overview

A GTK4 application that displays all Chess Cuernavaca glyphs with
their names and codepoints, serving as a reference and demo for the
`fonts-rs-cuernavaca` crate.

## Features

- **`gtk`** (default) - GTK4 application
- **`metadata`** (default) - Piece categories
  (king/queen/rook/bishop/knight/pawn) for the category sidebar and
  search, plus style variants (light/dark, normal/upside-down) with
  collapse + variant links

## Usage

```bash
cargo run -p fonts-rs-cuernavaca-cheat-sheet
```

## License

MIT
