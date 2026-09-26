//! GTK integration for the Tabler Icons font family.
//!
//! Implements [`GtkFont`] for the [`Tabler`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Tabler, with_font_gresource);
