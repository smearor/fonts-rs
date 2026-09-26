//! GTK integration for the Eva Icons font family.
//!
//! Implements [`GtkFont`] for the [`Eva`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Eva, with_font_gresource);
