//! Font Awesome search terms block from `icons.yml`.
//!
//! Each icon entry contains a `search` block with a `terms` list.

use serde::Deserialize;

use super::super::mapping::RawKeyword;

/// Search terms block within a Font Awesome icon entry.
#[derive(Debug, Deserialize)]
pub struct FaSearchTerms {
    /// List of search term keywords.
    #[serde(default)]
    pub terms: Vec<RawKeyword>,
}
