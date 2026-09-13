//! Newtype wrapping a `HashMap<CodePoint, Vec<String>>` for type-safe keyword lookups by codepoint.

use std::collections::HashMap;

use crate::CodePoint;

/// A mapping from Unicode codepoints to keyword lists.
///
/// Used by build-time metadata generators to look up CLDR annotation
/// keywords for each emoji codepoint.
#[derive(Debug, Clone, Default)]
pub struct CodePointKeywordMap(HashMap<CodePoint, Vec<String>>);

impl CodePointKeywordMap {
    /// Creates an empty `CodePointKeywordMap`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns a reference to the keywords for the given codepoint.
    pub fn get(&self, codepoint: &CodePoint) -> Option<&Vec<String>> {
        self.0.get(codepoint)
    }

    /// Inserts a codepoint → keywords mapping.
    pub fn insert(&mut self, codepoint: CodePoint, keywords: Vec<String>) -> Option<Vec<String>> {
        self.0.insert(codepoint, keywords)
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

impl IntoIterator for CodePointKeywordMap {
    type Item = (CodePoint, Vec<String>);
    type IntoIter = std::collections::hash_map::IntoIter<CodePoint, Vec<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a CodePointKeywordMap {
    type Item = (&'a CodePoint, &'a Vec<String>);
    type IntoIter = std::collections::hash_map::Iter<'a, CodePoint, Vec<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<(CodePoint, Vec<String>)> for CodePointKeywordMap {
    fn from_iter<I: IntoIterator<Item = (CodePoint, Vec<String>)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
