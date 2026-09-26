//! Metadata newtypes and registry trait for font glyph search.
//!
//! Provides [`GlyphKeywordMap`], [`GlyphCategoryMap`], and [`GlyphAliasMap`] -
//! newtype wrappers around `phf::Map` that enable idiomatic access (Deref,
//! iteration, len, get).
//!
//! The [`GlyphMetadata`] trait unifies per-font-family metadata access:
//! each font crate implements it with its build-time generated static maps.

mod alias_map;
mod category_map;
mod glyph_group;
mod glyph_metadata;
mod glyph_ordering;
mod group_label;
mod keyword_map;

pub use alias_map::GlyphAliasMap;
pub use category_map::GlyphCategoryMap;
pub use glyph_group::GlyphGroup;
pub use glyph_metadata::GlyphMetadata;
pub use glyph_ordering::GlyphOrdering;
pub use group_label::GroupLabel;
pub use keyword_map::GlyphKeywordMap;
