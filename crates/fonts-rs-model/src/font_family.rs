//! Core runtime trait for font families.
//!
//! [`FontFamily`] bundles the feature-free data surface every font family
//! has: the typed glyph name, family constants, and the glyph maps. Usage
//! modes (icon theme, Pango, HTML export, software rendering) live in the
//! capability traits [`IconThemeFont`](crate::IconThemeFont),
//! [`PangoFont`](crate::PangoFont), [`HtmlExport`](crate::HtmlExport), and
//! [`SoftwareRender`](crate::SoftwareRender).

use crate::CodePoint;
use crate::FontFamilyConfig;
use crate::FontFamilyMarker;
use crate::FontVariant;
use std::str::FromStr;

/// Runtime access to a font family: metadata and glyph maps.
///
/// Implemented once per font crate via `impl_font_family!`. The core
/// trait is feature-free - pure data access that works without GTK.
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
    ///
    /// `FromStr` lets implementations construct the typed name from the
    /// `&'static str` stored in the glyph maps; `AsRef<str>` lets lookup
    /// methods accept it alongside plain `&str` constants.
    type Name: AsRef<str> + FromStr;

    /// CSS `font-family` name, e.g. `"Libre Barcode Code 39"`.
    ///
    /// Defaults to the display name; override for families whose CSS
    /// name differs.
    const CSS_FONT_FAMILY: &'static str = Self::FAMILY_DISPLAY_NAME;

    /// Build-time variant metadata (see [`FontVariant`]). Empty for
    /// single-font families; lists the shipped TTF variants otherwise.
    /// Useful for UIs that offer a variant picker.
    const VARIANTS: &'static [FontVariant] = &[];

    /// Looks up the typed glyph name for a Unicode codepoint.
    ///
    /// Accepts both `char` and [`CodePoint`] via `Into<CodePoint>`.
    fn glyph_name(ch: impl Into<CodePoint>) -> Option<Self::Name>;

    /// Looks up the codepoint for a glyph name.
    ///
    /// Accepts `&str`, `String`, and the typed [`Self::Name`] via
    /// `AsRef<str>` - both `constants::UNI002A` and `GlyphName<F>` work.
    fn codepoint(name: impl AsRef<str>) -> Option<CodePoint>;

    /// Iterates all `(codepoint, glyph_name)` pairs.
    ///
    /// Yields the typed [`Self::Name`], consistent with
    /// [`glyph_name`](Self::glyph_name). Use [`AsRef<str>`] on the name
    /// when plain `&str` access is enough.
    fn all_glyphs() -> impl Iterator<Item = (CodePoint, Self::Name)>;

    /// Returns the number of glyphs in this family.
    ///
    /// Default: `all_glyphs().count()` - O(n). Implementations backed
    /// by a `phf::Map` override this with `map.len()` - O(1).
    fn glyph_count() -> usize {
        Self::all_glyphs().count()
    }
}
