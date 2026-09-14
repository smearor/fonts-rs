# `fonts-rs-generator` - Build-Time Pipeline

The `fonts-rs-generator` crate provides the build-time pipeline for exporting
glyphs from TTF/OTF font files, generating codepoint maps, compiling
GResource bundles, and producing Rust source constants. It is used as a
build-dependency by every font family crate.

## `ExportConfig<X: FontFamilyConfig>`

Runtime configuration for glyph export, decoupled from `FontDefinition`.
Enables variant-specific parameters at build time without a separate
`FontDefinition` impl per variant.

```rust
pub struct ExportConfig<X: FontFamilyConfig> {
    pub gresource_prefix: String,
    pub glyph_name_prefix: String,
    pub axes: AxisValues,
    pub name_filter: Option<fn(&str) -> bool>,
    pub family_display_name: String,
    _marker: PhantomData<X>,
}
```

### Constructors

- `ExportConfig::new()` - without a variant, uses `X::GRESOURCE_PREFIX`
  and `X::FONT_FAMILY_NAME`
- `ExportConfig::with_variant(variant)` - with a variant, constructs
  `gresource_prefix` as `{X::GRESOURCE_PREFIX}/{variant_slug}` and
  `glyph_name_prefix` as `{X::FONT_FAMILY_NAME}-{variant}`

### Methods

- **`export_glyphs(font_path, output_dir)`** - standard export:
  iterates over all glyph IDs, normalizes names to
  `{glyph_name_prefix}-{kebab}`, renders SVGs with axis location
- **`export_glyphs_by_name_map(font_path, output_dir, name_map)`** -
  export via external name→codepoint map (e.g. SMuFL `glyphnames.json`):
  for fonts without PostScript glyph names
- **`write_variant_info()`** - writes `OUT_DIR/variant.rs` with
  `GRESOURCE_PREFIX` and `GLYPH_PREFIX` constants
- **`generate_gresource_xml(entries, path)`** - generates
  `icons.gresource.xml` with `quick-xml` serialization

## `export_glyphs()` (Two Variants)

There are two distinct `export_glyphs` methods in the framework:

### `FontDefinition::export_glyphs` (trait default method)

Uses `Self::normalize_name` for type-safe glyph names. Iterates over all
glyph IDs, calls `should_skip` and `normalize_name`, renders SVGs without
axis location (default location).

### `ExportConfig::export_glyphs` (struct method)

Uses `normalize_to_kebab` + `glyph_name_prefix` for string-based names.
Renders SVGs with axis location (`glyph_to_svg_full_height_at`).
Supports `name_filter` for custom glyph filtering.

### Output

Both produce:
- `<output_dir>/scalable/{context}/*.svg`
- `<output_dir>/metadata.json`
- `<output_dir>/icons.gresource.xml`

## `write_variant_info()`

Writes `OUT_DIR/variant.rs` with two constants:

```rust
pub const GRESOURCE_PREFIX: &str = "/io/smearor/fonts/doto/regular_medium";
pub const GLYPH_PREFIX: &str = "doto-regular-medium";
```

Included by `lib.rs` via `include!(concat!(env!("OUT_DIR"), "/variant.rs"))`.
Enables the runtime API to construct GResource paths and glyph names with
the active variant prefix.

## `detect_and_get_active_variant()`

On `VariantList<X: FontFamilyConfig>`:

```rust
pub fn detect_and_get_active_variant(&self, default_index: usize) -> miette::Result<&'a FontVariant>
```

Scans `CARGO_FEATURE_{NAME}` environment variables for each variant.
Returns the active variant or `default_index` if none is active. Returns
an error if more than one variant is active. Uses `X::FAMILY_DISPLAY_NAME`
for diagnostic messages.

## `FontBuild` (Builder)

Builder for the common `build.rs` pipeline. Handles hash-based change
detection, conditional glyph export, GResource compilation, and code
generator invocation.

```rust
FontBuild::new(&font_path)
    .extra_hash(entry.as_str())     // for variable font variants
    .compile_font_gresource()       // optional: font.gresource
    .rerun_if_changed(path)         // additional cargo:rerun-if-changed
    .additional_gresource(xml, out) // additional GResource bundles
    .run(|font_path, resources_dir| {
        config.export_glyphs(font_path, resources_dir)
    })
    .map_err(|e| miette::miette!("{e}"))?;
```

### Builder Methods

- **`extra_hash`**: Additional hash component (e.g. variant name) - forces
  re-export on variant change even if the font file is unchanged
- **`compile_font_gresource`**: Also compile `font.gresource` in addition to
  `icons.gresource`
- **`rerun_if_changed`**: Add extra `cargo:rerun-if-changed` paths
- **`additional_gresource`**: Register additional GResource bundles to compile

### Run Methods

- **`run`**: Export closure + default code generation
  (`CodemapGenerator` + `RustConstantsGenerator`)
- **`run_with`**: Export closure + custom code generation closure
- Hash is computed as FNV-1a of the font file and stored in `resources/.hash`

## `impl_font_loader!` Macro

Generates a `font() -> Option<&'static ab_glyph::FontVec>` function that
loads a TTF font file and caches it in a `OnceLock`.

Two variants:

```rust
// Literal: relative to resources/
fonts_rs_generator::impl_font_loader!("Doto.ttf");

// Env var: absolute path set by build.rs via cargo:rustc-env
fonts_rs_generator::impl_font_loader!(env: "DOTO_FONT_PATH");
```

With the `embed-fonts` feature, `include_bytes!` is used; otherwise
`std::fs::read`.

## `MetadataGenerator` (Trait)

Trait for generating `phf::Map` metadata tables (keywords, categories,
aliases) at build time.

```rust
pub trait MetadataGenerator {
    type Name: AsRef<str>;

    fn keywords_for(&self, entry: &GlyphEntry<Self::Name>) -> Vec<String>;
    fn categories_for(&self, entry: &GlyphEntry<Self::Name>) -> Vec<String>;
    fn aliases(&self) -> Vec<(String, String)> { Vec::new() }
    fn deduplicate_statics() -> bool { false }

    fn generate_keywords(&self, entries: &[GlyphEntry<Self::Name>]) -> String { ... }
    fn generate_categories(&self, entries: &[GlyphEntry<Self::Name>]) -> String { ... }
    fn generate_aliases(&self) -> String { ... }
    fn run(&self, entries: &[GlyphEntry<Self::Name>]) -> Result<(), std::io::Error> { ... }
}
```

- `deduplicate_statics() = true`: Extracts repeated value lists into
  `static` constants (e.g. `static KW_0: &[GlyphKeyword] = &[...]`) -
  reduces binary size for large maps with many duplicates
- `run()`: Writes `keywords.rs`, `categories.rs`, `aliases.rs` to
  `OUT_DIR`

Example implementation: `NotoEmojiMetadataGenerator` in
`fonts-rs-noto-emoji-generator` uses `CodePointKeywordMap` and
`CodePointCategoryMap` as data sources.

## `VariantList<X: FontFamilyConfig>`

A typed list of font variants, generic over the font family config.
Provides methods to detect the active variant from Cargo features.

```rust
const VARIANTS: VariantList<DotoConfig> = VariantList::new(&[
    FontVariant::axes("regular-medium", &[AxisValue::new("wght", 500.0), AxisValue::new("ROND", 50.0)]),
    FontVariant::axes("bold-dot", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 100.0)]),
]);

let entry = VARIANTS.detect_and_get_active_variant(0)?;
```

## `FontVariantExt` (Extension Trait)

Extension trait providing a method to get the font file path from a
`FontVariant`, handling errors if no font file is associated.

## `set_font_path_env()` Helper

Sets `cargo:rustc-env` with the absolute font path for `include_bytes!`
in `lib.rs`. Constructs the absolute path from `CARGO_MANIFEST_DIR` and
the given relative font path.

```rust
set_font_path_env("DOTO_FONT_PATH", &font_path)?;
```

## `build_constants` Module

Provides common constants for build scripts:

- `RESOURCES_DIR` - the `resources/` directory path
- `METADATA_PATH` - path to `resources/metadata.json`
- `HASH_PATH` - path to `resources/.hash`
- `ICONS_GRESOURCE_XML` - path to `resources/icons.gresource.xml`
- `ICONS_GRESOURCE` - path to compiled `icons.gresource`
- `FONT_GRESOURCE_XML` - path to `resources/font.gresource.xml`
- `FONT_GRESOURCE` - path to compiled `font.gresource`
