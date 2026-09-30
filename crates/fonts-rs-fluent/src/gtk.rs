//! GTK integration for Fluent UI System Icons.
//!
//! Registers the glyph GResource bundle and the bundled TTF via
//! `font.gresource` (`with_font_gresource`).

use crate::naming::Fluent;

fonts_rs_generator::impl_gtk_font!(Fluent, with_font_gresource);
