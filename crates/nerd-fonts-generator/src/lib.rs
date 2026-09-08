//! Build-time code generation and icon export for Nerd Fonts.
//!
//! Provides:
//! - [`export::export_icons()`] for generating SVG icons from TTF/OTF fonts
//! - Generator traits for producing Rust constants, phf::Map tables, and CSS
//! - Metadata parsers for upstream icon sets (Font Awesome, Material Design, etc.)

pub mod export;
pub mod generator;
pub mod svg_path_builder;

pub use export::export_icons;
pub use generator::codemap::IconsCodemapGenerator;
pub use generator::css::WebCssGenerator;
pub use generator::error::GenerateError;
pub use generator::generate::NerdFontsGenerator;
pub use generator::icons::IconsRustGenerator;
pub use generator::metadata::IconsMetadataGenerator;
