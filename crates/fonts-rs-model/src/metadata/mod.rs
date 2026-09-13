//! Metadata newtypes and registry trait for font glyph search.
//!
//! Provides [`GlyphKeywordMap`], [`GlyphCategoryMap`], and [`GlyphAliasMap`] —
//! newtype wrappers around `phf::Map` that enable idiomatic access (Deref,
//! iteration, len, get).
//!
//! The [`GlyphMetadata`] trait unifies per-font-family metadata access:
//! each font crate implements it with its build-time generated static maps.

mod alias_map;
mod category_map;
mod glyph_metadata;
mod keyword_map;

pub use alias_map::GlyphAliasMap;
pub use category_map::GlyphCategoryMap;
pub use glyph_metadata::GlyphMetadata;
pub use keyword_map::GlyphKeywordMap;
