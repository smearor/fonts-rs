//! Generic build-time pipeline for font glyph export.
//!
//! Provides font-agnostic glyph extraction from TTF/OTF fonts, SVG icon
//! generation, reverse codepoint map construction, GResource XML generation,
//! and generic code generators — all parameterized by the [`FontDefinition`]
//! trait.
//!
//! Each font family crate implements [`FontDefinition`] to plug into the
//! generic [`export_glyphs`] pipeline.

pub mod export;
pub mod font;
pub mod font_definition;
pub mod generator;
pub mod gresource;
pub mod svg;

pub use export::export_glyphs;
pub use font::Font;
pub use font_definition::FontDefinition;
pub use font_definition::normalize_to_kebab;
pub use generator::codemap::CodemapGenerator;
pub use generator::error::GenerateError;
pub use generator::generate::GlyphGenerator;
pub use generator::rust_constants::RustConstantsGenerator;
pub use gresource::generate_gresource_xml;
pub use svg::EMPTY_SVG;
pub use svg::SvgPathBuilder;
