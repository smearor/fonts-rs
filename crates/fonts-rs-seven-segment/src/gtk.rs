//! GTK integration for the DSEG7 seven-segment font family.
//!
//! Implements [`GtkFont`] for the [`SevenSegment`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::SevenSegment, with_font_gresource);
