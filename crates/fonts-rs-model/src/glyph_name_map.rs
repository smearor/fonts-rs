//! Newtype wrapping a `HashMap<String, CodePoint>` for type-safe glyph name lookups.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Deserializer;

use crate::CodePoint;

/// A mapping from glyph names to Unicode codepoints.
///
/// Used by build scripts to drive glyph export via
/// [`ExportConfig::export_glyphs_by_name_map`](fonts_rs_generator::ExportConfig) when font-internal
/// glyph names are not semantically meaningful (e.g. `uniXXXX` PostScript
/// names) and must be replaced with names from external metadata
/// (CLDR annotations, SMuFL glyphnames.json).
#[derive(Debug, Clone, Default)]
pub struct GlyphNameMap(HashMap<String, CodePoint>);

impl GlyphNameMap {
    /// Creates an empty `GlyphNameMap`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns a reference to the codepoint for the given glyph name.
    pub fn get(&self, name: &str) -> Option<&CodePoint> {
        self.0.get(name)
    }

    /// Inserts a glyph name → codepoint mapping.
    ///
    /// If the name already exists, the old value is returned.
    pub fn insert(&mut self, name: String, codepoint: CodePoint) -> Option<CodePoint> {
        self.0.insert(name, codepoint)
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the map entries.
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, CodePoint> {
        self.0.iter()
    }
}

impl IntoIterator for GlyphNameMap {
    type Item = (String, CodePoint);
    type IntoIter = std::collections::hash_map::IntoIter<String, CodePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a GlyphNameMap {
    type Item = (&'a String, &'a CodePoint);
    type IntoIter = std::collections::hash_map::Iter<'a, String, CodePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<(String, CodePoint)> for GlyphNameMap {
    fn from_iter<I: IntoIterator<Item = (String, CodePoint)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'de> Deserialize<'de> for GlyphNameMap {
    /// Deserializes a SMuFL glyphnames.json map into a [`GlyphNameMap`].
    ///
    /// The JSON format is: `{ "glyphName": { "codepoint": "U+E050", ... } }`.
    /// Only the `codepoint` field is extracted; other fields are ignored.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Entry {
            codepoint: CodePoint,
        }

        let map: HashMap<String, Entry> = HashMap::deserialize(deserializer)?;
        Ok(map.into_iter().map(|(name, e)| (name, e.codepoint)).collect())
    }
}
