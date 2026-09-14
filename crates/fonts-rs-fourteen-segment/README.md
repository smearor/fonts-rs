# fonts-rs-fourteen-segment

DSEG14 fourteen-segment display font integration for GTK 4 and pixel-drawing.

## Overview

[DSEG14](https://www.keshikan.net/fonts.htm) is a fourteen-segment display
font family with classic and modern styles in regular and mini variants.
This crate integrates DSEG14 into the `fonts-rs` framework with 24 variant
features covering all style/weight combinations.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- 24 variant features (classic/modern x regular/mini x 6 weights)

## Usage

```toml
[dependencies]
fonts-rs-fourteen-segment = "0.1"
# Select a variant:
fonts-rs-fourteen-segment = { version = "0.1", features = ["modern-bold"] }
```

```rust
use fonts_rs_fourteen_segment::register_glyphs;

fn main() {
    register_glyphs().unwrap();
}
```

## Font License

DSEG14 is licensed under the SIL Open Font License (OFL-1.1). The crate
code is MIT.
