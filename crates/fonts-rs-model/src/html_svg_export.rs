//! Inline-SVG HTML export capability.
//!
//! Provides the [`HtmlSvgExport`] trait for font families that can
//! produce self-contained HTML pages using inline `<svg>` elements -
//! no font file required.

use crate::HtmlBuilder;
use crate::IconThemeFont;

/// Self-contained inline-SVG HTML export.
///
/// The only requirement is [`IconThemeFont::glyph_svg`], so fontless
/// icon families (e.g. Lucide) can implement this trait without a TTF.
/// Families that also ship a TTF implement [`HtmlExport`](crate::HtmlExport)
/// instead, which adds `@font-face` support on top.
///
/// This trait is implemented explicitly per family (opt-in via the
/// `html_svg` capability of `impl_font_family!`), unlike
/// [`HtmlExport`](crate::HtmlExport) which additionally requires a
/// loadable font.
pub trait HtmlSvgExport: IconThemeFont {
    /// Starts an [`HtmlBuilder`] for exporting a self-contained HTML
    /// page with inline `<svg>` glyph elements.
    fn html() -> HtmlBuilder<Self>
    where
        Self: Sized,
    {
        HtmlBuilder::new()
    }
}
