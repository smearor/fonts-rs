//! Icon metadata: keywords, categories, and aliases for search functionality.
//!
//! Provides keyword and category lookup for Tabler icons, enabling
//! search and filtering beyond simple name matching.
//!
//! Keywords are sourced from upstream `icons.json` (`tags` field).
//! Categories are sourced from upstream `icons.json` (`category` field).
//! Aliases are sourced from upstream `aliases.json` for the active style.
//!
//! This module is only available when the `metadata` feature is enabled.

use fonts_rs_model::GlyphAliasMap;
use fonts_rs_model::GlyphCategory;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));

/// Metadata registry for the Tabler Icons font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from upstream `icons.json` tags/categories and `aliases.json`.
pub struct TablerMetadata;

impl GlyphMetadata for TablerMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }

    fn alias_map(&self) -> &GlyphAliasMap {
        &ALIASES
    }
}
