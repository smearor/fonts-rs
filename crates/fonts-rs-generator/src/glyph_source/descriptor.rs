//! Glyph identity as provided by a [`GlyphSource`](crate::glyph_source::GlyphSource).

use fonts_rs_model::CodePoint;

/// A glyph's identity as provided by a [`GlyphSource`](crate::glyph_source::GlyphSource).
///
/// The shared `export` runner enriches this with `file` and
/// `resource_path` to produce a [`GlyphEntry`](fonts_rs_model::GlyphEntry)
/// — sources stay free of GResource layout knowledge.
#[derive(Debug, Clone)]
pub struct GlyphDescriptor {
    /// Unicode codepoint, or `None` if the glyph has no mapping.
    pub code: Option<CodePoint>,
    /// Semantic glyph name.
    pub name: String,
    /// Opaque key the source uses to locate the glyph's artwork in
    /// `svg_for` (source filename for `SvgDirSource`, glyph index for
    /// `FontFileSource`).
    pub source_key: String,
    /// Full codepoint sequence for multi-codepoint glyphs (ZWJ emoji,
    /// keycap sequences); `None` for single-codepoint glyphs.
    pub sequence: Option<Vec<CodePoint>>,
}
