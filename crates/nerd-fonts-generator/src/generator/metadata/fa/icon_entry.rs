//! Font Awesome icon entry data from `icons.yml`.
//!
//! Each entry contains a `search` block with search terms and optionally
//! an `aliases` block with alternative code identifiers.

use serde::Deserialize;

use super::super::mapping::RawAlias;
use super::search_term::FaSearchTerms;

/// Raw representation of a single icon entry in Font Awesome `icons.yml`.
#[derive(Debug, Deserialize)]
pub struct FaIconEntry {
    /// Search terms for this icon.
    pub search: FaSearchTerms,
    /// Alternative code identifiers for this icon, if any.
    #[serde(default)]
    pub aliases: Vec<RawAlias>,
}
