//! GTK integration for the Redacted placeholder font family.
//!
//! Implements [`GtkFont`] for the [`Redacted`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Redacted, with_font_gresource);
