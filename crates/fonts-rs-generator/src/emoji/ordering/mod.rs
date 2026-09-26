//! Parser and codegen for `googlefonts/emoji-metadata` ordering files
//! (`emoji_*_ordering.json`).
//!
//! The ordering file describes the emoji picker grid layout: an ordered
//! list of groups, each containing emoji entries in picker order with
//! skin-tone/gender alternates, shortcodes (`:smile:`), and emoticons
//! (`:D`). It works for any font covering the emoji codepoint ranges
//! (e.g. Noto Emoji, Blobmoji).

pub mod emoji;
pub mod generator;
pub mod group;

pub use emoji::OrderingEmoji;
pub use generator::EmojiOrderingGenerator;
pub use group::OrderingGroup;

use serde::Deserialize;

/// One ordering file: a list of groups in picker order.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct EmojiOrdering(pub Vec<OrderingGroup>);

/// Parses an `emoji_*_ordering.json` document.
///
/// # Errors
///
/// Returns an error when the JSON is malformed or does not match the
/// `googlefonts/emoji-metadata` schema.
pub fn parse_emoji_ordering(json: &str) -> miette::Result<EmojiOrdering> {
    serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse emoji ordering: {e}"))
}
