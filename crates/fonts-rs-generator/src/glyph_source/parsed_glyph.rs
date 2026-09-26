//! Result of parsing an SVG filename into a glyph identity.

use fonts_rs_model::CodePoint;

/// Result of parsing an SVG filename into a glyph identity.
///
/// Produced by [`SvgFilenameParser`](crate::glyph_source::SvgFilenameParser)
/// implementations and consumed by `SvgDirSource` for codepoint and name
/// resolution.
#[derive(Debug, Clone)]
pub struct ParsedGlyph {
    /// Codepoint sequence encoded in the filename, if any.
    pub codepoints: Option<Vec<CodePoint>>,
    /// Raw glyph name extracted from the filename.
    pub raw_name: String,
}
