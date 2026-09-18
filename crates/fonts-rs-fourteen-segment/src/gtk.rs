//! GTK integration for the DSEG14 fourteen-segment font family.
//!
//! Implements [`GtkFont`] for the [`FourteenSegment`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::FourteenSegment, with_font_gresource);
