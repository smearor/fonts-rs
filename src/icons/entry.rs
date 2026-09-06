//! Metadata entry for a single exported Nerd Font icon.

use std::path::PathBuf;

use super::codepoint::CodePoint;
use super::name::IconName;
use super::resource_path::ResourcePath;

/// Metadata entry for a single exported icon.
///
/// Serialized to `metadata.json` by the export tool and deserialized by
/// `build.rs` to generate `phf::Map`s and icon name constants.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IconEntry {
    /// Unicode codepoint (e.g. `U+F11B`), or `None` if the glyph has no
    /// Unicode mapping.
    pub code: Option<CodePoint>,
    /// Normalized GTK icon name (e.g. `nf-fa-gamepad-symbolic`).
    pub name: IconName,
    /// Relative file path to the SVG file (e.g. `resources/icons/nf-fa-gamepad-symbolic.svg`).
    pub file: PathBuf,
    /// Full GResource path (e.g. `/io/smearor/nerd_fonts/icons/nf-fa-gamepad-symbolic`).
    pub resource_path: ResourcePath,
}
