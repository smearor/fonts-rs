//! `@font-face` HTML export capability.
//!
//! Provides the [`HtmlExport`] trait for font families that can produce
//! self-contained HTML pages embedding their TTF as a base64 data URI.
//! Inline-SVG export (no font required) lives in the base trait
//! [`HtmlSvgExport`](crate::HtmlSvgExport).

use crate::FontError;
use crate::HtmlSvgExport;
use base64::Engine;

/// Self-contained `@font-face` HTML export.
///
/// Extends [`HtmlSvgExport`] with the font-based export mode: the
/// default-variant TTF is embedded via
/// [`GtkFont::load_font`](crate::GtkFont::load_font). Families that
/// deliberately avoid `@font-face` (Nerd Fonts) or ship no TTF at all
/// (fontless icon sets) do not implement this trait - they can still
/// implement [`HtmlSvgExport`] alone.
pub trait HtmlExport: HtmlSvgExport {
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
}
