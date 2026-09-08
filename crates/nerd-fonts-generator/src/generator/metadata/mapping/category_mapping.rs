//! Category mapping type: icon name to categories.

use std::collections::HashMap;

use super::category::RawCategory;

/// Maps upstream icon names to their category lists.
#[derive(Debug, Clone, Default)]
pub struct CategoryMapping(HashMap<String, Vec<RawCategory>>);

impl CategoryMapping {
    /// Creates an empty category mapping.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the category list for the given icon name, if any.
    pub fn get(&self, icon_name: &str) -> Option<&[RawCategory]> {
        self.0.get(icon_name).map(Vec::as_slice)
    }

    /// Inserts a category list for the given icon name.
    pub fn insert(&mut self, icon_name: String, categories: Vec<RawCategory>) {
        self.0.insert(icon_name, categories);
    }

    /// Adds a category to the icon's category list, creating the entry if needed.
    pub fn add(&mut self, icon_name: &str, category: RawCategory) {
        self.0.entry(icon_name.to_string()).or_default().push(category);
    }

    /// Returns the number of icons with category data.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if no icons have category data.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
