//! Pluggable SVG filename parsing for directory-based glyph sources.

use crate::glyph_source::parsed_glyph::ParsedGlyph;

/// Parses an SVG filename into a [`ParsedGlyph`].
///
/// Returns `None` for files that are not glyphs (e.g. `.gitkeep`,
/// license files), which are skipped silently.
///
/// Generic patterns ship as built-ins in this crate; genuinely
/// vendor-specific formats implement this trait in their
/// family-generator crate.
pub trait SvgFilenameParser {
    /// Parses `filename` (basename only) into a glyph identity.
    fn parse(&self, filename: &str) -> Option<ParsedGlyph>;
}
