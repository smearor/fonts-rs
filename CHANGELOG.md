# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-09-11

### Added

- **Workspace split into 4 crates**: `nerd-fonts-model` (shared data types), `nerd-fonts-generator` (build-time code generation and icon export), `nerd-fonts-rs` (GTK4 integration library), and `nerd-fonts-cheat-sheet` (browseable icon application)
- **Metadata resolution system**: keyword and category lookup for Nerd Font icons, sourced from upstream icon set metadata (Font Awesome, Material Design, Devicon, Octicons) via a build-time `phf::Map` code generation pipeline
- **Cheat sheet application** (`nerd-fonts-cheat-sheet`): GTK4 app to browse all Nerd Font icons with search filtering (names, keywords, categories, aliases), icon detail sidebar, navigation history, and SVG preview
- **SVG icon export**: `ttf-parser`-based glyph outline extraction producing GTK4 symbolic SVG icons, with `SvgPathBuilder` implementing `OutlineBuilder` for Bezier-to-SVG-path conversion
- **Reverse codepoint map**: font glyph-to-codepoint resolution by probing BMP and supplementary PUA ranges
- **GResource XML generation**: automatic `icons.gresource.xml` manifest creation for GTK4 icon bundles
- **CSS version targeting**: `GtkVersion` enum and GTK 4.10+ compatibility CSS generation (`css/version.rs`)
- **`InitOptions`**: configurable initialization with GTK version targeting
- **Book documentation**: new `icon-export.md` page covering the export pipeline, caching, CLI, and library API
- **Vendored metadata resources**: bundled `devicon.json`, `fontawesome/icons.yml`, `fontawesome/categories.yml`, `materialdesign-icons.json`, and `octicons-keywords.json`

### Changed

- **Workspace architecture**: migrated from single-crate `nerd-fonts-gtk` package to a 4-crate Cargo workspace with centralized dependency management via `[workspace.dependencies]`
- **Feature flags**: restructured features (`gtk`, `render`, `metadata`, `web`, `embed-fonts`, `export`) across crate boundaries
- **Resource paths**: updated GResource prefix from `/io/nerd_fonts/icons/` to `/io/smearor/nerd_fonts/icons/`
- **Code quality**: panic-free error handling throughout (`Result` types, `thiserror`/`miette`), removed `unwrap()`/`expect()` from production paths
- **Documentation**: updated `architecture.md`, `css-generation.md`, `gtk-integration.md`, `icon-resolution.md`, and `introduction.md` to reflect new crate structure

### Fixed

- GTK 4.10 CSS compatibility issues addressed with version-targeted CSS generation

### Distribution

- **Per-crate publishing**: release workflow now publishes each crate individually in dependency order (`nerd-fonts-model` → `nerd-fonts-generator` → `nerd-fonts-rs` → `nerd-fonts-cheat-sheet`)
- **Dependabot**: added per-crate Cargo ecosystem entries for all 4 workspace members

### Infrastructure

- **CI workflows**: updated all workflows (`build`, `test`, `clippy`, `docs`, `msrv`, `audit`) to use `--workspace` flag and `crates/**/*.rs` path filters
- **Workspace lints**: added `unexpected_cfgs` lint configuration at workspace level
