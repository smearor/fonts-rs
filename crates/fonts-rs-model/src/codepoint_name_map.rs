//! Newtype wrapping a `HashMap<CodePoint, String>` for type-safe glyph name lookups by codepoint.

use std::collections::HashMap;

use crate::CodePoint;

/// A mapping from Unicode codepoints to glyph names.
///
/// Used by build-time metadata generators to look up canonical glyph
/// names from CLDR annotation `tts` fields.
#[derive(Debug, Clone, Default)]
pub struct CodePointNameMap(HashMap<CodePoint, String>);

impl CodePointNameMap {
    /// Creates an empty `CodePointNameMap`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns a reference to the name for the given codepoint.
    pub fn get(&self, codepoint: &CodePoint) -> Option<&String> {
        self.0.get(codepoint)
    }

    /// Inserts a codepoint → name mapping.
    pub fn insert(&mut self, codepoint: CodePoint, name: String) -> Option<String> {
        self.0.insert(codepoint, name)
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for CodePointNameMap {
    type Item = (CodePoint, String);
    type IntoIter = std::collections::hash_map::IntoIter<CodePoint, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a CodePointNameMap {
    type Item = (&'a CodePoint, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, CodePoint, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<(CodePoint, String)> for CodePointNameMap {
    fn from_iter<I: IntoIterator<Item = (CodePoint, String)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
