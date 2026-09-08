//! Keyword mapping type: icon name to keywords.

use std::collections::HashMap;

use super::keyword::RawKeyword;

/// Maps upstream icon names to their keyword lists.
#[derive(Debug, Clone, Default)]
pub struct KeywordMapping(HashMap<String, Vec<RawKeyword>>);

impl KeywordMapping {
    /// Creates an empty keyword mapping.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the keyword list for the given icon name, if any.
    pub fn get(&self, icon_name: &str) -> Option<&[RawKeyword]> {
        self.0.get(icon_name).map(Vec::as_slice)
    }

    /// Inserts a keyword list for the given icon name.
    pub fn insert(&mut self, icon_name: String, keywords: Vec<RawKeyword>) {
        self.0.insert(icon_name, keywords);
    }

    /// Adds a keyword to the icon's keyword list, creating the entry if needed.
    pub fn add(&mut self, icon_name: &str, keyword: RawKeyword) {
        self.0.entry(icon_name.to_string()).or_default().push(keyword);
    }

    /// Returns the number of icons with keyword data.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if no icons have keyword data.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
