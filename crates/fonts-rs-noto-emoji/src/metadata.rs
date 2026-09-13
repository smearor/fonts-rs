//! Emoji metadata: keywords and categories for search functionality.
//!
//! Provides keyword and category lookup for Noto Emoji glyphs, enabling
//! search and filtering beyond simple name matching.
//!
//! Keywords are sourced from CLDR annotation data (`default` field).
//! Categories are sourced from Unicode `emoji-test.txt` (`group` field).
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

/// Metadata registry for the Noto Emoji font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from CLDR keyword annotations and Unicode emoji-test categories.
pub struct NotoEmojiMetadata;

impl GlyphMetadata for NotoEmojiMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_emoji_non_empty() {
        let results = NotoEmojiMetadata.search("face");
        assert!(!results.is_empty(), "search for 'face' should find emoji");
    }

    #[test]
    fn search_emoji_empty_query() {
        let results = NotoEmojiMetadata.search("");
        assert!(results.is_empty(), "empty query should return no results");
    }

    #[test]
    fn emoji_keywords_returns_slice() {
        let kws = NotoEmojiMetadata.keywords("noto-emoji-grinning-face");
        assert!(!kws.is_empty(), "grinning-face should have keywords");
    }

    #[test]
    fn emoji_categories_returns_slice() {
        let cats = NotoEmojiMetadata.categories("noto-emoji-grinning-face");
        assert!(!cats.is_empty(), "grinning-face should have a category");
    }
}
