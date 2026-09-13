//! Generator for Noto Emoji metadata phf::Map tables (keywords and categories).
//!
//! Keywords are sourced from CLDR annotation data (`default` field).
//! Categories are sourced from Unicode `emoji-test.txt` (`group` field).
//!
//! Implements [`MetadataGenerator`] from `fonts-rs-generator`.

use fonts_rs_generator::MetadataGenerator;
use fonts_rs_model::CodePointCategoryMap;
use fonts_rs_model::CodePointKeywordMap;
use fonts_rs_model::GlyphEntry;

/// Generates phf::Map constants for emoji keywords and categories.
///
/// Keywords come from CLDR annotation data. Categories come from
/// Unicode emoji-test.txt group headers.
pub struct NotoEmojiMetadataGenerator {
    keyword_map: CodePointKeywordMap,
    category_map: CodePointCategoryMap,
}

impl NotoEmojiMetadataGenerator {
    /// Creates a new metadata generator with the given keyword and category maps.
    pub fn new(keyword_map: CodePointKeywordMap, category_map: CodePointCategoryMap) -> Self {
        Self { keyword_map, category_map }
    }
}

impl MetadataGenerator for NotoEmojiMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        let Some(code) = &entry.code else { return Vec::new() };
        self.keyword_map.get(code).cloned().unwrap_or_default()
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        let Some(code) = &entry.code else { return Vec::new() };
        self.category_map.get(code).map(|c| vec![c.clone()]).unwrap_or_default()
    }
}
