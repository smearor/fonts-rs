//! Material Design icon entry data from the Google fonts metadata JSON.
//!
//! Each entry contains a name, category slugs, and search tags.

use serde::Deserialize;

use super::super::mapping::RawCategory;
use super::super::mapping::RawKeyword;

/// A single icon entry in the Material Design metadata.
#[derive(Debug, Deserialize)]
pub struct MdIconEntry {
    /// Icon name, e.g. `"home"`.
    pub name: String,
    /// Category slugs, e.g. `["action"]`.
    #[serde(default)]
    pub categories: Vec<RawCategory>,
    /// Search tags, e.g. `["house", "building"]`.
    #[serde(default)]
    pub tags: Vec<RawKeyword>,
}
