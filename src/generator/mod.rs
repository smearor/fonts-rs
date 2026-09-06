//! Code generators for build-time artifact generation.
//!
//! Provides a unified [`NerdFontsGenerator`] trait and implementations for
//! generating Rust constants, phf::Map codepoint tables, and web CSS from
//! icon metadata.

pub mod codemap;
pub mod css;
pub mod error;
pub mod generate;
pub mod icons;

pub use codemap::IconsCodemapGenerator;
pub use css::WebCssGenerator;
pub use error::GenerateError;
pub use generate::NerdFontsGenerator;
pub use icons::IconsRustGenerator;

pub use crate::icons::entry::IconEntry;
