# fonts-rs-model

Generic model types for font glyph packaging: `CodePoint`, `ResourcePath`,
`GlyphName`, `GlyphEntry`, and related types shared across all `fonts-rs-*`
crates.

## Overview

This crate provides the foundation types used by the modular fonts framework:

- **`FontFamily`** - Sealed marker trait for compile-time font family identity
- **`FontFamilyConfig`** - Trait defining compile-time constants for font
  family configuration (GResource prefixes, icon contexts, codepoint ranges)
- **`FontDefinition`** - Trait extending `FontFamilyConfig` for type-safe
  glyph name processing and default `export_glyphs` pipeline
- **`FontVariant`** / **`FontVariantType`** - Font variant identification and
  realization (separate file, variable font axes, or both)
- **`Axis`**, **`AxisValue`**, **`AxisValues`** - Variable font axis types
- **`CodePointRange`** - Inclusive Unicode range for reverse cmap probing
- **`GlyphEntry<N>`** - Metadata for an exported glyph
- **`GlyphName<F>`** - Phantom-typed newtype for type-safe glyph names
- Codepoint map types: `CodePointCategoryMap`, `CodePointKeywordMap`,
  `CodePointNameMap`, `GlyphNameMap`

## Usage

This crate is typically used as a dependency of `fonts-rs-generator` and font
family crates. You rarely depend on it directly.

```toml
[dependencies]
fonts-rs-model = "0.1"
```

## License

MIT
