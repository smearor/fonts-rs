//! Generator for emoji metadata phf::Map tables (keywords and categories).
//!
//! Keywords are sourced from CLDR annotation data (`default` field).
//! Categories are sourced from Unicode `emoji-test.txt` (`group` field).
//!
//! Implements [`MetadataGenerator`] for emoji families - keyword and
//! category maps are keyed by Unicode codepoint, so the generator works
//! for any font covering the emoji ranges (Noto Emoji, Blobmoji, ...).

use crate::MetadataGenerator;
use fonts_rs_model::CodePointCategoryMap;
use fonts_rs_model::CodePointKeywordMap;
use fonts_rs_model::GlyphEntry;

/// Generates phf::Map constants for emoji keywords and categories.
///
/// Keywords come from CLDR annotation data. Categories come from
/// Unicode emoji-test.txt group headers.
pub struct EmojiMetadataGenerator {
    keyword_map: CodePointKeywordMap,
    category_map: CodePointCategoryMap,
    alias_pairs: Vec<(String, String)>,
}

impl EmojiMetadataGenerator {
    /// Creates a new metadata generator with the given keyword and category maps.
    pub fn new(keyword_map: CodePointKeywordMap, category_map: CodePointCategoryMap) -> Self {
        Self {
            keyword_map,
            category_map,
            alias_pairs: Vec::new(),
        }
    }

    /// Adds `(alias, canonical_name)` pairs to the generated `aliases.rs`
    /// map - e.g. shortcodes/emoticons from the emoji ordering file.
    pub fn with_aliases(mut self, alias_pairs: Vec<(String, String)>) -> Self {
        self.alias_pairs = alias_pairs;
        self
    }
}

impl MetadataGenerator for EmojiMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        let Some(code) = &entry.code else { return Vec::new() };
        self.keyword_map.get(code).cloned().unwrap_or_default()
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        let Some(code) = &entry.code else { return Vec::new() };
        self.category_map.get(code).map(|c| vec![c.clone()]).unwrap_or_default()
    }

    fn aliases(&self) -> Vec<(String, String)> {
        self.alias_pairs.clone()
    }
}
