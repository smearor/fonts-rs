//! GTK integration for Heroicons.
//!
//! Registers the Heroicons glyph GResource bundle. There is no
//! `font.gresource` - Heroicons is a fontless SVG-only set, so
//! `register_font()` is a no-op and `load_font()` returns
//! `NoFontGresource`.

use crate::naming::Heroicons;

fonts_rs_generator::impl_gtk_font!(Heroicons);
