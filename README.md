# fonts-rs

[![crates.io](https://img.shields.io/crates/v/nerd-fonts-rs.svg)](https://crates.io/crates/nerd-fonts-rs)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![CI Build](https://github.com/smearor/fonts-rs/actions/workflows/build.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/build.yml)
[![MSRV](https://github.com/smearor/fonts-rs/actions/workflows/msrv.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/msrv.yml)
[![Security Audit](https://github.com/smearor/fonts-rs/actions/workflows/audit.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/audit.yml)
[![Book](https://github.com/smearor/fonts-rs/actions/workflows/book.yml/badge.svg)](https://github.com/smearor/fonts-rs/actions/workflows/book.yml)

A Rust library for integrating [Nerd Fonts](https://www.nerdfonts.com/) into GTK4
projects. It provides icon name resolution, font loading, and CSS generation
for Nerd Font symbols. Software rendering is available via the
[`pixel-drawing`](https://github.com/smearor/pixel-drawing) crate.

## Features

- **Icon Name Resolution** - Map human-readable names like `nf-fa-gamepad` to
  Unicode codepoints via the vendored codepoint map
- **GTK4 Integration** - Register GResource fonts, apply icon colors to
  `gtk4::Image` and `gtk4::Label` widgets via display-scoped CSS providers
- **Font Loading** - Load Nerd Font TTF/WOFF2 files via `ab_glyph` for use with
  [`pixel-drawing`](https://github.com/smearor/pixel-drawing) (no GTK required)
- **CSS Generation** - GTK `@font-face` CSS and web CSS with per-icon
  `content: "\XXXX"` mappings
- **Feature Gates** - Use only what you need: `gtk`, `v4_12`, `render`, `web`,
  `embed-fonts`, `metadata`

## Quick Start

### Prerequisites

- Rust toolchain (Edition 2024)
- Linux: `sudo apt-get install -y pkg-config libgtk-4-dev libglib2.0-dev`

### Add to Your Project

```toml
[dependencies]
nerd-fonts-rs = "0.1"
```

### Usage

```rust
use nerd_fonts_rs::{init, resolve_icon_codepoint};

fn main() {
    // Call once at startup (with gtk feature)
    init(None);

    // Resolve an icon name to its Unicode codepoint
    if let Some(c) = resolve_icon_codepoint("nf-fa-gamepad") {
        println!("Gamepad icon: U+{:04X}", c as u32);
    }
}
```

## Feature Flags

| Feature       | Default  | Description                                                         |
|---------------|----------|---------------------------------------------------------------------|
| `gtk`         | yes      | GTK4 icon resolution, color application, GResource registration     |
| `v4_12`       | no       | Enable GTK 4.12+ APIs (`load_from_string` for `CssProvider`)        |
| `render`      | no       | Font loading with `ab_glyph` (TTF/WOFF2, no GTK required)           |
| `web`         | no       | Web CSS generation (per-icon codepoint mappings)                    |
| `embed-fonts` | no       | Embed font files into the binary via `include_bytes!`               |
| `metadata`    | no       | Icon metadata: keywords and categories for search (~200-500 KB)     |

## Documentation

- **User Guide**: [mdBook](https://smearor.github.io/fonts-rs/book/)
- **API Reference**: [docs.rs](https://docs.rs/nerd-fonts-rs)
- **Changelog**: [CHANGELOG.md](./CHANGELOG.md)

## Bundled Fonts

This crate bundles font files from third-party projects. See [NOTICE](./NOTICE)
for licensing details.

## Contributing

Contributions are welcome. Please read the
[Code of Conduct](./CODE_OF_CONDUCT.md) before contributing.

## License

Licensed under the [MIT License](./LICENSE).