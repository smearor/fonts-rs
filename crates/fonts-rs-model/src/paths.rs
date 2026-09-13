//! Shared constants for GResource paths.
//!
//! All font families in the `fonts-rs` ecosystem share a common base
//! GResource prefix. Each family appends its own sub-path to this base.

/// Base GResource prefix for all fonts-rs resources.
///
/// Individual font families build their full prefix by appending their
/// family-specific sub-path (e.g. `nerd_fonts`, `dseg7`).
///
/// # Examples
///
/// - Nerd Fonts: `{GRESOURCE_BASE_PREFIX}/nerd_fonts` -> `/io/smearor/fonts/nerd_fonts`
/// - Seven Segment: `{GRESOURCE_BASE_PREFIX}/dseg7` -> `/io/smearor/fonts/dseg7`
pub const GRESOURCE_BASE_PREFIX: &str = "/io/smearor/fonts";

/// Subdirectory for vector (SVG) icons, following the Freedesktop Icon Theme
/// Specification used by GTK 4's `GtkIconTheme`.
pub const SCALABLE_DIR: &str = "scalable";

/// Default icon context for font glyph icons.
pub const ICONS_CONTEXT_GLYPHS: &str = "glyphs";

/// Icon context for emoji icons.
pub const ICONS_CONTEXT_EMOJI: &str = "emoji";
