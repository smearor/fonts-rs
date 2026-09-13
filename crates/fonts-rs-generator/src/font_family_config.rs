//! Base configuration trait for font families.
//!
//! Provides compile-time constants shared by both [`FontDefinition`](crate::FontDefinition)
//! (type-safe glyph name pipeline) and [`ExportConfig`](crate::ExportConfig)
//! (runtime variant pipeline).

use fonts_rs_model::BMP_RANGE;
use fonts_rs_model::CodePointRange;

/// Base configuration for font families.
///
/// Implemented by all font family definition types to provide
/// compile-time constants for GResource prefixes, icon context
/// directories, and codepoint ranges.
///
/// [`FontDefinition`](crate::FontDefinition) extends this trait with
/// type-safe glyph name handling. [`ExportConfig`](crate::ExportConfig)
/// is generic over any type implementing this trait.
pub trait FontFamilyConfig {
    /// GResource prefix for this font family.
    ///
    /// e.g. `/io/smearor/fonts/seven_segment` or `/io/smearor/fonts/nerd_fonts`.
    const GRESOURCE_PREFIX: &'static str;

    /// Icon context subdirectory within the GResource prefix.
    ///
    /// Follows the Freedesktop Icon Theme Specification used by GTK 4's
    /// `GtkIconTheme`: `{prefix}/scalable/{context}/{name}.svg`.
    ///
    /// The `scalable` directory signals that icons are vector (SVG) and
    /// can be rendered at any size. The `context` subdirectory groups
    /// icons by semantic category (e.g. `glyphs`, `status`, `actions`).
    ///
    /// For font glyph icons, `"glyphs"` is the default context.
    /// Font families can override this to use a different context if
    /// needed (e.g. `"emoji"` for Noto Emoji).
    const ICONS_CONTEXT: &'static str = "glyphs";

    /// Unicode codepoint ranges to probe when building the reverse cmap.
    ///
    /// Each tuple is `(start, end)` inclusive. The generic pipeline probes
    /// these ranges to map `GlyphId` -> `CodePoint` for each glyph in the
    /// font.
    ///
    /// Defaults to the BMP (`U+0000`–`U+FFFF`), which covers most fonts.
    /// Font families with glyphs in supplementary planes (e.g. Nerd Fonts
    /// PUA at `U+F0001`–`U+10FFFF`) should override this.
    const CODEPOINT_RANGES: &[CodePointRange] = &[BMP_RANGE];
}
