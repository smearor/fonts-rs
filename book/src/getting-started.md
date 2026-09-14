# Getting Started

## Prerequisites

### Rust

A working Rust toolchain is required. The project uses Rust Edition 2024.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Minimum Supported Rust Version (MSRV)

The crate declares `rust-version = "1.92"` in its `Cargo.toml`. The MSRV is
verified by CI on every push and pull request.

### Linux System Dependencies

The `gtk` feature requires GTK 4 development libraries.

**Ubuntu/Debian:**

```bash
sudo apt-get install -y pkg-config libgtk-4-dev libglib2.0-dev
```

**Fedora:**

```bash
sudo dnf install -y pkg-config gtk4-devel glib2-devel
```

**Arch Linux:**

```bash
sudo pacman -S pkgconf gtk4 glib2
```

### No System Dependencies (font loading or web only)

If you only need font loading (`render` feature) or web CSS (`web` feature),
no system libraries are required beyond a Rust toolchain.

## Installation

### From crates.io

Each font family is a separate crate. Add the ones you need:

```toml
[dependencies]
# Nerd Fonts (icon name resolution, GTK4 integration)
nerd-fonts-rs = "0.1"

# Specific font families
fonts-rs-doto = "0.1"
fonts-rs-bravura = "0.1"
fonts-rs-seven-segment = "0.1"
fonts-rs-noto-emoji = "0.1"
```

### From Source

```bash
git clone https://github.com/smearor/fonts-rs.git
cd fonts-rs
cargo build --release
```

## Feature Selection

Each font family crate provides the same feature gates:

```toml
# GTK4 integration (default)
fonts-rs-doto = "0.1"

# GTK4 + software rendering + embedded fonts
fonts-rs-doto = { version = "0.1", features = ["gtk", "render", "embed-fonts"] }

# Software rendering only (no GTK)
fonts-rs-doto = { version = "0.1", default-features = false, features = ["render"] }

# Variable font variant selection (Doto example)
fonts-rs-doto = { version = "0.1", features = ["bold-dot"] }
```

For Nerd Fonts specifically:

```toml
# GTK4 with 4.12+ APIs
nerd-fonts-rs = { version = "0.1", features = ["gtk", "v4_12"] }

# Font loading only (no GTK)
nerd-fonts-rs = { version = "0.1", default-features = false, features = ["render"] }

# Web CSS only
nerd-fonts-rs = { version = "0.1", default-features = false, features = ["web"] }

# Icon metadata (keywords & categories)
nerd-fonts-rs = { version = "0.1", features = ["metadata"] }
```

## Quick Start

### Font Family Crate (e.g. Doto)

```rust
use fonts_rs_doto::register_glyphs;
use fonts_rs_doto::GlyphNameExt;
use fonts_rs_doto::DotoName;

fn main() {
    // Register GResource (gtk feature)
    register_glyphs().unwrap();

    // Look up a glyph by name
    let name = DotoName::new("doto-a".to_string());
    if let Some(codepoint) = name.codepoint() {
        println!("Glyph codepoint: U+{:04X}", codepoint as u32);
    }
}
```

### Nerd Fonts (nerd-fonts-rs)

```rust
use nerd_fonts_rs::{init, resolve_icon_codepoint};

fn main() {
    // Call once at startup
    init(None);

    // Resolve an icon name to its Unicode codepoint
    if let Some(c) = resolve_icon_codepoint("nf-fa-gamepad") {
        println!("Gamepad icon: U+{:04X}", c as u32);
    }
}
```

## Running Tests

```bash
cargo test
```

## Building the Documentation

```bash
cd book
mdbook build
```

The rendered HTML is placed in `book/book/`. Open `book/book/index.html` in a
browser, or run `mdbook serve` for live preview during development.
