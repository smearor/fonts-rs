//! GTK integration for the Doto dot-matrix font family.
//!
//! Implements [`GtkFont`] for the [`Doto`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Doto, with_font_gresource);
