# Getting Started

## Prerequisites

### Rust

A working Rust toolchain is required. The project uses Rust Edition 2024.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Minimum Supported Rust Version (MSRV)

The crate declares `rust-version = "1.85"` in its `Cargo.toml`. The MSRV is
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

### No System Dependencies (render-only)

If you only need software rendering (`render` feature) or web CSS (`web` feature),
no system libraries are required beyond a Rust toolchain.

## Installation

### From crates.io

```toml
[dependencies]
nerd-fonts-gtk = "0.1"
```

### From Source

```bash
git clone https://github.com/smearor/nerd-fonts-gtk.git
cd nerd-fonts-gtk
cargo build --release
```

## Feature Selection

Choose only the features you need:

```toml
# GTK4 integration (default)
nerd-fonts-gtk = "0.1"

# Software rendering only (no GTK)
nerd-fonts-gtk = { version = "0.1", default-features = false, features = ["render"] }

# Web CSS only
nerd-fonts-gtk = { version = "0.1", default-features = false, features = ["web"] }

# Everything + embedded fonts
nerd-fonts-gtk = { version = "0.1", features = ["gtk", "render", "web", "embed-fonts"] }
```

## Quick Start

### GTK4 Application

```rust
use nerd_fonts_gtk::{init, resolve_icon_codepoint};

fn main() {
    // Call once at startup
    init(None);

    // Resolve an icon name to its Unicode codepoint
    if let Some(c) = resolve_icon_codepoint("nf-fa-gamepad") {
        println!("Gamepad icon: U+{:04X}", c as u32);
    }
}
```

### Software Rendering

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
