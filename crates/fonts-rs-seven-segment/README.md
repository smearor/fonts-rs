# fonts-rs-seven-segment

DSEG7 seven-segment display font integration for GTK 4 and pixel-drawing.

## Overview

[DSEG7](https://www.keshikan.net/fonts.htm) is a seven-segment display font
family with classic and modern styles in regular and mini variants. This
crate integrates DSEG7 into the `fonts-rs` framework with 24 variant
features covering all style/weight combinations.

## Features

- **`gtk`** (default) - GResource registration and GTK4 icon name resolution
- **`render`** - Font loading via `ab_glyph` for software rendering
- **`embed-fonts`** - Embed font file via `include_bytes!`
- 24 variant features (classic/modern x regular/mini x 6 weights)

## Usage

```toml
[dependencies]
fonts-rs-seven-segment = "0.1"
# Select a variant:
fonts-rs-seven-segment = { version = "0.1", features = ["modern-bold"] }
```

```rust
use fonts_rs_seven_segment::register_glyphs;

fn main() {
    register_glyphs().unwrap();
}
```

## Font License

DSEG7 is licensed under the SIL Open Font License (OFL-1.1). The crate code
is MIT.
