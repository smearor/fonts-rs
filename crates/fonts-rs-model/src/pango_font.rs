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
/// Only implemented by families that ship a usable TTF — icons-only
/// families simply lack this impl.
pub trait PangoFont: FontFamily + GtkFont {
    /// Loads all TTFs of the family into `font_map`, enabling CSS
    /// `font-family: 'FAMILY_DISPLAY_NAME'` in GTK widgets.
    ///
    /// Default: [`load_font`](GtkFont::load_font) +
    /// `font_map.add_font_file()`. Families with several TTFs (Nerd
    /// Fonts) override to install all of them.
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
}
