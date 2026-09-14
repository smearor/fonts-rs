# `fonts-rs-model` - Generic Model Types

The `fonts-rs-model` crate provides font-family-agnostic types shared across
all `fonts-rs-*` crates. These types are the foundation of the modular fonts
framework, enabling any font family crate to use the same build pipeline
without Nerd Fonts dependencies.

## `FontFamily` (Marker Trait)

A sealed marker trait that identifies each font family at compile time.
Each font family crate defines a zero-sized enum implementing `FontFamily`
and the private `Sealed` trait. This prevents external crates from defining
their own font family markers.

```rust
pub trait FontFamily: sealed::Sealed {}
```

Used as the phantom type parameter in `GlyphName<F>`, so that
`GlyphName<Doto>` and `GlyphName<SevenSegment>` are distinct types at
compile time - preventing mix-ups between font families.

### Example

```rust
use fonts_rs_model::FontFamily;
use fonts_rs_model::sealed;

pub enum Doto {}

impl FontFamily for Doto {}
impl sealed::Sealed for Doto {}
```

## `FontFamilyConfig` (Trait)

Base configuration for all font families. Provides compile-time constants
for GResource prefixes, icon context directories, and codepoint ranges.
Used by both `FontDefinition` (type-safe glyph names) and `ExportConfig`
(runtime variant pipeline).

```rust
pub trait FontFamilyConfig {
    const FONT_FAMILY_NAME: &'static str;
    const FAMILY_DISPLAY_NAME: &'static str;
    const GRESOURCE_PREFIX: &'static str;
    const ICONS_CONTEXT: &'static str = ICONS_CONTEXT_GLYPHS;
    const CODEPOINT_RANGES: &[CodePointRange] = &[BMP_RANGE];

    fn icons_dir(output_dir: &Path) -> PathBuf { ... }
    fn prepare_icons_dir(output_dir: &Path) -> std::io::Result<PathBuf> { ... }
    fn icons_resource_prefix() -> String { ... }
}
```

### Constants

- **`FONT_FAMILY_NAME`**: Slug for GResource prefix and glyph names (e.g.
  `"doto"`, `"dseg7"`)
- **`FAMILY_DISPLAY_NAME`**: Human-readable name for build logs (e.g.
  `"Doto"`, `"DSEG7"`)
- **`GRESOURCE_PREFIX`**: Full GResource prefix (e.g.
  `/io/smearor/fonts/doto`)
- **`ICONS_CONTEXT`**: Subdirectory after `scalable/` - default `"glyphs"`,
  override e.g. `"emoji"` for Noto Emoji
- **`CODEPOINT_RANGES`**: Unicode ranges for reverse cmap probing -
  default BMP, override e.g. `ASCII_PRINTABLE_RANGE` for barcode fonts

### Methods

- **`prepare_icons_dir`**: Removes the `scalable/{context}/` directory and
  recreates it (avoids stale SVGs from previous builds)
- **`icons_dir`**: Returns the path to `scalable/{context}/` within the
  output directory

## `FontDefinition` (Trait)

Extends `FontFamilyConfig` with type-safe glyph name processing and a
default `export_glyphs` pipeline. Used by font families with semantic
PostScript glyph names (e.g. Barcode, Seven-Segment).

```rust
pub trait FontDefinition: FontFamilyConfig {
    type Name: AsRef<str> + Clone + Ord + Serialize + for<'a> Deserialize<'a>;
    type Family: FontFamily;

    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name>;
    fn should_skip(raw_glyph_name: &str) -> bool { ... }
    fn generate_gresource_xml(...) -> std::io::Result<()> { ... }
    fn export_glyphs(font_path: &Path, output_dir: &Path) -> std::io::Result<usize> { ... }
}
```

### Associated Types

- **`Name`**: Glyph name type - `GlyphName<Self::Family>` for simple
  families, `IconName` for Nerd Fonts
- **`Family`**: Font family marker (e.g. `Doto`, `SevenSegment`)

### Methods

- **`normalize_name`**: Raw glyph name → normalized name (e.g.
  `"zero"` → `"dseg7-0"`)
- **`should_skip`**: Filter for glyph names to skip (default: `.`, `uni`,
  `u` - overridable)
- **`export_glyphs`**: Default pipeline: SVG export, `metadata.json`,
  `icons.gresource.xml`

## `FontVariant`

Identifies a font variant (e.g. `"classic-regular"`, `"bold-dot"`) and
describes how it is realized: separate file, variable font axes, or both.

```rust
pub struct FontVariant {
    name: &'static str,
    variant_type: FontVariantType,
}
```

### Constructors

```rust
// Separate file, no axes
FontVariant::file("classic-regular", "DSEG7Classic-Regular");

// Shared file, axes only (variable font)
FontVariant::axes("bold-dot", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 100.0)]);

// Separate file with additional axes
FontVariant::file_with_axes("bold-extended", "MyFont-Bold", &[AxisValue::new("wdth", 125.0)]);
```

### Methods

- **`is_active()`**: Checks the `CARGO_FEATURE_{NAME}` environment variable
- **`slug()`**: Variant name with `_` instead of `-` (for GResource paths)
- **`axis_values()`**: Returns the axis configuration
- **`font_file()`**: Returns the font file (if separate)

## `FontVariantType`

Describes how a variant is realized:

```rust
pub struct FontVariantType {
    pub font_file: Option<FontFile>,
    pub axis_values: AxisValues,
}
```

- **`file()`**: Separate file, no axes
- **`axes()`**: Shared file, axes only (variable font)
- **`file_with_axes()`**: Separate file with additional axes

## Axes (`Axis`, `AxisValue`, `AxisValues`)

Variable font axis identification and values:

```rust
pub struct Axis(&'static str);  // e.g. "wght", "ROND"

pub struct AxisValue {
    pub axis: Axis,
    pub value: f32,
}

pub struct AxisValues(&'static [AxisValue]);
```

- `AxisValues::EMPTY` - default location (no axes)
- `AxisValues` is `const`-compatible for `VARIANTS` arrays in `build.rs`
- `Deref` to `[AxisValue]` for iteration

## `CodePointRange`

An inclusive Unicode range for reverse cmap probing:

```rust
pub struct CodePointRange {
    start: CodePoint,
    end: CodePoint,
}
```

Predefined constants in `fonts-rs-model`:

- `BMP_RANGE` - `U+0000`–`U+FFFF` (Default)
- `ASCII_PRINTABLE_RANGE` - `U+0020`–`U+007E`
- `PUA_RANGE` - `U+E000`–`U+F8FF`
- `SUPPLEMENTARY_PUA_RANGE` - `U+F0001`–`U+10FFFF`

## Codepoint Map Types

Newtype wrappers around `HashMap` for type-safe lookups:

- **`CodePointCategoryMap`** - `HashMap<CodePoint, String>`: codepoint →
  category (e.g. from `emoji-test.txt`)
- **`CodePointKeywordMap`** - `HashMap<CodePoint, Vec<String>>`: codepoint →
  keyword list (e.g. from CLDR annotations)
- **`CodePointNameMap`** - `HashMap<CodePoint, String>`: codepoint →
  canonical name (e.g. from CLDR `tts` fields)
- **`GlyphNameMap`** - `HashMap<String, CodePoint>`: glyph name →
  codepoint (e.g. from SMuFL `glyphnames.json`). Implements `Deserialize`
  for the SMuFL JSON format `{ "glyphName": { "codepoint": "U+E050", ... } }`.

## `GlyphEntry<N>`

Metadata for a single exported glyph:

```rust
pub struct GlyphEntry<N: AsRef<str>> {
    pub code: Option<CodePoint>,
    pub name: N,
    pub file: PathBuf,
    pub resource_path: ResourcePath,
}
```

- `GlyphEntry::new(code, name, gresource_prefix, icons_context)` -
  constructs `file` and `resource_path` automatically from prefix, context,
  and name
- Generic over `N`: `GlyphName<F>` for type-safe families, `String` for
  the `ExportConfig` pipeline, `IconName` for Nerd Fonts

## `GlyphName<F>`

Phantom-typed glyph name newtype:

```rust
pub struct GlyphName<F: FontFamily> {
    name: String,
    _marker: PhantomData<F>,
}
```

Implements `AsRef<str>`, `Display`, `Serialize`, `Deserialize`, `Hash`, `Eq`,
`Ord`. Construction via `GlyphName::new(String)` after normalization.

Each font family crate defines a type alias:

```rust
pub type DotoName = GlyphName<Doto>;
```

## `FontFile`

A font file name without extension, e.g. `"DSEG7Classic-Regular"`, `"Doto"`.
Wraps a `&'static str` for type-safe font file references in build scripts.

## `ResourcePath`

A GResource resource path (e.g. `/io/smearor/fonts/doto/scalable/glyphs/doto-a.svg`).
Used by `GlyphEntry` to store the GResource URI for each exported glyph.
