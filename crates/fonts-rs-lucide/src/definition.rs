//! Font family configuration for Lucide Icons.
//!
//! Defines the constants that make Lucide a distinct font family:
//! family name, display name, GResource prefix, codepoint ranges, and
//! icon name prefix.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Lucide codepoint range: hashed PUA allocations
/// (U+E000–U+F8FF, `CodepointStrategy::HashedPua`).
const LUCIDE_RANGES: &[CodePointRange] = &[CodePointRange::new(CodePoint::from_char('\u{E000}'), CodePoint::from_char('\u{F8FF}'))];

/// Configuration for the Lucide Icons font family.
///
/// Lucide is a fontless SVG-only set — there is no TTF, so there is no
/// CSS `font-family` name and no font-loader integration. Glyphs are
/// resolved through the GResource icon theme; codepoints are stable
/// hash-of-name PUA allocations.
pub struct LucideConfig;

impl FontFamilyConfig for LucideConfig {
    /// Cargo feature / font family identifier.
    const FONT_FAMILY_NAME: &'static str = "lucide";

    /// Human-readable display name for UI and documentation.
    const FAMILY_DISPLAY_NAME: &'static str = "Lucide Icons";

    /// GResource path prefix for this family's resources.
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", LucideConfig::FONT_FAMILY_NAME);

    /// Codepoint ranges covered by this font family (hashed PUA).
    const CODEPOINT_RANGES: &[CodePointRange] = LUCIDE_RANGES;

    /// Prefix for icon names and CSS classes (`lucide-{name}`).
    const ICON_PREFIX: &'static str = "lucide-";
}
