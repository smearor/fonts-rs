//! Parser for `googlefonts/emoji-metadata` ordering files (`emoji_*_ordering.json`).
//!
//! The ordering file describes the emoji picker grid layout: an ordered
//! list of groups, each containing emoji entries in picker order with
//! skin-tone/gender alternates, shortcodes (`:smile:`), and emoticons
//! (`:D`). It works for any font covering the emoji codepoint ranges
//! (e.g. Noto Emoji, Blobmoji).

use serde::Deserialize;

/// One ordering file: a list of groups in picker order.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct EmojiOrdering(pub Vec<OrderingGroup>);

/// An emoji picker group (e.g. `"Smileys and emotions"`).
#[derive(Debug, Clone, Deserialize)]
pub struct OrderingGroup {
    /// Display name of the group.
    pub group: String,
    /// Emoji entries in picker order.
    pub emoji: Vec<OrderingEmoji>,
}

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
}

/// Parses an `emoji_*_ordering.json` document.
///
/// # Errors
///
/// Returns an error when the JSON is malformed or does not match the
/// `googlefonts/emoji-metadata` schema.
pub fn parse_emoji_ordering(json: &str) -> miette::Result<EmojiOrdering> {
    serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse emoji ordering: {e}"))
}
