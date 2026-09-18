//! Core runtime trait for font families.
//!
//! [`FontFamily`] bundles the feature-free data surface every font family
//! has: the typed glyph name, family constants, and the glyph maps. Usage
//! modes (icon theme, Pango, HTML export, software rendering) live in the
//! capability traits [`IconThemeFont`](crate::IconThemeFont),
//! [`PangoFont`](crate::PangoFont), [`HtmlExport`](crate::HtmlExport), and
//! [`SoftwareRender`](crate::SoftwareRender).

use crate::FontFamilyConfig;
use crate::FontFamilyMarker;
use crate::FontVariant;

/// Runtime access to a font family: metadata and glyph maps.
///
/// Implemented once per font crate via `impl_font_family!`. The core
/// trait is feature-free — pure data access that works without GTK.
///
/// # Usage Modes
///
/// | User wants… | Trait |
/// |---|---|
/// | Icons via `GtkIconTheme` / `Image::from_icon_name` | [`IconThemeFont`](crate::IconThemeFont) |
/// | Text in GTK widgets (Pango, CSS `font-family`) | [`PangoFont`](crate::PangoFont) |
/// | One glyph as SVG / `gdk::Texture` | [`IconThemeFont`](crate::IconThemeFont) |
/// | Text composed into a single SVG | [`IconThemeFont`](crate::IconThemeFont) |
/// | HTML page with `@font-face` or inline SVG | [`HtmlExport`](crate::HtmlExport) |
/// | Software rendering without GTK | [`SoftwareRender`](crate::SoftwareRender) |
/// | Fail-fast setup at app start | [`GtkFont::install`](crate::GtkFont::install) |
pub trait FontFamily: FontFamilyMarker + FontFamilyConfig {
    /// Typed glyph name of this family (e.g. `GlyphName<Code39>`,
    /// `IconName` for Nerd Fonts).
    type Name: AsRef<str>;

    /// CSS `font-family` name, e.g. `"Libre Barcode Code 39"`.
    ///
    /// Defaults to the display name; override for families whose CSS
    /// name differs.
    const CSS_FONT_FAMILY: &'static str = Self::FAMILY_DISPLAY_NAME;

    /// Build-time variant metadata (see [`FontVariant`]). Empty for
    /// single-font families; lists the shipped TTF variants otherwise.
    /// Useful for UIs that offer a variant picker.
    const VARIANTS: &'static [FontVariant] = &[];

    /// Looks up the glyph name for a Unicode codepoint.
    fn glyph_name(ch: char) -> Option<&'static str>;

    /// Looks up the codepoint for a glyph name.
    fn codepoint(name: &str) -> Option<char>;

    /// Iterates all `(codepoint, glyph_name)` pairs.
    fn all_glyphs() -> impl Iterator<Item = (char, &'static str)>;
}
