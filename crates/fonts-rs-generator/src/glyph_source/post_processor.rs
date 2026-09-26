//! Post-processing hooks for SVG artwork.

use crate::glyph_source::error::GlyphSourceError;

/// Post-processing hook applied to SVG artwork before it is written
/// to the output directory.
///
/// Implementations should be XML-aware (`quick_xml` is already a
/// dependency of this crate) — naive string replacement would also
/// match `fill="…"` inside comments, CDATA, or embedded metadata.
pub trait SvgPostProcessor {
    /// Transforms the SVG source text.
    fn process(&self, svg: String) -> Result<String, GlyphSourceError>;
}
