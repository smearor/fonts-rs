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
use fonts_rs_model::GlyphKeyword;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));

/// Returns search keywords for an emoji glyph, e.g. `["face", "grin"]` for
/// `"noto-emoji-grinning-face"`.
///
/// Accepts any type that implements `AsRef<str>` — this includes `&str`,
/// `String`, and `GlyphName<F>` from any font family crate.
///
/// Returns an empty slice if no keywords are available.
pub fn emoji_keywords(glyph_name: impl AsRef<str>) -> &'static [GlyphKeyword] {
    KEYWORDS.get(glyph_name.as_ref()).copied().unwrap_or(&[])
}

/// Returns the category for an emoji glyph, e.g. `"Smileys & Emotion"` for
/// `"noto-emoji-grinning-face"`.
///
/// Accepts any type that implements `AsRef<str>` — this includes `&str`,
/// `String`, and `GlyphName<F>` from any font family crate.
///
/// Returns `None` if no category is available.
pub fn emoji_category(glyph_name: impl AsRef<str>) -> Option<GlyphCategory> {
    CATEGORIES.get(glyph_name.as_ref()).copied()
}

/// Search emoji by keyword, category, or name.
///
/// Performs a case-insensitive substring match against glyph names,
/// keywords, and categories. Returns matching glyph names sorted
/// alphabetically.
pub fn search_emoji(query: &str) -> Vec<&'static str> {
    if query.is_empty() {
        return Vec::new();
    }

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (glyph_name, keywords) in KEYWORDS.entries() {
        let name_lower = glyph_name.to_lowercase();
        if name_lower.contains(&query) {
            results.push(*glyph_name);
            continue;
        }

        if keywords.iter().any(|kw| kw.as_str().to_lowercase().contains(&query)) {
            results.push(*glyph_name);
            continue;
        }

        if let Some(cat) = CATEGORIES.get(glyph_name) {
            if cat.as_str().to_lowercase().contains(&query) {
                results.push(*glyph_name);
                continue;
            }
        }
    }

    // Also include glyphs that have no keywords but match by name
    for (glyph_name, _) in CATEGORIES.entries() {
        if !results.contains(glyph_name) {
            let name_lower = glyph_name.to_lowercase();
            if name_lower.contains(&query) {
                results.push(*glyph_name);
            }
        }
    }

    results.sort_unstable();
    results.dedup();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_emoji_non_empty() {
        let results = search_emoji("face");
        assert!(!results.is_empty(), "search for 'face' should find emoji");
    }

    #[test]
    fn search_emoji_empty_query() {
        let results = search_emoji("");
        assert!(results.is_empty(), "empty query should return no results");
    }
}
