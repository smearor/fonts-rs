//! Pango text rendering capability.
//!
//! Provides the [`PangoFont`] trait for font families shipping a TTF
//! that can be loaded into a Pango font map.

use crate::FontError;
use crate::FontFamily;
use crate::GtkFont;
use pango::prelude::FontMapExt;

/// Pango text rendering via the family's TTF(s).
///
/// Only implemented by families that ship a usable TTF - icons-only
/// families simply lack this impl.
pub trait PangoFont: FontFamily + GtkFont {
    /// Loads all TTFs of the family into `font_map`, enabling CSS
    /// `font-family: 'FAMILY_DISPLAY_NAME'` in GTK widgets.
    ///
    /// Default: [`load_font`](GtkFont::load_font) +
    /// `font_map.add_font_file()`. Families with several TTFs (Nerd
    /// Fonts, Line Awesome) override to install all of them - typically
    /// by delegating to [`install_pango_resources`](Self::install_pango_resources).
    ///
    /// # Errors
    ///
    /// Returns a [`FontError`] if registration, font loading, or the
    /// font map operation fails.
    fn install_pango(font_map: &pango::FontMap) -> Result<(), FontError> {
        Self::ensure_registered()?;
        let path = Self::load_font()?;
        font_map.add_font_file(path)?;
        Ok(())
    }

    /// Registers the family's bundles and installs every TTF listed in
    /// `font_resources` into `font_map`.
    ///
    /// Provided method for multi-TTF families: each GResource path
    /// (e.g. `"/io/smearor/fonts/line_awesome/la-solid-900.ttf"`) is
    /// extracted via [`GtkFont::load_font_from_gresource`] and added
    /// via `FontMapExt::add_font_file`. `temp_tag` (typically
    /// `env!("CARGO_PKG_NAME")` at the call site) namespaces the temp
    /// files. Implementors are not expected to override it.
    ///
    /// # Errors
    ///
    /// Returns the first [`FontError`] encountered during registration,
    /// extraction, or font map registration.
    fn install_pango_resources(font_map: &pango::FontMap, font_resources: &[&str], temp_tag: &str) -> Result<(), FontError> {
        Self::ensure_registered()?;
        for resource_path in font_resources {
            let path = Self::load_font_from_gresource(resource_path, temp_tag)?;
            font_map.add_font_file(path)?;
        }
        Ok(())
    }
}
