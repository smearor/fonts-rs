# fonts-rs-generator

Generic build-time pipeline for font glyph export, SVG generation, and
GResource packaging. Used as a build-dependency by every font family crate.

## Overview

This crate provides the build-time tooling for the modular fonts framework:

- **`ExportConfig<X>`** - Runtime configuration for glyph export, decoupled
  from `FontDefinition`, supporting variant-specific parameters
- **`FontBuild`** - Builder for `build.rs` pipeline with hash-based change
  detection, conditional export, GResource compilation, and code generation
- **`VariantList<X>`** - Typed list of font variants with Cargo feature
  detection via `detect_and_get_active_variant()`
- **`impl_font_loader!`** - Macro for cached font loading from embedded bytes
  or disk
- **`MetadataGenerator`** - Trait for build-time `phf::Map` metadata table
  generation (keywords, categories, aliases)
- **`write_variant_info()`** - Writes `OUT_DIR/variant.rs` with GResource
  and glyph prefix constants
- **`build_constants`** - Common path constants for build scripts

## Usage

```toml
[build-dependencies]
fonts-rs-generator = "0.1"
fonts-rs-model = "0.1"
```

See the [Build Patterns](../book/src/build-patterns.md) documentation for
complete `build.rs` examples.

## License

MIT
