//! The [`IconWhitelistEntry`] enum - one icon in a source's whitelist.

use serde::Deserialize;
use serde::Serialize;

/// A single whitelisted icon - either a bare upstream name or a
/// `{ name, rename }` table exporting it under an application-specific
/// name.
///
/// Deserializes untagged: `"check"` parses as [`Name`](Self::Name),
/// `{ name = "trash", rename = "delete" }` as [`Renamed`](Self::Renamed).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IconWhitelistEntry {
    /// Plain upstream icon name, exported under its own name.
    Name(String),
    /// Upstream icon renamed in the generated set.
    Renamed {
        /// Upstream icon name (file stem), used as the whitelist lookup key.
        name: String,
        /// Exported icon name replacing the upstream name.
        /// Kebab-case, produces `{icon_prefix}-{rename}-symbolic`.
        rename: String,
    },
}

impl IconWhitelistEntry {
    /// The upstream icon name - the whitelist lookup key against SVG
    /// file stems.
    pub fn upstream_name(&self) -> &str {
        match self {
            Self::Name(name) => name,
            Self::Renamed { name, .. } => name,
        }
    }

    /// The exported icon base name, before the
    /// `{icon_prefix}-…-symbolic` decoration.
    pub fn exported_name(&self) -> &str {
        match self {
            Self::Name(name) => name,
            Self::Renamed { rename, .. } => rename,
        }
    }
}
