//! GTK integration for the Bravura SMuFL music notation font family.
//!
//! Implements [`GtkFont`] for the [`Bravura`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::Bravura, with_font_gresource);
