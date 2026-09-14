# Adding a New Font Family

This guide walks through creating a new font family crate in the
`fonts-rs` workspace. The process is designed to require minimal
boilerplate - most of the build pipeline is handled by the generic
framework in `fonts-rs-generator`.

## Overview

A font family crate integrates a TTF/OTF font into the `fonts-rs`
ecosystem, providing:

- Build-time glyph export to SVG files
- GResource bundle compilation for GTK
- Codepoint map generation (`phf::Map`)
- Runtime glyph name resolution and GResource registration
- Optional software rendering via `ab_glyph`

## Step 1: Create the Crate

Create a new directory under `crates/`:

```
crates/fonts-rs-{name}/
├── Cargo.toml
├── build.rs
├── resources/
│   └── {font}.ttf
└── src/
    ├── lib.rs
    ├── definition.rs
    ├── naming.rs
    ├── variant.rs
    ├── codepoint_map.rs
    ├── constants.rs
    └── fonts.rs       (optional)
```

## Step 2: Bundle the Font File

Place the TTF/OTF font file and its license into `resources/`:

```
resources/
├── MyFont-Regular.ttf
└── OFL.txt            (or appropriate license)
```

## Step 3: Write `definition.rs`

Implement `FontFamilyConfig` with the family's constants:

```rust
//! `FontFamilyConfig` for the MyFont font family.

use const_format::concatcp;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::PUA_RANGE;

pub struct MyFontConfig;

impl FontFamilyConfig for MyFontConfig {
    const FONT_FAMILY_NAME: &'static str = "myfont";
    const FAMILY_DISPLAY_NAME: &'static str = "MyFont";
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", MyFontConfig::FONT_FAMILY_NAME);
    const CODEPOINT_RANGES: &[CodePointRange] = &[PUA_RANGE];
}
```

Choose `CODEPOINT_RANGES` based on the font's Unicode coverage:
- `BMP_RANGE` - full BMP (default, most fonts)
- `ASCII_PRINTABLE_RANGE` - barcode fonts, seven-segment display fonts
- `PUA_RANGE` - SMuFL music fonts, icon fonts in the Private Use Area

## Step 4: Write `naming.rs`

Define the `FontFamily` marker enum and `GlyphName<F>` type alias:

```rust
//! Font family marker type for MyFont.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying MyFont in `GlyphName<MyFont>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MyFont {}

impl FontFamily for MyFont {}
impl sealed::Sealed for MyFont {}

/// Convenience type alias for MyFont glyph names.
pub type MyFontName = GlyphName<MyFont>;
```

## Step 5: Write `build.rs`

Choose the pattern that matches your font:

### Variable Font with Variants

If the font has multiple variants (e.g. weight/roundness combinations),
use `VariantList` + `ExportConfig::with_variant`:

```rust
use std::path::Path;

use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::AxisValue;
use fonts_rs_model::FontVariant;

#[path = "src/definition.rs"]
mod definition;
use definition::MyFontConfig;

const FONT_FILE: &str = "MyFont.ttf";

const VARIANTS: VariantList<MyFontConfig> = VariantList::new(&[
    FontVariant::axes("regular", &[AxisValue::new("wght", 400.0)]),
    FontVariant::axes("bold", &[AxisValue::new("wght", 700.0)]),
]);

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(FONT_FILE);

    set_font_path_env("MYFONT_FONT_PATH", &font_path)?;

    let config = ExportConfig::<MyFontConfig>::with_variant(*entry);

    FontBuild::new(&font_path)
        .extra_hash(entry.as_str())
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;
    Ok(())
}
```

### Static Font (No Variants)

If the font has a single style, use `ExportConfig::new()`:

```rust
use std::path::Path;

use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;

#[path = "src/definition.rs"]
mod definition;
use definition::MyFontConfig;

const FONT_FILE: &str = "MyFont-Regular.ttf";

fn main() -> miette::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(FONT_FILE);

    set_font_path_env("MYFONT_FONT_PATH", &font_path)?;

    let config = ExportConfig::<MyFontConfig>::new();

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;
    Ok(())
}
```

### Font with External Name Map

If the font lacks PostScript glyph names (e.g. SMuFL fonts), use
`export_glyphs_by_name_map` with an external metadata file:

```rust
use fonts_rs_model::GlyphNameMap;

// Parse external metadata (e.g. SMuFL glyphnames.json)
let name_map: GlyphNameMap = serde_json::from_str(&json)?;

let config = ExportConfig::<MyFontConfig>::new();

FontBuild::new(&font_path)
    .run(|font_path, resources_dir| {
        config.export_glyphs_by_name_map(font_path, resources_dir, &name_map)
    })
    .map_err(|e| miette::miette!("{e}"))?;
```

## Step 6: Write `lib.rs`

```rust
//! MyFont font integration for GTK 4.

pub mod codepoint_map;
pub mod constants;
pub mod naming;
pub mod variant;

#[cfg(feature = "render")]
pub mod fonts;

use fonts_rs_model::CodePoint;

pub use naming::MyFontName;

/// Type alias for this font family's glyph name type.
pub type FamilyName = MyFontName;

/// Extension trait adding codepoint lookup to `GlyphName<MyFont>`.
pub trait GlyphNameExt {
    fn codepoint(&self) -> Option<CodePoint>;
}

impl GlyphNameExt for MyFontName {
    fn codepoint(&self) -> Option<CodePoint> {
        let key = self.as_ref();
        if let Some(ch) = codepoint_map::REVERSE_GLYPHS.get(key).copied() {
            return Some(CodePoint::from(ch));
        }
        let full = format!("{}-{}", variant::GLYPH_PREFIX, key);
        codepoint_map::REVERSE_GLYPHS.get(&full).copied().map(CodePoint::from)
    }
}

/// Registers embedded glyphs as a GResource.
#[cfg(feature = "gtk")]
pub fn register_glyphs() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")?;
    Ok(())
}

/// Returns an iterator over all known glyphs.
pub fn all_glyphs() -> impl Iterator<Item = (char, &'static str)> {
    codepoint_map::GLYPHS.entries().map(|(c, name)| (*c, *name))
}
```

### Generated Module Stubs

Create these stub files that `include!` the generated code:

**`src/codepoint_map.rs`**:
```rust
include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
```

**`src/constants.rs`**:
```rust
include!(concat!(env!("OUT_DIR"), "/icons.rs"));
```

**`src/variant.rs`**:
```rust
include!(concat!(env!("OUT_DIR"), "/variant.rs"));
```

**`src/fonts.rs`** (optional, for `render` feature):
```rust
fonts_rs_generator::impl_font_loader!(env: "MYFONT_FONT_PATH");
```

## Step 7: Configure `Cargo.toml`

```toml
[package]
name = "fonts-rs-myfont"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
repository.workspace = true
homepage.workspace = true
description = "MyFont font integration for GTK 4 and pixel-drawing"
publish = true

# Font license
license-file = "resources/OFL.txt"

include = [
    "src/**/*.rs",
    "build.rs",
    "resources/*.ttf",
    "resources/OFL.txt",
    "resources/icons.gresource.xml",
    "Cargo.toml",
]

[features]
default = ["gtk"]

# Variant features (if applicable)
regular = []
bold = []

gtk = ["dep:gtk4", "dep:gio"]
render = ["dep:ab_glyph"]
embed-fonts = []

[dependencies]
fonts-rs-generator.workspace = true
fonts-rs-model.workspace = true
gio = { workspace = true, optional = true }
gtk4 = { workspace = true, optional = true }
ab_glyph = { workspace = true, optional = true }
phf.workspace = true
tracing.workspace = true

[build-dependencies]
fonts-rs-generator.workspace = true
fonts-rs-model.workspace = true
const_format.workspace = true
miette.workspace = true

[lints]
workspace = true
```

## Step 8: Update Workspace `Cargo.toml`

Add the new crate to the workspace members and dependencies:

```toml
[workspace]
members = [
    # ...
    "crates/fonts-rs-myfont",
]

[workspace.dependencies]
# ...
fonts-rs-myfont = { version = "0.1.0", path = "crates/fonts-rs-myfont" }
```

## Step 9: Write Tests

Add tests to `lib.rs` to verify the export:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_is_not_empty() {
        let count = all_glyphs().count();
        assert!(count > 0, "MyFont should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_family_prefix() {
        for (_, name) in all_glyphs() {
            assert!(
                name.starts_with("myfont-"),
                "glyph name '{name}' should start with 'myfont-'"
            );
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in all_glyphs() {
            assert_eq!(*name, name.to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in all_glyphs() {
            assert!(!name.contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = all_glyphs().map(|(c, _)| c).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }
}
```

## Verification

Build the crate and run tests:

```bash
cargo build -p fonts-rs-myfont
cargo test -p fonts-rs-myfont
cargo clippy -p fonts-rs-myfont -- -D warnings
cargo fmt -p fonts-rs-myfont -- --check
```

Check the generated output:

```bash
ls resources/scalable/glyphs/*.svg | wc -l    # glyph count
cat resources/metadata.json | head -20         # metadata preview
```
