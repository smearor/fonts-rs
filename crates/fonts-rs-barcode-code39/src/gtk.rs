//! GTK integration for the Code 39 font family.
//!
//! Implements [`GtkFont`] for the [`Code39`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::definition::Code39, with_font_gresource);
