//! Font family configuration for Heroicons.
//!
//! Defines the constants that make Heroicons a distinct font family:
//! family name, display name, GResource prefix, codepoint ranges, and
//! icon name prefix.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Heroicons codepoint ranges: hashed PUA allocations
/// (`CodepointStrategy::HashedPua`). The outline style keeps its
/// legacy BMP PUA arena (U+E000–U+F8FF); the other styles share a
/// salted PUA-B allocator (U+100000–U+10FFFD).
const HEROICONS_RANGES: &[CodePointRange] = &[
    CodePointRange::new(CodePoint::from_char('\u{E000}'), CodePoint::from_char('\u{F8FF}')),
    CodePointRange::new(CodePoint::from_char('\u{100000}'), CodePoint::from_char('\u{10FFFD}')),
];

/// Configuration for the Heroicons font family.
///
/// Heroicons is a fontless SVG-only set - there is no TTF, so there is
/// no CSS `font-family` name and no font-loader integration. Glyphs are
/// resolved through the GResource icon theme; codepoints are stable
/// hash-of-name PUA allocations.
pub struct HeroiconsConfig;

impl FontFamilyConfig for HeroiconsConfig {
    /// Cargo feature / font family identifier.
    const FONT_FAMILY_NAME: &'static str = "heroicons";

    /// Human-readable display name for UI and documentation.
    const FAMILY_DISPLAY_NAME: &'static str = "Heroicons";

    /// GResource path prefix for this family's resources.
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", HeroiconsConfig::FONT_FAMILY_NAME);

    /// Codepoint ranges covered by this font family (hashed PUA).
    const CODEPOINT_RANGES: &[CodePointRange] = HEROICONS_RANGES;

    /// Prefix for icon names and CSS classes (`heroicons-{name}`).
    const ICON_PREFIX: &'static str = "heroicons-";
}
