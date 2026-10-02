//! Error type for [`GtkFont::register_font`](crate::GtkFont::register_font).

/// Error returned by [`GtkFont::register_font`](crate::GtkFont::register_font).
#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to register font GResource: {0}")]
pub struct RegisterFontError(#[from] gio::glib::Error);
