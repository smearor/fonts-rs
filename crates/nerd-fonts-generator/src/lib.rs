//! Build-time code generation and icon export for Nerd Fonts.
//!
//! Provides:
//! - [`export::export_icons()`] for generating SVG icons from TTF/OTF fonts
//! - [`NerdFontsDefinition`] implementing the generic [`FontDefinition`] trait
//! - Generator traits for producing Rust constants, phf::Map tables, and CSS
//! - Metadata parsers for upstream icon sets (Font Awesome, Material Design, etc.)
//!
//! Generic pipeline types are re-exported from `fonts-rs-generator`.

pub mod definition;
pub mod export;
pub mod font;
pub mod generator;
pub mod gresource;
pub mod svg;

pub use definition::NerdFonts;
pub use definition::NerdFontsDefinition;
pub use export::export_icons;

// Re-export generic types from the framework for backward compatibility.
pub use fonts_rs_generator::EMPTY_SVG;
pub use fonts_rs_generator::Font;
pub use fonts_rs_generator::FontDefinition;
pub use fonts_rs_generator::GlyphGenerator;
pub use fonts_rs_generator::SvgPathBuilder;
pub use fonts_rs_generator::export_glyphs;
pub use fonts_rs_generator::generate_gresource_xml;
pub use fonts_rs_generator::normalize_to_kebab;

pub use generator::codemap::IconsCodemapGenerator;
pub use generator::css::WebCssGenerator;
pub use generator::error::GenerateError;
pub use generator::generate::NerdFontsGenerator;
pub use generator::icons::IconsRustGenerator;
pub use generator::metadata::IconsMetadataGenerator;
