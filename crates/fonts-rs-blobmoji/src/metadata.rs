//! Emoji metadata: keywords, categories, and picker ordering.
//!
//! Provides keyword and category lookup for Blobmoji glyphs, enabling
//! search and filtering beyond simple name matching, plus the canonical
//! emoji picker group ordering.
//!
//! Keywords are sourced from CLDR annotation data (`default` field) plus
//! shortcodes (`:smile:`) and emoticons (`:D`) from the ordering file.
//! Categories are sourced from Unicode `emoji-test.txt` (`group` field).
//! Multi-codepoint sequences (ZWJ, keycap, flag) carry no CLDR keywords -
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
include!(concat!(env!("OUT_DIR"), "/ordering.rs"));

/// An emoji picker group in canonical ordering.
///
/// Groups and glyph order follow `googlefonts/emoji-metadata` (the same
/// ordering used by Android's emoji picker); skin-tone and gender
/// alternates are inlined after their base emoji.
#[derive(Debug, Clone, Copy)]
pub struct EmojiGroup {
    /// Display name of the group (e.g. `"Smileys and emotions"`).
    pub name: &'static str,
    /// Exported glyph names in picker order.
    pub glyphs: &'static [&'static str],
}

impl EmojiGroup {
    /// Creates a group. Called by the generated `GROUPS` table.
    pub const fn new(name: &'static str, glyphs: &'static [&'static str]) -> Self {
        Self { name, glyphs }
    }
}

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

impl BlobmojiMetadata {
    /// Returns the emoji picker groups in canonical ordering.
    ///
    /// Each [`EmojiGroup`] lists exported glyph names in picker order.
    /// Useful for emoji pickers that want the Android-style group layout
    /// instead of an alphabetical grid.
    pub fn groups(&self) -> &'static [EmojiGroup] {
        GROUPS
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

    #[test]
    fn groups_are_non_empty() {
        let groups = BlobmojiMetadata.groups();
        assert!(groups.len() >= 5, "expected multiple emoji picker groups, got {}", groups.len());
        for group in groups {
            assert!(!group.name.is_empty(), "group name should not be empty");
            assert!(!group.glyphs.is_empty(), "group '{}' should contain glyphs", group.name);
        }
    }

    #[test]
    fn groups_first_is_smileys() {
        let first = BlobmojiMetadata.groups().first().expect("at least one group");
        assert!(first.name.contains("Smileys"), "first group should be smileys, got '{}'", first.name);
        assert!(first.glyphs.iter().any(|name| name.ends_with("grinning-face")), "first group should contain grinning-face");
    }

    #[test]
    fn groups_contain_exported_names() {
        let exported: std::collections::HashSet<String> = crate::all_emoji().into_iter().map(|(_, name)| name).collect();
        for group in BlobmojiMetadata.groups() {
            for name in group.glyphs {
                assert!(exported.contains(*name), "grouped glyph '{name}' is not an exported name");
            }
        }
    }

    #[test]
    fn shortcode_keyword_search() {
        let results = BlobmojiMetadata.search(":smile:");
        assert!(!results.is_empty(), "shortcode ':smile:' should hit search");
        let results = BlobmojiMetadata.search(":joy:");
        assert!(!results.is_empty(), "shortcode ':joy:' should hit search");
    }
}
