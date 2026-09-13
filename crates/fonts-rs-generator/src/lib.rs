//! Generic build-time pipeline for font glyph export.
//!
//! Provides font-agnostic glyph extraction from TTF/OTF fonts, SVG icon
//! generation, reverse codepoint map construction, GResource XML generation,
//! and generic code generators — all parameterized by the [`FontDefinition`]
//! trait.
//!
//! Each font family crate implements [`FontDefinition`] to plug into the
//! generic [`FontDefinition::export_glyphs`] pipeline.

pub mod build_constants;
pub mod build_helpers;
pub mod build_runner;
pub mod export_config;
pub mod export_error;
pub mod font;
pub mod font_definition;
pub mod font_family_config;
pub mod font_loader;
pub mod font_variant_ext;
pub mod generator;
pub mod gresource;
pub mod svg;
pub mod variant_list;

pub use build_constants::hash_font_file;
pub use build_helpers::set_font_path_env;
pub use build_runner::FontBuild;
pub use export_config::ExportConfig;
pub use export_error::ExportError;
pub use font::Font;
pub use font_definition::FontDefinition;
pub use font_definition::normalize_to_kebab;
pub use font_family_config::FontFamilyConfig;
pub use font_variant_ext::FontVariantExt;
pub use generator::codemap::CodemapGenerator;
pub use generator::error::GenerateError;
pub use generator::generate::GlyphGenerator;
pub use generator::metadata_generator::MetadataGenerator;
pub use generator::metadata_generator::escape_str;
pub use generator::rust_constants::RustConstantsGenerator;
pub use svg::EMPTY_SVG;
pub use svg::Svg;
pub use svg::SvgPathBuilder;
pub use variant_list::VariantList;
