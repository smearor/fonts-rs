//! GTK integration for the Dicefont dice face font family.
//!
//! Implements [`GtkFont`] for the [`Dicefont`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Dicefont, with_font_gresource);
