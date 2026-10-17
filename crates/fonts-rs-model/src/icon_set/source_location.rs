//! The [`IconSetSourceLocation`] enum - where a source's SVGs live.

use crate::layout::Package;
use serde::Deserialize;
use serde::Serialize;

/// Source location for one `[[iconset.sources]]` table's SVG files.
///
/// Deserializes untagged from the flattened location keys:
/// `crate = "…"` (optionally with `subdirectory`) selects
/// [`Crate`](Self::Crate), `dir = "…"` selects [`Dir`](Self::Dir).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IconSetSourceLocation {
    /// SVGs from a dependency crate's packaged `resources/svg/`
    /// directory, located via its `DEP_*_SVG_DIR` links metadata or
    /// `cargo metadata` at build time.
    Crate {
        /// Dependency package name, e.g. `fonts-rs-lucide`.
        #[serde(rename = "crate")]
        name: Package,
        /// Subdirectory inside the crate's `resources/svg/`,
        /// e.g. `filled` or `outline`.
        #[serde(skip_serializing_if = "Option::is_none")]
        subdirectory: Option<String>,
    },
    /// SVGs from a local directory, resolved against
    /// `CARGO_MANIFEST_DIR`.
    Dir {
        /// Directory containing `*.svg` files.
        dir: String,
    },
}

impl IconSetSourceLocation {
    /// Stable identity string used as this source's `pua_salt` - keeps
    /// hashed PUA codepoints distinct across sources even when two
    /// sources ship same-named icons (e.g. `svg/fill` vs. `svg/outline`
    /// variants of one crate).
    pub fn salt(&self) -> String {
        match self {
            Self::Crate { name, subdirectory } => {
                format!("{name}:{}", subdirectory.as_deref().unwrap_or_default())
            }
            Self::Dir { dir } => format!("dir:{dir}"),
        }
    }

    /// Human-readable identifier for error messages.
    pub fn label(&self) -> String {
        match self {
            Self::Crate { name, subdirectory } => match subdirectory {
                Some(sub) => format!("crate '{name}' ({sub})"),
                None => format!("crate '{name}'"),
            },
            Self::Dir { dir } => format!("directory '{dir}'"),
        }
    }
}
