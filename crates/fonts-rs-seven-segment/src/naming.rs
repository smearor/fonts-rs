//! Font family marker and `FontDefinition` implementation for DSEG7 Classic.

use const_format::concatcp;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_model::FontFamily;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying DSEG7 Classic in `GlyphName<SevenSegment>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<SevenSegment>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SevenSegment {}

impl FontFamily for SevenSegment {}

impl sealed::Sealed for SevenSegment {}

/// Convenience type alias for seven-segment glyph names.
pub type SevenSegmentName = GlyphName<SevenSegment>;

/// Build-time configuration for the DSEG7 Classic font family.
///
/// Implements [`FontDefinition`] to plug into the generic
/// [`FontDefinition::export_glyphs`] pipeline.
pub struct SevenSegmentDefinition;

impl FontDefinition for SevenSegmentDefinition {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/seven_segment");

    const ICONS_CONTEXT: &'static str = "glyphs";

    /// Unicode codepoint range for DSEG7 Classic.
    ///
    /// DSEG7 maps glyphs in the ASCII range (`U+0020`–`U+007E`),
    /// covering digits 0–9, letters A–F (for hex), and common
    /// punctuation used in seven-segment displays.
    const CODEPOINT_RANGES: &[(u32, u32)] = &[(0x20, 0x7E)];

    type Name = GlyphName<SevenSegment>;

    type Family = SevenSegment;

    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        let name = normalize_to_kebab(raw_glyph_name)?;
        Some(GlyphName::new(format!("dseg7-{name}")))
    }
}
