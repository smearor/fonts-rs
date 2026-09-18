//! GTK integration for the EAN13 font family.
//!
//! Implements [`GtkFont`] for the [`Ean13`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::definition::Ean13, with_font_gresource);
