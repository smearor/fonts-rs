//! Single emoji entry within an ordering file.

use serde::Deserialize;

/// A single emoji entry in the ordering file.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderingEmoji {
    /// The base emoji as a codepoint sequence (e.g. `[128512]` for U+1F600,
    /// multi-element for ZWJ sequences). May carry a trailing U+FE0F.
    pub base: Vec<u32>,
    /// Skin-tone/gender/direction variants as codepoint sequences, each
    /// starting with the same base. Order matches picker expansion.
    #[serde(default)]
    pub alternates: Vec<Vec<u32>>,
    /// Text shortcodes such as `":smile:"`.
    #[serde(default)]
    pub shortcodes: Vec<String>,
    /// ASCII emoticons such as `":D"`.
    #[serde(default)]
    pub emoticons: Vec<String>,
    /// Whether upstream ships an animated variant; informational.
    #[serde(default)]
    pub animated: bool,
    /// Whether the emoji reads left/right; informational for RTL pickers.
    #[serde(default)]
    pub directional: bool,
    /// Opaque variant-shape group key: entries sharing a `tone_group`
    /// have the same alternate shape (0 = no tone variants).
    #[serde(default)]
    pub tone_group: u32,
}
