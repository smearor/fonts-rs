//! GTK integration for the Blobmoji font family.
//!
//! Implements [`GtkFont`] for the [`Blobmoji`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Blobmoji, with_font_gresource);
