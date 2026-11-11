//! Devicon icon entry data from `devicon.json`.
//!
//! Each entry contains a name, alternative names, tags, and style aliases.

use serde::Deserialize;
use smearor_commons_color::Color;

use super::super::mapping::RawAlias;
use super::super::mapping::RawKeyword;
use super::style_alias::DeviconStyleAlias;

/// A single icon entry in `devicon.json`.
#[derive(Debug, Deserialize)]
pub struct DeviconEntry {
    /// Icon name, e.g. `"python"`.
    pub name: String,
    /// Alternative names for the icon.
    #[serde(default)]
    pub altnames: Vec<RawAlias>,
    /// Tags grouping the icon by type, e.g. `["language", "programming"]`.
    #[serde(default)]
    pub tags: Vec<RawKeyword>,
    /// Brand color of the icon, e.g. `"#3776AB"`.
    #[serde(default)]
    pub color: Option<Color<u8>>,
    /// Style variant aliases, e.g. `{"base": "plain", "alias": "plain-wordmark"}`.
    #[serde(default)]
    pub aliases: Vec<DeviconStyleAlias>,
}
