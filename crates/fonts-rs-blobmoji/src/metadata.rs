//! Emoji metadata: keywords and categories for search functionality.
//!
//! Provides keyword and category lookup for Blobmoji glyphs, enabling
//! search and filtering beyond simple name matching.
//!
//! Keywords are sourced from CLDR annotation data (`default` field).
//! Categories are sourced from Unicode `emoji-test.txt` (`group` field).
//! Multi-codepoint sequences (ZWJ, keycap, flag) carry no keywords -
//! the CLDR keyword map is keyed by single codepoints.
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

/// Metadata registry for the Blobmoji font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from CLDR keyword annotations and Unicode emoji-test categories.
pub struct BlobmojiMetadata;

impl GlyphMetadata for BlobmojiMetadata {
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
        let results = BlobmojiMetadata.search("face");
        assert!(!results.is_empty(), "search for 'face' should find emoji");
    }

    #[test]
    fn search_emoji_empty_query() {
        let results = BlobmojiMetadata.search("");
        assert!(results.is_empty(), "empty query should return no results");
    }

    /// Returns the variant-prefixed glyph name for U+1F600 (grinning face),
    /// derived from the actual glyph map so the test works with any active
    /// variant.
    fn grinning_face_name() -> String {
        crate::all_emoji()
            .into_iter()
            .map(|(_, name)| name)
            .find(|name| name.ends_with("grinning-face"))
            .expect("grinning-face glyph should exist")
    }

    #[test]
    fn emoji_keywords_returns_slice() {
        let kws = BlobmojiMetadata.keywords(grinning_face_name());
        assert!(!kws.is_empty(), "grinning-face should have keywords");
    }

    #[test]
    fn emoji_categories_returns_slice() {
        let cats = BlobmojiMetadata.categories(grinning_face_name());
        assert!(!cats.is_empty(), "grinning-face should have a category");
    }
}
