//! Generic metadata entry for a single exported glyph.

use crate::codepoint::CodePoint;
use crate::resource_path::ResourcePath;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metadata for a single exported glyph.
///
/// Generic over the font family's name type `N`. For simple font families,
/// `N` is `GlyphName<F>` (e.g. `GlyphName<SevenSegment>`). For Nerd Fonts,
/// `N` is `IconName`, which carries additional semantics like `IconSet`
/// detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlyphEntry<N: AsRef<str>> {
    /// Unicode codepoint (e.g. `U+F11B`), or `None` if the glyph has no
    /// Unicode mapping.
    pub code: Option<CodePoint>,
    /// Normalized glyph name (e.g. `nf-fa-gamepad-symbolic` or `dseg7-0`).
    pub name: N,
    /// Relative file path to the SVG file.
    pub file: PathBuf,
    /// Full GResource path.
    pub resource_path: ResourcePath,
}
