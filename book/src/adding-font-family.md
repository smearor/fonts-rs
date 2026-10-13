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

## Step 2: Vendor the Upstream Assets

Everything under `resources/` that comes from an upstream project is
**vendored, not copied by hand**: write `upstream.toml` first, then let
`cargo xtask sync -p fonts-rs-{name}` materialize it. The manifest
declares where each file comes from (a pinned git ref or an immutable
URL); `sync` produces `resources/` plus `upstream.lock`, and both are
committed:

```toml
# crates/fonts-rs-{name}/upstream.toml
[upstream]
repo = "myproject/myfont"     # owner/name shorthand or git URL
ref = "v1.2.3"                # tag, branch or commit SHA

[upstream.license]
source = "OFL.txt"
target = "resources/OFL.txt"

[[artifacts]]
url = "https://github.com/myproject/myfont/releases/download/v1.2.3/MyFont-Regular.ttf"
target = "resources/MyFont-Regular.ttf"
```

```bash
cargo xtask sync -p fonts-rs-{name}    # writes resources/ + upstream.lock
cargo xtask verify -p fonts-rs-{name}  # sanity check
```

Result:

```
resources/
├── MyFont-Regular.ttf
└── OFL.txt            (or appropriate license)
```

Curated files without an upstream source may live in `resources/` too -
`verify` warns about them (no lock entry) but accepts them. See the
[Upstream Sourcing](./upstream-sourcing.md) chapter for the full
`upstream.toml` schema, provider-generated metadata (`[[metadata]]`)
and the update-bump procedure (`sync --update`).

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

Define the `FontFamilyMarker` enum and `GlyphName<F>` type alias:

```rust
//! Font family marker type for MyFont.

use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying MyFont in `GlyphName<MyFont>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MyFont {}

impl FontFamilyMarker for MyFont {}
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

### Pre-rendered SVGs (Hybrid)

If the upstream project ships full-color source SVGs (e.g. Blobmoji,
Tabler Icons), use `SvgDirSource` for the GResource and bundle the TTF
separately for `pixel-drawing`. Glyph names come from an external name
map or a `name_resolver` closure; codepoints come from the filename or a
`GlyphNameMap`:

```rust
use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::build_constants;

let config = ExportConfig::<MyFontConfig>::with_variant(*entry);
let svg_dir = Path::new(build_constants::RESOURCES_DIR).join("svg");

FontBuild::new(&font_path)
    .compile_font_gresource()
    .gresource_prefix(MyFontConfig::GRESOURCE_PREFIX)
    .extra_hash(format!("{}-{:016x}", entry.as_str(), hash_svg_dir(&svg_dir)))
    .run(|_font_path, resources_dir| {
        SvgDirSource::new(&svg_dir)
            .filename_parser(EmojiUFilenameParser)       // or default NameParser
            .codepoint_strategy(CodepointStrategy::FromFilename) // or FromNameMap
            .name_resolver(move |parsed| /* map to semantic name */ { todo!() })
            .name_prefix(&glyph_name_prefix)
            .filter(move |codepoints| /* e.g. drop flags for *-noflags */ true)
            .export(&config, resources_dir)
    })
    .map_err(|e| miette::miette!("{e}"))?;
```

Key points:

- **`SvgDirSource`** iterates `resources/svg/` - no TTF outline
  extraction. The SVGs are the final artwork (full color).
- **`filename_parser`** maps filenames to codepoints
  (`EmojiUFilenameParser` for `emoji_u{cp}.svg`, `NameParser` for plain
  `{name}.svg` like Tabler).
- **`codepoint_strategy`** assigns codepoints: `FromFilename` (emoji
  codepoints in the filename) or `FromNameMap` (external
  `GlyphNameMap`, e.g. Tabler's `icons.json` unicode values).
- **`name_resolver`** maps parsed filenames to semantic names (e.g. CLDR
  `tts` names for emoji, `format!("{name}-symbolic")` for icons).
- **`filter`** drops glyphs per variant (e.g. flag emoji for
  `*-noflags`).
- **`dir(other)`** merges a second SVG directory into the same source -
  used when the artwork is split across `links` resource crates
  (`*-glyphs` + `*-sequences` for the emoji families). The sets must
  partition the glyph space; duplicates are a hard error.
- **`extra_hash`** must cover the SVG directory - the default hash only
  covers the TTF file, so SVG changes would not trigger a re-export.
  Call it once per input directory (`hash_svg_dir`).

When the vendored artwork plus TTFs would exceed the crates.io
10 MiB package limit, the family is split into resource crates, one
crate per TTF variant, a generator crate and an umbrella - see
[Build Patterns](./build-patterns.md#pattern-3-multi-crate-emoji-family) (the
emoji families use exactly that topology).

See `crates/fonts-rs-blobmoji-generator/src/build/pipeline.rs` and
`crates/fonts-rs-noto-emoji-generator/src/build/pipeline.rs` (CLDR
name map + flag filter) and `crates/fonts-rs-tabler/build.rs`
(`FromNameMap` + metadata) for complete implementations.

### Fontless SVG-Only Icon Set

Icon sets that ship only SVGs - no TTF at all (e.g. Lucide) - use
`FontBuild::svg_only()` + `run_svg()`. There is no font file to hash,
so change detection relies entirely on `extra_hash` components: hash
the SVG directory, metadata files, and a manual
`EXPORT_PIPELINE_VERSION` constant (bump it when the export pipeline
itself changes - naming, post-processors):

```rust
const EXPORT_PIPELINE_VERSION: u32 = 1;

FontBuild::svg_only()
    .extra_hash(EXPORT_PIPELINE_VERSION.to_string())
    .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
    .extra_hash(format!("{:016x}", hash_str(&icons_json)))
    .rerun_if_changed(&svg_dir)
    .run_svg(|resources_dir| {
        SvgDirSource::new(&svg_dir)
            .codepoint_strategy(CodepointStrategy::HashedPua)
            .name_resolver(|parsed| Some(format!("{}-symbolic", parsed.raw_name)))
            .name_prefix(MyFontConfig::FONT_FAMILY_NAME)
            .post_process(SymbolicClasses)   // stroke-based sets: symbolic classes
            .export(&config, resources_dir)
    })
    .map_err(|e| miette::miette!("{e}"))?;
```

Key points:

- **`CodepointStrategy::HashedPua`** allocates a stable PUA codepoint
  per glyph via hash-of-name - needed because the `phf` maps require
  unique codepoints and upstream ships none.
- **`post_process(SymbolicClasses)`** adds GTK symbolic classes to
  stroke-based SVGs. Without it, `*-symbolic` icons render as solid
  blobs - GTK's symbolic recoloring ignores `fill="none"` and fills
  every shape. Requires GTK ≥ 4.20 for stroke rendering.
- No `set_font_path_env`, no `font.gresource`, no `render`/`pango`
  capability - `lib.rs` declares only `[icon_theme, html_svg]`.

See `crates/fonts-rs-lucide/build.rs` for the complete implementation.

### Multi-TTF Webfont Family (One TTF per Style)

Icon sets that ship **one TTF per style** (Font Awesome 5 model -
e.g. Line Awesome with `la-regular-400`/`la-solid-900`/`la-brands-400`)
are a hybrid: GResource SVGs from a vendored `svg/` directory plus a
`font.gresource` bundling **all** faces:

```rust
let font_path = resources.join("fonts").join("la-solid-900.ttf");

// Per-face paths for ab_glyph/pixel-drawing at runtime:
set_font_path_env("MYFONT_REGULAR_FONT_PATH", &resources.join("fonts").join("my-regular.ttf"))?;
set_font_path_env("MYFONT_SOLID_FONT_PATH", &font_path)?;

FontBuild::new(&font_path)
    .compile_font_gresource()   // NO .gresource_prefix() - see below
    .extra_hash(format!("{EXPORT_PIPELINE_VERSION}"))
    .extra_hash(format!("{:016x}", hash_svg_dir(&svg_dir)))
    .extra_hash(format!("{:016x}", hash_str(&codepoints_json)))
    .rerun_if_changed(&svg_dir)
    .rerun_if_changed(resources.join("fonts"))
    .run(|_font_path, resources_dir| {
        SvgDirSource::new(&svg_dir)
            .codepoint_map(&codepoints)
            .codepoint_strategy(CodepointStrategy::FromNameMap)
            .post_process(CurrentColor::new(PaintTarget::Fill).inject_missing())
            .export(&config, resources_dir)
    })?;
```

Key differences from a single-TTF family:

- **Static `resources/font.gresource.xml`** - when
  `compile_font_gresource()` is called *without*
  `gresource_prefix(...)`, `FontBuild` falls back to the hand-written
  manifest in `resources/` instead of generating one. The generated
  manifest covers exactly one TTF; a static manifest can list all
  faces with aliases.
- **Manual `GtkFont` impl** - `impl_gtk_font!` cannot express
  multi-TTF families. Implement it by hand with your own
  `FONT_RESOURCE_PATH` constant(s) for the bundled faces; the
  `load_font_from_gresource(path, tag)` helper still collapses the
  `OnceLock` + temp-extraction body to one call.
- **Manual `PangoFont::install_pango`** - a three-line override that
  calls the shared `PangoFont::install_pango_resources(font_map,
  &RESOURCE_PATHS, crate_name)` helper once per face.
- **`CurrentColor::inject_missing()`** - required when upstream SVG
  paths carry *no* paint attribute at all (plain `CurrentColor`
  rewriting would be a no-op); it injects `fill="currentColor"` on
  the `<svg>` root. Add a post-export assertion that every emitted
  SVG actually contains the injected fill.
- **Shared codepoints** - style pairs (e.g. `adjust` /
  `adjust-solid`) share one FA5 codepoint. `CodemapGenerator`
  dedupes the forward `char -> name` map per codepoint (the
  canonical/shortest name wins) while the reverse `name -> char` map
  keeps every exported name - `phf_map!` rejects duplicate keys.
- **`SoftwareRender`** needs a variant enum (`LineAwesomeStyle`) to
  select the face, since `type Variant = ()` cannot express multiple
  TTFs.

See `crates/fonts-rs-line-awesome/build.rs` and `src/gtk.rs` for the
complete implementation.

## Step 6: Write `family.rs` and `lib.rs`

`family.rs` wires the generated phf maps into the `FontFamily` trait
and declares the family's capabilities via `impl_font_family!`:

```rust
//! Runtime font family API for MyFont.

use crate::codepoint_map;
use crate::definition::MyFontConfig;
use crate::naming::MyFont;
use crate::naming::MyFontName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    MyFont,
    MyFontConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = MyFontName,
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
```

Valid capability names: `icon_theme` (`IconThemeFont` - icon-theme
lookup, `glyph_svg`, `compose_svg`), `pango` (`PangoFont` - needs a
TTF), `html_svg` (`HtmlSvgExport` - inline-SVG HTML export, fontless
compatible), `html` (`HtmlExport` - additionally `@font-face` export,
needs a TTF). Fontless sets declare only `[icon_theme, html_svg]`.

The macro emits the `FontFamily` impl (`codepoint`, `glyph_name`,
`all_glyphs`, `glyph_count`), the declared capability impls, and
inherent forwarders so callers can write `MyFont::codepoint("x")`
without importing the trait.

`lib.rs` ties everything together:

```rust
//! MyFont font integration for GTK 4.

pub mod codepoint_map;
pub mod constants;
pub mod definition;
mod family;
pub mod naming;
pub mod variant;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "render")]
pub mod fonts;

pub use definition::MyFontConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::MyFont;
pub use naming::MyFontName;

/// Type alias for this font family's glyph name type.
pub type FamilyName = MyFontName;
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
        let count = MyFont::glyph_count();
        assert!(count > 0, "MyFont should export at least one glyph");
    }

    #[test]
    fn all_glyphs_names_start_with_family_prefix() {
        for (_, name) in MyFont::all_glyphs() {
            assert!(
                name.as_ref().starts_with("myfont-"),
                "glyph name '{name}' should start with 'myfont-'"
            );
        }
    }

    #[test]
    fn all_glyphs_names_are_lowercase() {
        for (_, name) in MyFont::all_glyphs() {
            assert_eq!(name.as_ref(), name.as_ref().to_lowercase(), "glyph name '{name}' should be lowercase");
        }
    }

    #[test]
    fn all_glyphs_names_contain_no_underscores() {
        for (_, name) in MyFont::all_glyphs() {
            assert!(!name.as_ref().contains('_'), "glyph name '{name}' should not contain underscores");
        }
    }

    #[test]
    fn all_glyphs_codepoints_are_unique() {
        let codepoints: Vec<char> = MyFont::all_glyphs().map(|(c, _)| c.as_char()).collect();
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
