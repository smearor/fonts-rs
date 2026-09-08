//! Font Awesome shim entry data from `shims.json`.
//!
//! Each entry maps an old (v4/v5) icon name to a v6 replacement.

use serde::Deserialize;

use super::shim_replacement::FaShimReplacement;

/// Raw representation of a single entry in Font Awesome `shims.json`.
#[derive(Debug, Deserialize)]
pub struct FaShim {
    /// The old (v4/v5) icon name.
    pub name: String,
    /// The replacement icon name in v6.
    pub replacement: Option<FaShimReplacement>,
}
