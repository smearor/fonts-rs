//! Font Awesome category data from `categories.yml`.
//!
//! Each top-level key in `categories.yml` is a category slug containing
//! an `icons` list and a human-readable `label`.

use serde::Deserialize;

use super::super::mapping::RawCategory;

/// Raw representation of a category entry in Font Awesome `categories.yml`.
#[derive(Debug, Deserialize)]
pub struct FaCategories {
    /// Icon names belonging to this category.
    pub icons: Vec<String>,
    /// Human-readable category label.
    pub label: RawCategory,
}
