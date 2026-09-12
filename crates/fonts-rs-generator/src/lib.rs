//! Generic build-time pipeline for font glyph export.
//!
//! Provides font-agnostic glyph extraction from TTF/OTF fonts, SVG icon
//! generation, reverse codepoint map construction, GResource XML generation,
//! and generic code generators — all parameterized by the [`FontDefinition`]
//! trait.
//!
//! Each font family crate implements [`FontDefinition`] to plug into the
//! generic [`FontDefinition::export_glyphs`] pipeline.

pub mod export_config;
pub mod font;
pub mod font_definition;
pub mod font_loader;
pub mod generator;
pub mod gresource;
pub mod svg;

pub use export_config::ExportConfig;
pub use export_config::export_glyphs_with_config;
pub use font::Font;
pub use font_definition::FontDefinition;
pub use font_definition::normalize_to_kebab;
pub use generator::codemap::CodemapGenerator;
pub use generator::error::GenerateError;
pub use generator::generate::GlyphGenerator;
pub use generator::rust_constants::RustConstantsGenerator;
pub use svg::EMPTY_SVG;
pub use svg::SvgPathBuilder;
