//! The [`IconSetFile`] struct - root document of `icons.toml`.

use serde::Deserialize;
use serde::Serialize;

use crate::icon_set::spec::IconSetSpec;

/// Root document of `icons.toml` - wraps the `[iconset]` table so
/// `toml::from_str`/`toml::to_string` produce the documented layout:
///
/// ```toml
/// [iconset]
/// name = "MyAppIcons"
/// # …
///
/// [[iconset.sources]]
/// crate = "fonts-rs-lucide"
/// icons = ["check"]
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconSetFile {
    /// The `[iconset]` table.
    pub iconset: IconSetSpec,
}
