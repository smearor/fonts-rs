//! Code generators for build-time artifact generation.
//!
//! Provides implementations of the generic [`GlyphGenerator`] trait for
//! generating Rust constants, phf::Map codepoint tables, and web CSS from
//! icon metadata.

pub mod codemap;
pub mod css;
pub mod icons;
pub mod metadata;

pub use codemap::IconsCodemapGenerator;
pub use css::WebCssGenerator;
pub use icons::IconsRustGenerator;
pub use metadata::IconsMetadataGenerator;

pub use nerd_fonts_model::IconName;
