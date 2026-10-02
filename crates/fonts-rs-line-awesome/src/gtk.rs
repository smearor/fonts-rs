//! GTK integration for Line Awesome.
//!
//! Registers the glyph GResource bundle (`icons.gresource`) and the
//! font GResource bundle (`font.gresource`). The latter bundles all
//! three upstream TTFs (`la-regular-400`, `la-solid-900`,
//! `la-brands-400`) via the static `resources/font.gresource.xml`
//! manifest - the generated single-file path cannot express multi-TTF
//! families, so `GtkFont` is implemented manually (Nerd Fonts
//! precedent) instead of `impl_gtk_font!`.

use const_format::concatcp;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GtkFont;
use fonts_rs_model::LoadFontError;
use fonts_rs_model::RegisterFontError;
use fonts_rs_model::RegisterGlyphError;
use std::path::Path;

use crate::definition::LineAwesomeConfig;
use crate::naming::LineAwesome;

/// GResource paths of all three bundled TTFs inside `font.gresource`.
///
/// Consumed by `install_pango` (all faces) and
/// [`GtkFont::load_font`] (primary face only).
pub(crate) const FONT_RESOURCE_PATHS: &[&str] = &[
    concatcp!(LineAwesomeConfig::GRESOURCE_PREFIX, "/la-regular-400.ttf"),
    concatcp!(LineAwesomeConfig::GRESOURCE_PREFIX, "/la-solid-900.ttf"),
    concatcp!(LineAwesomeConfig::GRESOURCE_PREFIX, "/la-brands-400.ttf"),
];

/// GResource path of the primary TTF inside `font.gresource`.
///
/// The solid face is primary: it carries the majority of Line Awesome
/// icons (every FA5 icon has a solid style) and its family name is
/// `css_font_family` for @font-face embedding.
const FONT_RESOURCE_PATH: &str = FONT_RESOURCE_PATHS[1];

impl GtkFont for LineAwesome {
    fn register_glyphs() -> Result<(), RegisterGlyphError> {
        gio::resources_register_include!("icons.gresource")?;
        Ok(())
    }

    fn register_font() -> Result<(), RegisterFontError> {
        gio::resources_register_include!("font.gresource")?;
        Ok(())
    }

    fn load_font() -> Result<&'static Path, LoadFontError> {
        <Self as GtkFont>::register_font()?;
        Self::load_font_from_gresource(FONT_RESOURCE_PATH, env!("CARGO_PKG_NAME"))
    }
}
