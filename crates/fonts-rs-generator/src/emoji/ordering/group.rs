//! Picker group within an emoji ordering file.

use serde::Deserialize;

use crate::emoji::ordering::OrderingEmoji;

/// An emoji picker group (e.g. `"Smileys and emotions"`).
#[derive(Debug, Clone, Deserialize)]
pub struct OrderingGroup {
    /// Display name of the group.
    pub group: String,
    /// Emoji entries in picker order.
    pub emoji: Vec<OrderingEmoji>,
}
