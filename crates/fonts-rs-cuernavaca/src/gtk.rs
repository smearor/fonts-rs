//! GTK integration for the Cuernavaca chess font family.
//!
//! Implements [`GtkFont`] for the [`Cuernavaca`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Cuernavaca, with_font_gresource);
