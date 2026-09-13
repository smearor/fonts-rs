//! Generic metadata entry for a single exported glyph.

use crate::codepoint::CodePoint;
use crate::paths::SCALABLE_DIR;
use crate::resource_path::ResourcePath;
use serde::Deserialize;
use serde::Serialize;
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

impl<N: AsRef<str>> GlyphEntry<N> {
    /// Create a new `GlyphEntry`, deriving `file` and `resource_path` from
    /// the GResource prefix, icons context, and glyph name.
    ///
    /// The `file` path is constructed as `resources/{scalable}/{context}/{name}.svg`
    /// and the `resource_path` as `{gresource_prefix}/{scalable}/{context}/{name}`.
    ///
    /// This avoids manual `format!` calls and ensures consistency between
    /// the two paths.
    pub fn new(code: Option<CodePoint>, name: N, gresource_prefix: &str, icons_context: &str) -> Self {
        let name_ref = name.as_ref();
        let file = PathBuf::from(format!("resources/{SCALABLE_DIR}/{icons_context}/{name_ref}.svg"));
        let resource_prefix = format!("{gresource_prefix}/{SCALABLE_DIR}/{icons_context}");
        let resource_path = ResourcePath::from_name(&resource_prefix, name_ref);
        Self { code, name, file, resource_path }
    }
}
