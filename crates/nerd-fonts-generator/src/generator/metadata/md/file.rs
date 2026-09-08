//! Material Design top-level metadata file structure.
//!
//! The Google fonts metadata JSON contains a single `icons` array.

use serde::Deserialize;

use super::icon_entry::MdIconEntry;

/// Top-level structure of the Material Design metadata JSON.
#[derive(Debug, Deserialize)]
pub struct MdMetadataFile {
    /// List of all Material Design icons.
    pub icons: Vec<MdIconEntry>,
}
