# fonts-rs

[![crates.io](https://img.shields.io/crates/v/fonts-rs.svg)](https://crates.io/crates/fonts-rs)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![CI Build](https://github.com/smearor/fonts-rs/actions/workflows/build.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/build.yml)
[![MSRV](https://github.com/smearor/fonts-rs/actions/workflows/msrv.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/msrv.yml)
[![Security Audit](https://github.com/smearor/fonts-rs/actions/workflows/audit.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/audit.yml)
[![Book](https://github.com/smearor/fonts-rs/actions/workflows/book.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/book.yml)

A Rust workspace for integrating font families into GTK4 projects. It
provides a modular framework for exporting glyphs as SVG icons, generating
GResource bundles, resolving icon names to Unicode codepoints, and rendering
text with software rasterization.

## Architecture

The workspace is organized into three layers:

1. **Generic framework** - `fonts-rs-model` (shared types) and
   `fonts-rs-generator` (build-time pipeline)
2. **Font family crates** - one crate per font family
3. **Nerd Fonts integration** - `nerd-fonts-model`, `nerd-fonts-generator`,
   and `nerd-fonts-rs`

## Available Crates

### Framework

| Crate | Description |
|-------|-------------|
| [`fonts-rs-model`](./crates/fonts-rs-model) | Generic model types (`CodePoint`, `FontFamily`, `GlyphName`, `GlyphEntry`) |
| [`fonts-rs-generator`](./crates/fonts-rs-generator) | Build-time pipeline (`ExportConfig`, `FontBuild`, `impl_font_loader!`) |

### Font Families

| Crate | Font | License | Variants |
|-------|------|---------|----------|
| [`fonts-rs-doto`](./crates/fonts-rs-doto) | Doto | OFL-1.1 | Variable (wght, ROND) |
| [`fonts-rs-seven-segment`](./crates/fonts-rs-seven-segment) | DSEG7 | OFL-1.1 | 24 variants |
| [`fonts-rs-fourteen-segment`](./crates/fonts-rs-fourteen-segment) | DSEG14 | OFL-1.1 | 24 variants |
| [`fonts-rs-barcode-code39`](./crates/fonts-rs-barcode-code39) | Libre Barcode 39 | OFL-1.1 | - |
| [`fonts-rs-barcode-code128`](./crates/fonts-rs-barcode-code128) | Libre Barcode 128 | OFL-1.1 | - |
| [`fonts-rs-barcode-ean13`](./crates/fonts-rs-barcode-ean13) | Libre Barcode EAN13 | OFL-1.1 | - |
| [`fonts-rs-bravura`](./crates/fonts-rs-bravura) | Bravura (SMuFL) | OFL-1.1 | - |
| [`fonts-rs-redacted`](./crates/fonts-rs-redacted) | Redacted | OFL-1.1 | - |
| [`fonts-rs-dicefont`](./crates/fonts-rs-dicefont) | DiceFont | OFL-1.1 | - |
| [`fonts-rs-cuernavaca`](./crates/fonts-rs-cuernavaca) | Cuernavaca (chess) | OFL-1.1 | - |
| [`fonts-rs-noto-emoji`](./crates/fonts-rs-noto-emoji) | Noto Emoji | OFL-1.1 | - |

### Nerd Fonts

| Crate | Description |
|-------|-------------|
| [`nerd-fonts-model`](./crates/nerd-fonts-model) | Shared Nerd Font data types |
| [`nerd-fonts-generator`](./crates/nerd-fonts-generator) | Build-time icon export and code generation |
| [`nerd-fonts-rs`](./crates/nerd-fonts-rs) | Runtime integration (icon resolution, CSS, font loading) |

### Applications

| Crate | Description |
|-------|-------------|
| [`fonts-rs-noto-emoji-cheat-sheet`](./crates/fonts-rs-noto-emoji-cheat-sheet) | Noto Emoji cheat sheet app |
| [`nerd-fonts-cheat-sheet`](./crates/nerd-fonts-cheat-sheet) | Nerd Fonts cheat sheet app |

## Features

- **Modular font family crates** - each font family is a separate crate with
  its own build script, GResource bundle, and runtime API
- **Type-safe glyph names** - `GlyphName<F>` phantom typing prevents mix-ups
  between font families at compile time
- **Variable font support** - variants via Cargo features with axis-based
  rendering (e.g. weight, roundness)
- **GTK4 integration** - GResource registration, icon name resolution, CSS
  providers
- **Software rendering** - font loading via `ab_glyph` for headless rendering
  (see [`pixel-drawing`](https://github.com/smearor/pixel-drawing))
- **Build-time code generation** - `phf::Map` codepoint maps, Rust constants,
  and GResource XML generated from font files
- **Metadata generation** - keywords, categories, and aliases for search
  functionality (e.g. Noto Emoji CLDR annotations)

## Quick Start

### Prerequisites

- Rust toolchain (Edition 2024, MSRV 1.92)
- Linux: `sudo apt-get install -y pkg-config libgtk-4-dev libglib2.0-dev`

### Font Family Crate

```toml
[dependencies]
fonts-rs-doto = "0.1"
```

```rust
use fonts_rs_doto::register_glyphs;
use fonts_rs_doto::GlyphNameExt;
use fonts_rs_doto::DotoName;

fn main() {
    register_glyphs().unwrap();

    let name = DotoName::new("doto-a".to_string());
    if let Some(codepoint) = name.codepoint() {
        println!("Glyph codepoint: U+{:04X}", codepoint as u32);
    }
}
```

### Nerd Fonts

```toml
[dependencies]
nerd-fonts-rs = "0.1"
```

```rust
use nerd_fonts_rs::{init, resolve_icon_codepoint};

fn main() {
    init(None);

    if let Some(c) = resolve_icon_codepoint("nf-fa-gamepad") {
        println!("Gamepad icon: U+{:04X}", c as u32);
    }
}
```

## Documentation

- **User Guide**: [mdBook](https://smearor.github.io/fonts-rs/book/)
- **API Reference**: [docs.rs](https://docs.rs/fonts-rs)
- **Changelog**: [CHANGELOG.md](./CHANGELOG.md)

## Adding a New Font Family

See the [Adding a Font Family](./book/src/adding-font-family.md) guide in the
book for a step-by-step walkthrough.

## Bundled Fonts

This workspace bundles font files from third-party projects. See
[NOTICE](./NOTICE) for licensing details. Font files retain their original
licenses (OFL-1.1 for most fonts, MIT for Nerd Fonts).

## Contributing

Contributions are welcome. Please read the
[Code of Conduct](./CODE_OF_CONDUCT.md) before contributing.

## License

Framework code is licensed under the [MIT License](./LICENSE). Bundled font
files retain their original licenses.