# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-09-14

### Changed

- **Build artifacts moved to `OUT_DIR`**: all generated files (SVGs, `metadata.json`, `icons.gresource.xml`, `.font-hash`, `nerdfont.css`) are now written to Cargo's `OUT_DIR` instead of the source `resources/` directory, keeping the working tree clean during `cargo publish`
- **`build_constants.rs`**: replaced static path constants (`METADATA_PATH`, `HASH_PATH`, `ICONS_GRESOURCE_XML`) with `OUT_DIR`-based functions (`out_dir()`, `metadata_path()`, `hash_path()`, `icons_gresource_xml()`)
- **`nerd-fonts-rs/build.rs`**: sets `NERDFONT_CSS_PATH` environment variable for `include_str!(env!(...))` in `web.rs` instead of hardcoding `resources/nerdfont.css`
- **`nerd-fonts-rs/src/web.rs`**: uses `include_str!(env!("NERDFONT_CSS_PATH"))` to load generated CSS from `OUT_DIR`
- **`fonts-rs-noto-emoji/build.rs`**: uses `build_constants::metadata_path()` instead of static path constant
- **`WebCssGenerator`**: writes `nerdfont.css` to `OUT_DIR` instead of `resources/`

### Removed

- **`resources/icons.gresource.xml`**: removed from all 12 crate `Cargo.toml` include lists (generated file, no longer in source tree)
- **`.gitignore` entries**: removed obsolete ignore rules for generated artifacts that now live in `OUT_DIR`
- **`--allow-dirty --no-verify` flags**: removed from all `cargo publish` commands in `release.yml` (no longer needed since build scripts write only to `OUT_DIR`)

## [0.2.0] - 2026-09-14

### Added

- **Modular fonts framework**: generic `fonts-rs-model` and `fonts-rs-generator` crates providing a font-family-agnostic build pipeline for glyph export, SVG generation, GResource packaging, and code generation
- **`FontFamily` sealed marker trait**: compile-time font family identity via phantom typing in `GlyphName<F>`
- **`FontFamilyConfig` trait**: compile-time constants for GResource prefixes, icon contexts, codepoint ranges, and font family display names
- **`FontDefinition` trait**: extends `FontFamilyConfig` with type-safe glyph name normalization (`normalize_name`), glyph filtering (`should_skip`), and default `export_glyphs` pipeline
- **`ExportConfig<X>`**: runtime export configuration generic over `FontFamilyConfig`, decoupled from `FontDefinition`, supporting variant-specific parameters, `name_filter`, and two export methods (`export_glyphs` and `export_glyphs_by_name_map`)
- **`FontBuild` builder**: hash-based change detection (FNV-1a), conditional export, GResource compilation, code generation (`codemap.rs`, `icons.rs`, `variant.rs`), and custom codegen via `run_with()`
- **`VariantList<X>`**: typed list of font variants with `detect_and_get_active_variant()` for Cargo feature-based variant selection
- **`impl_font_loader!` macro**: cached font loading from embedded bytes or disk via `OnceLock`
- **`MetadataGenerator` trait**: build-time `phf::Map` metadata table generation for keywords, categories, and aliases with deduplication support
- **`CodePointRange`**: inclusive Unicode range type for reverse cmap probing with predefined constants (`BMP_RANGE`, `ASCII_PRINTABLE_RANGE`, `PUA_RANGE`, `SUPPLEMENTARY_PUA_RANGE`)
- **Typed codepoint maps**: `CodePointCategoryMap`, `CodePointKeywordMap`, `CodePointNameMap`, `GlyphNameMap` for type-safe metadata lookups
- **`GlyphEntry<N>`**: generic struct for exported glyph metadata with automatic `file` and `resource_path` construction
- **`GlyphName<F>`**: phantom-typed newtype preventing glyph name mix-ups between font families at compile time
- **`FontVariant` / `FontVariantType`**: font variant identification and realization (separate file, variable font axes, or both)
- **`Axis`, `AxisValue`, `AxisValues`**: variable font axis identification and values, `const`-compatible for `VARIANTS` arrays
- **`FontFile` / `ResourcePath`**: font file path resolution and GResource path types
- **11 new font family crates**: `fonts-rs-doto` (variable, 25 variants), `fonts-rs-bravura` (SMuFL music notation), `fonts-rs-seven-segment` (24 variants), `fonts-rs-fourteen-segment` (24 variants), `fonts-rs-barcode-code39`, `fonts-rs-barcode-code128`, `fonts-rs-barcode-ean13`, `fonts-rs-redacted` (4 variants), `fonts-rs-dicefont`, `fonts-rs-cuernavaca` (chess), `fonts-rs-noto-emoji` (with CLDR metadata)
- **`fonts-rs-noto-emoji-generator`**: build-time code generation for Noto Emoji including CLDR annotation parsing
- **`fonts-rs-noto-emoji-cheat-sheet`**: GTK4 application to browse Noto Emoji with search
- **Per-crate README.md**: README files for all 19 workspace crates
- **Book documentation**: new pages for `fonts-rs-model` API, `fonts-rs-generator` API, build patterns, and "Adding a Font Family" guide
- **Book overhaul**: updated `introduction.md`, `architecture.md`, `getting-started.md`, `icon-resolution.md` (renamed to "Glyph Resolution"), `icon-export.md` (renamed to "Glyph Export"), `font-loading.md`, `gtk-integration.md`, and `SUMMARY.md` to reflect the modular framework
- **Root README.md overhaul**: workspace overview with crate tables, architecture diagram, and dual quick start examples
- **Examples**: `dot_matrix_marquee` (Doto), `wiener_blut` (Bravura), `interactive_demo` (Seven-Segment, Fourteen-Segment, Barcode), `dice_roller` (DiceFont), `chess_board` (Cuernavaca)

### Changed

- **Workspace expanded from 4 to 19 crates**: modular architecture with generic framework, font family crates, Nerd Fonts integration, and application crates
- **`nerd-fonts-rs` refactored**: now uses `fonts-rs-model` and `fonts-rs-generator` mechanics, new `font_loader.rs` module, removed `css/version.rs`
- **`nerd-fonts-generator` refactored**: delegates to `fonts-rs-generator` for SVG export, codepoint maps, and GResource generation; removed redundant `export.rs`, `font/reverse_codepoint_map.rs`, `generator/generate.rs`, `gresource/generate.rs`, `svg/export.rs`, `svg/path_builder.rs`
- **`nerd-fonts-model` refactored**: uses `fonts-rs-model` types (`GlyphName`, `GlyphEntry`), removed `resource_path.rs` and `entry.rs`
- **`ExportConfig` constructors**: replaced free function `build_config` with `ExportConfig::new()` and `ExportConfig::with_variant()`
- **`GlyphEntry` constructor**: added `GlyphEntry::new()` for automatic `file` and `resource_path` construction
- **Error handling**: dedicated `ExportError` type for export failures, nicer error propagation instead of build-time panics
- **Paths instead of `&str`**: build pipeline methods now use `Path` / `PathBuf` instead of string slices
- **Cleanup strategy**: improved cleanup for fonts with variants (hash-based change detection with `extra_hash`)
- **Generated files split**: code generation now produces separate `codemap.rs`, `icons.rs`, and `variant.rs` files instead of a single file
- **Abstracted font family metadata**: `FONT_FAMILY_NAME`, `GRESOURCE_PREFIX`, `ICONS_CONTEXT`, and `CODEPOINT_RANGES` as `FontFamilyConfig` associated constants
- **Variant selection fallback**: `VariantList::detect_active_index` now falls back to the default variant when multiple variant features are active (e.g. `--all-features` in CI) instead of erroring
- **Example binaries renamed**: all `interactive_demo.rs` examples renamed to unique per-crate names (`seven_segment_demo`, `fourteen_segment_demo`, `barcode_code39_demo`, etc.)
- **Generator examples renamed**: `generate.rs` examples renamed to `generate_nerd_fonts.rs` and `generate_noto_emoji.rs` to avoid output filename collisions

### Removed

- **`css/version.rs`**: removed from `nerd-fonts-rs` (GTK version targeting now handled differently)
- **Duplicate code**: removed redundant export, font, gresource, and SVG modules from `nerd-fonts-generator` (now delegated to `fonts-rs-generator`)

## [0.1.0] - 2026-09-11

### Added

- **Repository rename**: project renamed from `nerd-fonts-gtk` to `fonts-rs` with updated repository URL (`https://github.com/smearor/fonts-rs`), App IDs (`io.smearor.fonts_rs.*`), log messages, and doc comments
- **CI feature coverage**: `--all-features` flag added to `cargo clippy`, `cargo test`, `cargo build`, `cargo doc`, and doctest jobs to test all feature gates (`v4_12`, `render`, `web`, `embed-fonts`, `metadata`)
- **Workspace split into 4 crates**: `nerd-fonts-model` (shared data types), `nerd-fonts-generator` (build-time code generation and icon export), `nerd-fonts-rs` (GTK4 integration library), and `nerd-fonts-cheat-sheet` (browseable icon application)
- **Metadata resolution system**: keyword and category lookup for Nerd Font icons, sourced from upstream icon set metadata (Font Awesome, Material Design, Devicon, Octicons) via a build-time `phf::Map` code generation pipeline
- **Cheat sheet application** (`nerd-fonts-cheat-sheet`): GTK4 app to browse all Nerd Font icons with search filtering (names, keywords, categories, aliases), icon detail sidebar, navigation history, and SVG preview
- **SVG icon export**: `skrifa`-based glyph outline extraction producing GTK4 symbolic SVG icons, with `SvgPathBuilder` implementing `OutlinePen` for Bezier-to-SVG-path conversion
- **Reverse codepoint map**: font glyph-to-codepoint resolution by probing BMP and supplementary PUA ranges
- **GResource XML generation**: automatic `icons.gresource.xml` manifest creation for GTK4 icon bundles
- **CSS version targeting**: `GtkVersion` enum and GTK 4.10+ compatibility CSS generation (`css/version.rs`)
- **`InitOptions`**: configurable initialization with GTK version targeting
- **Book documentation**: new `icon-export.md` page covering the export pipeline, caching, CLI, and library API
- **Vendored metadata resources**: bundled `devicon.json`, `fontawesome/icons.yml`, `fontawesome/categories.yml`, `materialdesign-icons.json`, and `octicons-keywords.json`

### Security

- **RUSTSEC-2026-0192**: replaced direct dependency on unmaintained `ttf-parser` crate with `skrifa` (Google Fonts fontations project) for font parsing, glyph outline extraction, and codepoint mapping. `skrifa` is actively maintained, `#![forbid(unsafe_code)]`, and provides a richer metadata API via `MetadataProvider` trait

### Changed

- **MSRV bumped to 1.92**: required by gtk4 0.11.x / glib 0.22.x ecosystem
- **Workspace architecture**: migrated from single-crate `nerd-fonts-gtk` package to a 4-crate Cargo workspace with centralized dependency management via `[workspace.dependencies]`
- **Feature flags**: restructured features (`gtk`, `render`, `metadata`, `web`, `embed-fonts`, `export`) across crate boundaries
- **Resource paths**: updated GResource prefix from `/io/nerd_fonts/icons/` to `/io/smearor/nerd_fonts/icons/`
- **Code quality**: panic-free error handling throughout (`Result` types, `thiserror`/`miette`), removed `unwrap()`/`expect()` from production paths
- **Documentation**: updated `architecture.md`, `css-generation.md`, `gtk-integration.md`, `icon-resolution.md`, and `introduction.md` to reflect new crate structure
- **Feature flag documentation**: added missing `v4_12` and `metadata` features to README and book; updated `render` feature description from "software rendering" to "font loading" after `pixel-drawing` extraction
- **GTK integration docs**: removed obsolete `Color` struct, `apply_icon_color`, and `apply_text_color` documentation from `gtk-integration.md` (moved to `pixel-drawing`)
- **Icon export docs**: updated `icon-export.md` to reference `nerd-fonts-generator` crate instead of non-existent `export` feature

### Removed

- **`software-rendering.md`**: removed from book (content lives in `pixel-drawing` book); removed from `SUMMARY.md`

### Fixed

- GTK 4.10 CSS compatibility issues addressed with version-targeted CSS generation

### Distribution

- **Per-crate publishing**: release workflow now publishes each crate individually in dependency order (`nerd-fonts-model` → `nerd-fonts-generator` → `nerd-fonts-rs` → `nerd-fonts-cheat-sheet`)
- **Dependabot**: added per-crate Cargo ecosystem entries for all 4 workspace members

### Infrastructure

- **CI workflows**: updated all workflows (`build`, `test`, `clippy`, `docs`, `msrv`, `audit`) to use `--workspace` flag and `crates/**/*.rs` path filters
- **Workspace lints**: added `unexpected_cfgs` lint configuration at workspace level
