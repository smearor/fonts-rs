//! GTK integration for the Code 128 font family.
//!
//! Implements [`GtkFont`] for the [`Code128`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::definition::Code128, with_font_gresource);
