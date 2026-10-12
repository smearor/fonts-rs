//! GTK integration for the Noto Emoji font family.
//!
//! Implements [`GtkFont`] for the [`NotoEmoji`] marker type, providing
//! GResource registration and font loading via the `impl_gtk_font!` macro.

fonts_rs_generator::impl_gtk_font!(crate::naming::NotoEmoji, with_font_gresource);
