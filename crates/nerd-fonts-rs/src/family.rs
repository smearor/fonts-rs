//! Runtime font family API for Nerd Fonts.
//!
//! Manual implementations of the [`FontFamily`] core trait and the
//! capability traits - Nerd Fonts does not use the `impl_font_family!`
//! macro because its build pipeline differs from the generated font
//! crates (vendored icon GResource, two TTFs installed into the Pango
//! font map). Icons follow the standard `{prefix}/scalable/glyphs`
//! resource layout.
//!
//! [`HtmlExport`](fonts_rs_model::HtmlExport) is deliberately not
//! implemented: Nerd Fonts avoids `@font-face` because the GTK4 CSS
//! parser does not reliably load fonts from `@font-face` rules.

use crate::definition::NerdFontsConfig;
use crate::icons::codepoint_map;
use crate::naming::NerdFonts;
#[cfg(feature = "render")]
use crate::variant::NerdFontVariant;
use fonts_rs_model::CodePointRange;
#[cfg(feature = "gtk")]
use fonts_rs_model::FontError;
use fonts_rs_model::FontFamily;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::FontVariant;
#[cfg(feature = "gtk")]
use fonts_rs_model::IconThemeFont;
#[cfg(feature = "gtk")]
use fonts_rs_model::PangoFont;
#[cfg(feature = "render")]
use fonts_rs_model::SoftwareRender;
use nerd_fonts_model::IconName;
use nerd_fonts_model::paths::ICONS_RESOURCE_PATH;

impl FontFamilyConfig for NerdFonts {
    const FONT_FAMILY_NAME: &'static str = NerdFontsConfig::FONT_FAMILY_NAME;

    const FAMILY_DISPLAY_NAME: &'static str = NerdFontsConfig::FAMILY_DISPLAY_NAME;

    const GRESOURCE_PREFIX: &'static str = NerdFontsConfig::GRESOURCE_PREFIX;

    const CODEPOINT_RANGES: &[CodePointRange] = NerdFontsConfig::CODEPOINT_RANGES;

    /// Nerd Fonts icon names carry the `nf-` prefix.
    const ICON_PREFIX: &'static str = NerdFontsConfig::ICON_PREFIX;

    /// Delegates to [`NerdFontsConfig::is_valid_icon_name`] so
    /// `IconName<NerdFonts>` validates identically to the canonical
    /// `IconName<NerdFontsConfig>`.
    fn is_valid_icon_name(name: &str) -> bool {
        NerdFontsConfig::is_valid_icon_name(name)
    }

    /// Nerd Font icons follow the standard
    /// `{GRESOURCE_PREFIX}/scalable/glyphs` layout.
    fn icons_resource_prefix() -> String {
        ICONS_RESOURCE_PATH.to_string()
    }
}

impl FontFamily for NerdFonts {
    type Name = IconName;

    /// CSS `font-family` of the symbols-only icon font.
    const CSS_FONT_FAMILY: &'static str = "NerdFontsSymbolsOnly";

    /// The shipped font files: symbols regular + mono, plus the
    /// JetBrains Mono label font (mirrors [`NerdFontVariant`]).
    const VARIANTS: &'static [FontVariant] = &[
        FontVariant::file("symbols", "SymbolsNerdFont-Regular"),
        FontVariant::file("symbols-mono", "SymbolsNerdFontMono-Regular"),
        FontVariant::file("label", "JetBrainsMonoNLNerdFont-Regular"),
    ];

    fn glyph_name(ch: impl Into<fonts_rs_model::CodePoint>) -> Option<Self::Name> {
        let ch = ch.into().as_char();
        codepoint_map::ICONS.get(&ch).and_then(|name| IconName::parse(name))
    }

    fn codepoint(name: impl AsRef<str>) -> Option<fonts_rs_model::CodePoint> {
        let icon = IconName::parse(name.as_ref())?;
        codepoint_map::REVERSE_ICONS
            .get(icon.as_ref())
            .copied()
            .map(fonts_rs_model::CodePoint::from_char)
    }

    fn all_glyphs() -> impl Iterator<Item = (fonts_rs_model::CodePoint, Self::Name)> {
        crate::icons::all_icons().filter_map(|(ch, name)| IconName::parse(name).map(|name| (fonts_rs_model::CodePoint::from_char(ch), name)))
    }

    fn glyph_count() -> usize {
        codepoint_map::ICONS.len()
    }
}

#[cfg(feature = "gtk")]
impl IconThemeFont for NerdFonts {}

#[cfg(feature = "gtk")]
impl PangoFont for NerdFonts {
    /// Installs both Nerd Font TTFs (symbols regular + mono) into
    /// `font_map` via the shared `font_loader` implementation.
    fn install_pango(font_map: &pango::FontMap) -> Result<(), FontError> {
        crate::font_loader::install_fonts(font_map).map_err(|e| FontError::from(std::io::Error::other(e.to_string())))
    }
}

#[cfg(feature = "render")]
impl SoftwareRender for NerdFonts {
    type Variant = NerdFontVariant;

    fn ab_glyph(variant: Self::Variant) -> Option<&'static ab_glyph::FontVec> {
        match variant {
            NerdFontVariant::Symbols => crate::fonts::nerd_font(),
            NerdFontVariant::SymbolsMono => crate::fonts::mono_font(),
            NerdFontVariant::Label => crate::fonts::label_font(),
        }
    }
}

impl NerdFonts {
    /// Resolves an icon name to its Unicode codepoint.
    ///
    /// Accepts all [`IconName`] forms (`fa-gamepad`, `nf-fa-gamepad`,
    /// `nf-fa-gamepad-symbolic`) plus `IconName` itself via `AsRef<str>`.
    /// Forwards to [`FontFamily::codepoint`](fonts_rs_model::FontFamily::codepoint).
    pub fn codepoint(name: impl AsRef<str>) -> Option<fonts_rs_model::CodePoint> {
        <Self as FontFamily>::codepoint(name)
    }

    /// Resolves a Unicode codepoint to its icon name.
    ///
    /// Accepts `char` and `CodePoint` via `Into<CodePoint>`.
    /// Forwards to [`FontFamily::glyph_name`](fonts_rs_model::FontFamily::glyph_name).
    pub fn glyph_name(ch: impl Into<fonts_rs_model::CodePoint>) -> Option<IconName> {
        <Self as FontFamily>::glyph_name(ch)
    }

    /// Returns an iterator over all known icons, yielding
    /// `(CodePoint, IconName)` pairs of codepoint and typed icon name.
    ///
    /// Forwards to [`FontFamily::all_glyphs`](fonts_rs_model::FontFamily::all_glyphs).
    pub fn all_glyphs() -> impl Iterator<Item = (fonts_rs_model::CodePoint, IconName)> {
        <Self as FontFamily>::all_glyphs()
    }

    /// Returns the number of icons in this family - O(1).
    ///
    /// Forwards to [`FontFamily::glyph_count`](fonts_rs_model::FontFamily::glyph_count).
    pub fn glyph_count() -> usize {
        <Self as FontFamily>::glyph_count()
    }
}

#[cfg(feature = "gtk")]
impl NerdFonts {
    /// Explicit fail-fast setup for application startup.
    ///
    /// Forwards to [`GtkFont::install`](fonts_rs_model::GtkFont::install).
    pub fn install() -> Result<(), FontError> {
        <Self as fonts_rs_model::GtkFont>::install()
    }

    /// Idempotent registration of all GResource bundles.
    ///
    /// Forwards to [`GtkFont::ensure_registered`](fonts_rs_model::GtkFont::ensure_registered).
    pub fn ensure_registered() -> Result<(), FontError> {
        <Self as fonts_rs_model::GtkFont>::ensure_registered()
    }

    /// Registers the icon GResource and adds the icon resource path to
    /// the `GtkIconTheme` of `display`.
    ///
    /// Forwards to [`IconThemeFont::install_icon_theme`](fonts_rs_model::IconThemeFont::install_icon_theme).
    pub fn install_icon_theme(display: &gtk4::gdk::Display) -> Result<(), FontError> {
        <Self as IconThemeFont>::install_icon_theme(display)
    }

    /// The icon theme resource path (`{GRESOURCE_PREFIX}/icons`).
    ///
    /// Forwards to [`IconThemeFont::icon_theme_path`](fonts_rs_model::IconThemeFont::icon_theme_path).
    pub fn icon_theme_path() -> String {
        <Self as IconThemeFont>::icon_theme_path()
    }

    /// Returns the SVG document for one icon.
    ///
    /// Forwards to [`IconThemeFont::glyph_svg`](fonts_rs_model::IconThemeFont::glyph_svg).
    pub fn glyph_svg(name: impl AsRef<str>) -> Option<String> {
        <Self as IconThemeFont>::glyph_svg(name)
    }

    /// Composes `text` into a single SVG document.
    ///
    /// Forwards to [`IconThemeFont::compose_svg`](fonts_rs_model::IconThemeFont::compose_svg).
    pub fn compose_svg(text: &str, glyph_height: f64) -> String {
        <Self as IconThemeFont>::compose_svg(text, glyph_height)
    }

    /// Loads both Nerd Font TTFs (symbols regular + mono) into `font_map`.
    ///
    /// Forwards to [`PangoFont::install_pango`](fonts_rs_model::PangoFont::install_pango).
    pub fn install_pango(font_map: &pango::FontMap) -> Result<(), FontError> {
        <Self as PangoFont>::install_pango(font_map)
    }
}

#[cfg(feature = "render")]
impl NerdFonts {
    /// Returns the cached `ab_glyph` font for `variant`.
    ///
    /// Forwards to [`SoftwareRender::ab_glyph`](fonts_rs_model::SoftwareRender::ab_glyph).
    pub fn ab_glyph(variant: NerdFontVariant) -> Option<&'static ab_glyph::FontVec> {
        <Self as SoftwareRender>::ab_glyph(variant)
    }

    /// Returns the cached symbols-only `ab_glyph` font (default variant).
    ///
    /// Forwards to [`SoftwareRender::font`](fonts_rs_model::SoftwareRender::font).
    pub fn font() -> Option<&'static ab_glyph::FontVec> {
        <Self as SoftwareRender>::font()
    }
}
