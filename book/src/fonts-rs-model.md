# `fonts-rs-model` - Generic Model Types

The `fonts-rs-model` crate provides font-family-agnostic types shared across
all `fonts-rs-*` crates. These types are the foundation of the modular fonts
framework, enabling any font family crate to use the same build pipeline
without Nerd Fonts dependencies.

## `FontFamilyMarker` (Marker Trait)

A sealed marker trait that identifies each font family at compile time.
Each font family crate defines a zero-sized enum implementing
`FontFamilyMarker` and the private `Sealed` trait. This prevents external
crates from defining their own font family markers.

```rust
pub trait FontFamilyMarker: sealed::Sealed {}
```

Used as the phantom type parameter in `GlyphName<F>`, so that
`GlyphName<Doto>` and `GlyphName<SevenSegment>` are distinct types at
compile time - preventing mix-ups between font families.

### Example

```rust
use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::sealed;

pub enum Doto {}

impl FontFamilyMarker for Doto {}
impl sealed::Sealed for Doto {}
```

## `FontFamily` (Core Trait)

The runtime glyph API implemented by each font family marker type -
typically via `impl_font_family!` in `family.rs`:

```rust
pub trait FontFamily: FontFamilyMarker + FontFamilyConfig {
    type Name: AsRef<str> + FromStr;
    const VARIANTS: &'static [FontVariant] = &[];

    fn glyph_name(ch: impl Into<CodePoint>) -> Option<Self::Name>;
    fn codepoint(name: impl AsRef<str>) -> Option<CodePoint>;
    fn all_glyphs() -> impl Iterator<Item = (CodePoint, Self::Name)>;
    fn glyph_count() -> usize { Self::all_glyphs().count() }
}
```

The lookup methods are generic: `codepoint` accepts `&str`, `String`,
and the typed `Self::Name` (`GlyphName<F>` / `IconName`) via `AsRef<str>`;
`glyph_name` accepts `char` and `CodePoint` via `Into<CodePoint>`.
`all_glyphs` yields the typed `Self::Name` — use `name.as_ref()` when
plain `&str` access is enough.

The macro also emits inherent forwarders, so callers write
`Doto::codepoint("doto-a")` without importing the trait.

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
    const ICON_PREFIX: &'static str = "";

    fn icons_dir(output_dir: &Path) -> PathBuf { ... }
    fn prepare_icons_dir(output_dir: &Path) -> std::io::Result<PathBuf> { ... }
    fn icons_resource_prefix() -> String { ... }
    fn is_valid_icon_name(name: &str) -> bool { ... }
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
- **`ICON_PREFIX`**: Icon name prefix for icon fonts (e.g. `"nf-"` for
  Nerd Fonts, `"tabler-"` for Tabler). Empty for non-icon families.

### Methods

- **`prepare_icons_dir`**: Removes the `scalable/{context}/` directory and
  recreates it (avoids stale SVGs from previous builds)
- **`icons_dir`**: Returns the path to `scalable/{context}/` within the
  output directory
- **`icons_resource_prefix`**: Returns
  `{GRESOURCE_PREFIX}/scalable/{ICONS_CONTEXT}` — the canonical resource
  layout for icon-theme glyphs. Every family ships SVGs at
  `{prefix}/scalable/{context}/{name}.svg`; override only when upstream
  assets demand a different layout.
- **`is_valid_icon_name`**: Validation hook used by `IconName<F>` —
  default accepts names starting with `ICON_PREFIX`; multi-set fonts
  override it (Nerd Fonts delegates to `IconSet` prefix detection).

## `FontDefinition` (Trait)

Extends `FontFamilyConfig` with type-safe glyph name processing and a
default `export_glyphs` pipeline. Used by font families with semantic
PostScript glyph names (e.g. Barcode, Seven-Segment).

```rust
pub trait FontDefinition: FontFamilyConfig {
    type Name: AsRef<str> + Clone + Ord + Serialize + for<'a> Deserialize<'a>;
    type Family: FontFamilyMarker;

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
pub struct GlyphName<F: FontFamilyMarker> {
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

## `IconName<F>`

Phantom-typed GTK icon name for icon font families — the icon-font
counterpart of `GlyphName<F>`:

```rust
pub struct IconName<F: FontFamilyConfig> {
    name: String,
    _marker: PhantomData<F>,
}
```

Normalizes raw glyph names to the canonical `{ICON_PREFIX}{kebab}-symbolic`
form (lowercase, `_` → `-`, `-symbolic` suffix) and validates via
`F::is_valid_icon_name`. Implements `AsRef<str>`, `Display`, `FromStr`,
`Serialize`, `Deserialize`, `Hash`, `Eq`, `Ord` — all without bounds on `F`.

```rust
// nerd-fonts-model
pub type IconName = fonts_rs_model::IconName<NerdFontsConfig>;
```

Key methods: `IconName::parse` (accepts `nf-fa-x`, `nf-fa-x-symbolic`,
`fa-x`), `from_glyph_name`, `css_class_name` (name without `-symbolic`),
`icon_set` (delegates to `F: IconSetDetect` when implemented).

## `IconSetDetect` (Trait)

Optional capability for multi-set icon fonts — detects which icon set a
name belongs to:

```rust
pub trait IconSetDetect {
    type IconSet;
    fn detect_icon_set(icon_name: &str) -> Self::IconSet;
}
```

Nerd Fonts implements it with its `IconSet` enum; single-set icon fonts
simply don't implement it. `IconName::icon_set` is only available when
`F: IconSetDetect`.

## Icon CSS helpers

- **`icon_css::<F>()`** (`fonts-rs-model`, `gtk` feature): generates the
  GTK CSS class rule `.{ICON_PREFIX}* { font-family: ... }` used by
  family init paths.
- **`load_css(css)`** (`fonts-rs-model`, `gtk` feature): loads a
  `CssProvider` onto the default display at application priority.
- **`web_icon_css_rules(icons, class_prefix)`** (`fonts-rs-generator`):
  emits per-icon `.prefix-name:before { content: "\xxxx" }` rules for
  web CSS generators.

## `FontFile`

A font file name without extension, e.g. `"DSEG7Classic-Regular"`, `"Doto"`.
Wraps a `&'static str` for type-safe font file references in build scripts.

## `ResourcePath`

A GResource resource path (e.g. `/io/smearor/fonts/doto/scalable/glyphs/doto-a.svg`).
Used by `GlyphEntry` to store the GResource URI for each exported glyph.
