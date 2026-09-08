//! Devicon style alias entry from `devicon.json`.
//!
//! Maps a base style to an alias style, e.g. `{"base": "plain", "alias": "plain-wordmark"}`.

use serde::Deserialize;

/// A style alias entry in `devicon.json`, mapping a base style to an alias style.
#[derive(Debug, Deserialize)]
pub struct DeviconStyleAlias {
    /// The canonical style name, e.g. `"plain"`.
    pub base: String,
    /// The alias style name, e.g. `"plain-wordmark"`.
    pub alias: String,
}
