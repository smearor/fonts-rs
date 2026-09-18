//! HTML export capability.
//!
//! Provides the [`HtmlExport`] trait for font families that can produce
//! self-contained HTML pages (`@font-face` or inline SVG).

use crate::FontError;
use crate::HtmlBuilder;
use crate::IconThemeFont;
use base64::Engine;

/// Self-contained HTML export.
///
/// Supertrait [`IconThemeFont`] provides `glyph_svg` for the inline-SVG
/// mode. Families that deliberately avoid `@font-face` (Nerd Fonts) do
/// not implement this trait.
pub trait HtmlExport: IconThemeFont {
    /// Returns a `@font-face` CSS rule embedding the default-variant
    /// TTF as a base64 data URI. Low-level building block for custom
    /// HTML pages.
    ///
    /// # Errors
    ///
    /// Returns a [`FontError`] if registration, font loading, or reading
    /// the temp file fails.
    fn css_font_face() -> Result<String, FontError> {
        Self::ensure_registered()?;
        let font_path = Self::load_font()?;
        let font_bytes = std::fs::read(font_path)?;
        let font_b64 = base64::engine::general_purpose::STANDARD.encode(&font_bytes);
        Ok(format!(
            "@font-face {{\n    font-family: '{}';\n    src: url('data:font/ttf;base64,{font_b64}') format('truetype');\n}}",
            Self::CSS_FONT_FAMILY
        ))
    }

    /// Starts an [`HtmlBuilder`] for exporting a self-contained HTML
    /// page.
    fn html() -> HtmlBuilder<Self>
    where
        Self: Sized,
    {
        HtmlBuilder::new()
    }
}
