//! Manual [`GtkFont`] implementation for [`NerdFonts`].
//!
//! Unlike other font crates which use the `impl_gtk_font!` macro,
//! Nerd Fonts has a separate `compiled.gresource` bundle (containing
//! the TTF files) that is registered alongside the `icons.gresource`
//! bundle (containing SVG icons).

use std::sync::OnceLock;

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
        static FONT_TEMP_PATH: OnceLock<std::path::PathBuf> = OnceLock::new();

        if let Some(path) = FONT_TEMP_PATH.get() {
            return Ok(path);
        }

        <Self as GtkFont>::register_font()?;
        let bytes = gio::resources_lookup_data(FONT_RESOURCE_PATH, gio::ResourceLookupFlags::NONE)?;
        let temp_path = std::env::temp_dir().join(format!("{}-{}.ttf", env!("CARGO_PKG_NAME"), std::process::id()));
        std::fs::write(&temp_path, bytes.as_ref())?;

        let path = FONT_TEMP_PATH.get_or_init(|| temp_path);
        Ok(path)
    }
}
