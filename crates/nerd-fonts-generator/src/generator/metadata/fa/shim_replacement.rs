//! Font Awesome shim replacement target from `shims.json`.

use serde::Deserialize;

/// The replacement target for a shim entry.
#[derive(Debug, Deserialize)]
pub struct FaShimReplacement {
    /// The canonical v6 icon name.
    pub name: String,
}
