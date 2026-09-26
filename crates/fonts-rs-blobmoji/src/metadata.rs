//! Emoji metadata: keywords, categories, aliases, and picker ordering.
//!
//! Provides keyword and category lookup for Blobmoji glyphs, enabling
//! search and filtering beyond simple name matching, plus the canonical
//! emoji picker group ordering.
//!
//! Keywords are sourced from CLDR annotation data (`default` field).
//! Categories are sourced from Unicode `emoji-test.txt` (`group` field).
//! Aliases are shortcodes (`:smile:`) and emoticons (`:D`) from
//! `googlefonts/emoji-metadata`. Multi-codepoint sequences (ZWJ, keycap,
//! flag) carry no CLDR keywords - the CLDR keyword map is keyed by
//! single codepoints.
//!
//! Ordering tables (`ORDERING`, `GROUPS`, `GROUP_OF`, `ALTERNATES`,
//! `BASE_OF`, `SHORTCODES`) are generated from
//! `emoji_17_0_ordering.json`; alternates (skin tones, gender, ZWJ
//! combos) are linked to their base emoji.
//!
//! This module is only available when the `metadata` feature is enabled.

use fonts_rs_model::GlyphAliasMap;
use fonts_rs_model::GlyphCategory;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphGroup;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;
use fonts_rs_model::GlyphOrdering;
use fonts_rs_model::GroupLabel;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));
include!(concat!(env!("OUT_DIR"), "/ordering.rs"));
include!(concat!(env!("OUT_DIR"), "/groups.rs"));
include!(concat!(env!("OUT_DIR"), "/alternates.rs"));
include!(concat!(env!("OUT_DIR"), "/shortcodes.rs"));

/// Metadata registry for the Blobmoji font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from CLDR keyword annotations, Unicode emoji-test categories,
/// and `emoji-metadata` shortcode/emoticon aliases.
pub struct BlobmojiMetadata;

impl GlyphMetadata for BlobmojiMetadata {
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

impl GlyphOrdering for BlobmojiMetadata {
    /// Returns the canonical picker position of a base emoji.
    ///
    /// The index is flat across all ordered base entries (grouped
    /// order); alternates and unordered glyphs return `None`.
    fn order_index(&self, name: &str) -> Option<u32> {
        ORDERING.get(name).copied()
    }

    /// Returns the picker group label (e.g. `"Smileys and emotions"`)
    /// for a glyph, including alternates.
    fn group(&self, name: &str) -> Option<GroupLabel> {
        GROUP_OF.get(name).copied().map(GroupLabel::new)
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        GROUP_ORDER
    }

    /// Returns the emoji picker groups in canonical ordering.
    ///
    /// Each [`GlyphGroup`] lists exported glyph names in picker order,
    /// alternates inlined after their base. Useful for emoji pickers
    /// that want the Android-style group layout instead of an
    /// alphabetical grid.
    fn groups(&self) -> &'static [GlyphGroup] {
        GROUPS
    }

    /// Returns the variant glyph names of a base emoji (skin tones,
    /// gender combos, ...), in picker order. Empty for emoji without
    /// alternates.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        ALTERNATES.get(name).copied().unwrap_or_default()
    }

    /// Returns the base emoji name for a variant glyph (e.g. a
    /// skin-tone sequence), or `None` if `name` is a base glyph.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        BASE_OF.get(name).copied()
    }
}

impl BlobmojiMetadata {
    /// Returns the shortcodes and emoticons registered for a glyph
    /// (`":smile:"`, `":+1:"`, `":D"`), or an empty slice.
    pub fn shortcodes(&self, name: &str) -> &'static [&'static str] {
        SHORTCODES.get(name).copied().unwrap_or_default()
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
            assert!(!group.name.as_str().is_empty(), "group name should not be empty");
            assert!(!group.glyphs.is_empty(), "group '{}' should contain glyphs", group.name);
        }
    }

    #[test]
    fn groups_first_is_smileys() {
        let first = BlobmojiMetadata.groups().first().expect("at least one group");
        assert!(first.name.as_str().contains("Smileys"), "first group should be smileys, got '{}'", first.name);
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

    /// `:smile:` (U+1F600, "grinning face") is a shortcode alias —
    /// `resolve_alias` returns the canonical name.
    #[test]
    fn resolve_alias_shortcode() {
        let canonical = BlobmojiMetadata.resolve_alias(":smile:").expect(":smile: should resolve");
        assert!(canonical.ends_with("grinning-face"), "got '{canonical}'");
    }

    #[test]
    fn order_index_is_picker_order() {
        let grinning = grinning_face_name();
        let index = BlobmojiMetadata.order_index(&grinning).expect("grinning-face should be ordered");
        let next = BlobmojiMetadata.groups()[0].glyphs[1];
        let next_index = BlobmojiMetadata.order_index(next).expect("second glyph should be ordered");
        assert_eq!(index + 1, next_index, "group order should be a flat consecutive index");
    }

    #[test]
    fn group_label_for_base() {
        let grinning = grinning_face_name();
        let group = BlobmojiMetadata.group(&grinning).expect("grinning-face should have a group");
        assert!(group.as_str().contains("Smileys"), "got '{group}'");
    }

    /// `thumbs-up` (U+1F44D) has five skin-tone alternates upstream;
    /// `base_of`/`alternates` must round-trip.
    #[test]
    fn alternates_round_trip() {
        let thumbs_up = crate::all_emoji()
            .into_iter()
            .map(|(_, name)| name)
            .find(|name| name.ends_with("thumbs-up") || name.ends_with("thumbsup"))
            .expect("thumbs-up glyph should exist");
        let alternates = BlobmojiMetadata.alternates(&thumbs_up);
        assert!(!alternates.is_empty(), "thumbs-up should have skin-tone alternates");
        for variant in alternates {
            assert_eq!(BlobmojiMetadata.base_of(variant), Some(thumbs_up.as_str()), "variant '{variant}' should map back to base");
            assert_eq!(
                BlobmojiMetadata.group(variant),
                BlobmojiMetadata.group(&thumbs_up),
                "variant '{variant}' should share the base's group"
            );
        }
        // The base itself is not its own alternate.
        assert_eq!(BlobmojiMetadata.base_of(&thumbs_up), None);
    }

    #[test]
    fn shortcodes_accessor() {
        let results = BlobmojiMetadata.search(":smile:");
        let name = results.first().expect(":smile: should resolve");
        let shortcodes = BlobmojiMetadata.shortcodes(name);
        assert!(shortcodes.contains(&":smile:"), "expected ':smile:' in {shortcodes:?}");
        let empty = BlobmojiMetadata.shortcodes("blobmoji-nonexistent");
        assert!(empty.is_empty());
    }
}
