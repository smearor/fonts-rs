//! Runtime font family API for Line Awesome.
//!
//! Implements the family facade via `impl_font_family!` with the
//! generated glyph maps, the typed icon name, and the capabilities
//! Line Awesome supports: `icon_theme` (GResource SVGs) and `html`
//! (TTF via font.gresource / @font-face - the `html` capability
//! already includes `HtmlSvgExport` for inline-SVG export).
//!
//! `pango` is deliberately *not* a declared capability: the macro emits
//! the single-TTF `install_pango` default, but Line Awesome ships three
//! faces - a manual [`PangoFont`](fonts_rs_model::PangoFont) impl below
//! registers all of them. `SoftwareRender` (per-style `ab_glyph` faces)
//! is implemented here as well.

use crate::codepoint_map;
use crate::definition::GLYPH_PREFIX;
use crate::definition::LineAwesomeConfig;
use crate::definition::TTF_FAMILY_NAME;
use crate::naming::LineAwesome;
use crate::naming::LineAwesomeIconName;
use fonts_rs_model::FontVariant;

/// Shipped TTF faces: one Font Awesome 5 style per file.
const FONT_VARIANTS: &[FontVariant] = &[
    FontVariant::file("regular", "la-regular-400"),
    FontVariant::file("solid", "la-solid-900"),
    FontVariant::file("brands", "la-brands-400"),
];

fonts_rs_generator::impl_font_family!(
    LineAwesome,
    LineAwesomeConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = LineAwesomeIconName,
    css_font_family = TTF_FAMILY_NAME,
    glyph_prefix = GLYPH_PREFIX,
    gresource_prefix = LineAwesomeConfig::GRESOURCE_PREFIX,
    variants = FONT_VARIANTS,
    capabilities = [icon_theme, html],
);

#[cfg(feature = "gtk")]
impl fonts_rs_model::PangoFont for LineAwesome {
    fn install_pango(font_map: &fonts_rs_model::pango::FontMap) -> Result<(), fonts_rs_model::FontError> {
        Self::install_pango_resources(font_map, crate::gtk::FONT_RESOURCE_PATHS, env!("CARGO_PKG_NAME"))
    }
}

#[cfg(feature = "gtk")]
impl LineAwesome {
    /// Loads all three TTFs (regular, solid, brands) into `font_map`.
    ///
    /// Forwards to [`PangoFont::install_pango`](fonts_rs_model::PangoFont::install_pango).
    pub fn install_pango(font_map: &fonts_rs_model::pango::FontMap) -> Result<(), fonts_rs_model::FontError> {
        <Self as fonts_rs_model::PangoFont>::install_pango(font_map)
    }
}

#[cfg(feature = "render")]
impl fonts_rs_model::SoftwareRender for LineAwesome {
    type Variant = crate::style::LineAwesomeStyle;

    fn ab_glyph(style: Self::Variant) -> Option<&'static ab_glyph::FontVec> {
        crate::fonts::font(style)
    }
}

#[cfg(feature = "render")]
impl LineAwesome {
    /// Returns the cached `ab_glyph` font for the given style.
    ///
    /// Forwards to [`SoftwareRender::ab_glyph`](fonts_rs_model::SoftwareRender::ab_glyph).
    pub fn ab_glyph(style: crate::style::LineAwesomeStyle) -> Option<&'static ab_glyph::FontVec> {
        <Self as fonts_rs_model::SoftwareRender>::ab_glyph(style)
    }

    /// Returns the cached `ab_glyph` font for the default (solid) style.
    ///
    /// Forwards to [`SoftwareRender::font`](fonts_rs_model::SoftwareRender::font).
    pub fn font() -> Option<&'static ab_glyph::FontVec> {
        <Self as fonts_rs_model::SoftwareRender>::font()
    }
}

impl fonts_rs_model::UpstreamIcons for LineAwesome {
    const UPSTREAM_POST_PROCESSORS: &'static [fonts_rs_model::IconSetPostProcessor] = &[fonts_rs_model::IconSetPostProcessor::CurrentColorFill];
}
