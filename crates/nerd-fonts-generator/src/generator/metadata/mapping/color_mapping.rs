//! Color mapping type: icon name to brand color.

use std::collections::HashMap;

use smearor_commons_color::Color;

/// Maps upstream icon names to their brand color.
#[derive(Debug, Clone, Default)]
pub struct ColorMapping(HashMap<String, Color<u8>>);

impl ColorMapping {
    /// Creates an empty color mapping.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the brand color for the given icon name, if any.
    pub fn get(&self, icon_name: &str) -> Option<&Color<u8>> {
        self.0.get(icon_name)
    }

    /// Inserts a brand color for the given icon name.
    pub fn insert(&mut self, icon_name: &str, color: Color<u8>) {
        self.0.insert(icon_name.to_string(), color);
    }

    /// Returns the number of icons with a brand color.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if no icon has a brand color.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
