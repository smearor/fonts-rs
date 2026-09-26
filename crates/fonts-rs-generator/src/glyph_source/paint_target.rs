//! Paint attribute selection for [`CurrentColor`](crate::glyph_source::CurrentColor).

/// Which SVG paint attributes [`CurrentColor`](crate::glyph_source::CurrentColor)
/// rewrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintTarget {
    /// Rewrite `stroke` (stroke-based sets: Tabler, Lucide, Feather).
    Stroke,
    /// Rewrite `fill` (fill-based sets: Heroicons solid, Simple Icons).
    Fill,
    /// Rewrite both `stroke` and `fill`.
    Both,
}

impl PaintTarget {
    /// Whether `stroke` values are rewritten.
    pub(crate) fn rewrites_stroke(&self) -> bool {
        matches!(self, Self::Stroke | Self::Both)
    }

    /// Whether `fill` values are rewritten.
    pub(crate) fn rewrites_fill(&self) -> bool {
        matches!(self, Self::Fill | Self::Both)
    }
}
