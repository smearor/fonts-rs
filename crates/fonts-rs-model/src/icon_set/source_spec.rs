//! The [`IconSetSourceSpec`] struct - one `[[iconset.sources]]` table.

use crate::icon_set::post_processor::IconSetPostProcessor;
use crate::icon_set::source_location::IconSetSourceLocation;
use crate::icon_set::whitelist_entry::IconWhitelistEntry;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

/// One `[[iconset.sources]]` table: a source location plus the subset
/// and processing applied to it.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconSetSourceSpec {
    /// Where this source's SVGs are located.
    #[serde(flatten)]
    pub location: IconSetSourceLocation,
    /// Post-processors applied to this source's exported SVGs.
    #[serde(default)]
    pub post_processors: Vec<IconSetPostProcessor>,
    /// Whitelist of icons to export - plain upstream names or renames.
    pub icons: Vec<IconWhitelistEntry>,
}

impl IconSetSourceSpec {
    /// The whitelist as a lookup set of upstream names.
    pub fn whitelist(&self) -> HashSet<String> {
        self.icons.iter().map(|entry| entry.upstream_name().to_string()).collect()
    }

    /// Upstream name → exported name map for the rename-aware name
    /// resolution. Entries without a rename map to themselves.
    pub fn rename_map(&self) -> HashMap<String, String> {
        self.icons
            .iter()
            .map(|entry| (entry.upstream_name().to_string(), entry.exported_name().to_string()))
            .collect()
    }
}
