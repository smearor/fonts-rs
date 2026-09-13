//! Generic model types for font glyph packaging.
//!
//! Provides font-family-agnostic types shared across all `fonts-rs-*` crates:
//! [`CodePoint`], [`ResourcePath`], [`FontFamily`] marker trait,
//! [`GlyphName`] phantom-typed newtype, and [`GlyphEntry`] generic metadata.
//!
//! These types are extracted from `nerd-fonts-model` and generalized so
//! that any font family crate can use them without Nerd Font dependencies.

pub mod category;
pub mod codepoint;
pub mod font_family;
pub mod glyph_entry;
pub mod glyph_name;
pub mod keyword;
pub mod paths;
pub mod resource_path;

pub use category::GlyphCategory;
pub use codepoint::CodePoint;
pub use codepoint::CodePointParseError;
pub use font_family::FontFamily;
pub use font_family::sealed;
pub use glyph_entry::GlyphEntry;
pub use glyph_name::GlyphName;
pub use keyword::GlyphKeyword;
pub use paths::GRESOURCE_BASE_PREFIX;
pub use resource_path::ResourcePath;
