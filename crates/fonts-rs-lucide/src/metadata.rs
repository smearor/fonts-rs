//! Icon metadata: keywords and categories for search functionality.
//!
//! Provides keyword and category lookup for Lucide icons, enabling
//! search beyond simple name matching — e.g. "font size" finds
//! `lucide-a-arrow-down-symbolic`.
//!
//! Data is sourced from upstream per-icon JSON files (`icons/{name}.json`
//! `tags`/`categories` fields), vendored merged as
//! `resources/metadata/icons.json`. Lucide ships no aliases, so the
//! alias map is empty.
//!
//! This module is only available when the `metadata` feature is enabled.

use fonts_rs_model::GlyphCategory;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));

/// Metadata registry for the Lucide Icons font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from upstream `icons.json`.
pub struct LucideMetadata;

impl GlyphMetadata for LucideMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }
}
