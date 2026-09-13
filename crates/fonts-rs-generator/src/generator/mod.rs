//! Generic code generators for build-time artifact generation.
//!
//! Provides a unified [`GlyphGenerator`] trait and implementations for
//! generating Rust constants and phf::Map codepoint tables from glyph
//! metadata.

pub mod codemap;
pub mod error;
pub mod generate;
pub mod metadata_generator;
pub mod rust_constants;

pub use codemap::CodemapGenerator;
pub use error::GenerateError;
pub use generate::GlyphGenerator;
pub use metadata_generator::MetadataGenerator;
pub use metadata_generator::escape_str;
pub use rust_constants::RustConstantsGenerator;
