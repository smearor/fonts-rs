# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-09-11

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
