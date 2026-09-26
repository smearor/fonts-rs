//! Icon metadata: keywords and aliases for search functionality.
//!
//! Provides keyword and alias lookup for Heroicons icons, enabling
//! search beyond simple name matching - e.g. "stars" finds
//! `heroicons-sparkles-symbolic`.
//!
//! Data is sourced from the heroicons.com site bundle (keyword map,
//! vendored as `resources/metadata/keywords.json`) and the iconify
//! heroicons set's v1→v2 alias table (vendored as
//! `resources/metadata/aliases.json`). Heroicons ships no categories,
//! so the category map is empty.
//!
//! This module is only available when the `metadata` feature is enabled.

use fonts_rs_model::GlyphAliasMap;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));

/// Metadata registry for the Heroicons font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from the vendored keyword and alias tables.
pub struct HeroiconsMetadata;

impl GlyphMetadata for HeroiconsMetadata {
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
