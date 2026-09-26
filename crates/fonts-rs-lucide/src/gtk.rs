//! GTK integration for Lucide Icons.
//!
//! Registers the Lucide glyph GResource bundle. There is no
//! `font.gresource` - Lucide is a fontless SVG-only set, so
//! `register_font()` is a no-op and `load_font()` returns
//! `NoFontGresource`.

use crate::naming::Lucide;

fonts_rs_generator::impl_gtk_font!(Lucide);
