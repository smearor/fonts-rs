# nerd-fonts-gtk

[![crates.io](https://img.shields.io/crates/v/nerd-fonts-gtk.svg)](https://crates.io/crates/nerd-fonts-gtk)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![CI Build](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/build.yml/badge.svg)](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/build.yml)
[![MSRV](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/msrv.yml/badge.svg)](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/msrv.yml)
[![Security Audit](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/audit.yml/badge.svg)](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/audit.yml)
[![Book](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/book.yml/badge.svg)](https://github.com/smearor/nerd-fonts-gtk/actions/workflows/book.yml)

A Rust library for integrating [Nerd Fonts](https://www.nerdfonts.com/) into GTK4
projects. It provides icon name resolution, font loading, software rendering, and
CSS generation for Nerd Font symbols.

## Features

- **Icon Name Resolution** - Map human-readable names like `nf-fa-gamepad` to
  Unicode codepoints via the `nerd_gtk_icons` codepoint map
- **GTK4 Integration** - Register GResource fonts, apply icon colors to
  `gtk4::Image` and `gtk4::Label` widgets via display-scoped CSS providers
- **Software Rendering** - Draw Nerd Font icons, text labels, progress bars, and
  icon grids onto raw RGBA pixel buffers using `ab_glyph` (no GTK required)
- **CSS Generation** - GTK `@font-face` CSS and web CSS with per-icon
  `content: "\XXXX"` mappings
- **Feature Gates** - Use only what you need: `gtk`, `render`, `web`,
  `embed-fonts`

## Quick Start

### Prerequisites

- Rust toolchain (Edition 2024)
- Linux: `sudo apt-get install -y pkg-config libgtk-4-dev libglib2.0-dev`

### Add to Your Project

```toml
[dependencies]
nerd-fonts-gtk = "0.1"
```

### Usage

```rust
use nerd_fonts_gtk::{init, resolve_icon_codepoint};

fn main() {
    // Call once at startup (with gtk feature)
    init(None);

    // Resolve an icon name to its Unicode codepoint
    if let Some(c) = resolve_icon_codepoint("nf-fa-gamepad") {
        println!("Gamepad icon: U+{:04X}", c as u32);
    }
}
```

### Software Rendering (no GTK)

```rust
use nerd_fonts_gtk::drawing::{fill_background, draw_nerd_font_icon};

let mut pixels = vec![0u8; 64 * 64 * 4];
fill_background(&mut pixels, 64, 64, [30, 30, 30, 255]);
draw_nerd_font_icon(
    &mut pixels, 64, 64,
    "nf-fa-gamepad",
    true,
    nerd_fonts_gtk::resolve_icon_codepoint,
    Some([100, 180, 255, 255]),
);
```

## Feature Flags

| Feature       | Default  | Description                                                        |
|---------------|----------|--------------------------------------------------------------------|
| `gtk`         | yes      | GTK4 icon resolution, color application, GResource registration    |
| `render`      | no       | Software rendering with `ab_glyph` (pixel buffer, no GTK required) |
| `web`         | no       | Web CSS generation (per-icon codepoint mappings)                   |
| `embed-fonts` | no       | Embed font files into the binary via `include_bytes!`              |

## Documentation

- **User Guide**: [mdBook](https://smearor.github.io/nerd-fonts-gtk/book/)
- **API Reference**: [docs.rs](https://docs.rs/nerd-fonts-gtk)
- **Changelog**: [CHANGELOG.md](./CHANGELOG.md)

## Bundled Fonts

This crate bundles font files from third-party projects. See [NOTICE](./NOTICE)
for licensing details.

## Contributing

Contributions are welcome. Please read the
[Code of Conduct](./CODE_OF_CONDUCT.md) before contributing.

## License

Licensed under the [MIT License](./LICENSE).