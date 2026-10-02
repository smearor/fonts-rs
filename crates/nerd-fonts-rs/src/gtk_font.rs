//! Manual [`GtkFont`] implementation for [`NerdFonts`].
//!
//! Unlike other font crates which use the `impl_gtk_font!` macro,
//! Nerd Fonts has a separate `compiled.gresource` bundle (containing
//! the TTF files) that is registered alongside the `icons.gresource`
//! bundle (containing SVG icons).

use const_format::concatcp;
use fonts_rs_model::GtkFont;
use fonts_rs_model::LoadFontError;
use fonts_rs_model::RegisterFontError;
use fonts_rs_model::RegisterGlyphError;
use nerd_fonts_model::paths::GRESOURCE_PREFIX;

use crate::naming::NerdFonts;

/// GResource path for the Symbols Nerd Font TTF file.
const FONT_RESOURCE_PATH: &str = concatcp!(GRESOURCE_PREFIX, "/SymbolsNerdFont-Regular.ttf");

impl GtkFont for NerdFonts {
    fn register_glyphs() -> Result<(), RegisterGlyphError> {
        gio::resources_register_include!("icons.gresource")?;
        Ok(())
    }

    fn register_font() -> Result<(), RegisterFontError> {
        gio::resources_register_include!("compiled.gresource")?;
        Ok(())
    }

    fn load_font() -> Result<&'static std::path::Path, LoadFontError> {
        <Self as GtkFont>::register_font()?;
        Self::load_font_from_gresource(FONT_RESOURCE_PATH, env!("CARGO_PKG_NAME"))
    }
}
