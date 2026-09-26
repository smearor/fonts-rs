//! Icon metadata: keywords for search functionality.
//!
//! Provides keyword lookup for Eva icons, enabling search beyond
//! simple name matching — e.g. "alarm" finds `eva-bell-symbolic` and
//! `eva-clock-symbolic`.
//!
//! Keywords are sourced from upstream `icons-tags.json` (converted
//! from `icons-tags.ts`, the search tags powering the official
//! eva-icons.com cheat sheet). Eva ships no categories or aliases, so
//! the corresponding maps are empty.
//!
//! This module is only available when the `metadata` feature is enabled.

use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));

/// Metadata registry for the Eva Icons font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from upstream `icons-tags.json`.
pub struct EvaMetadata;

impl GlyphMetadata for EvaMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }
}
