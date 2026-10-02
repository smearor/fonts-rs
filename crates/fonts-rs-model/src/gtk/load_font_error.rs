//! Error type for [`GtkFont::load_font`](crate::GtkFont::load_font).

use std::sync::Arc;

use crate::RegisterFontError;

/// Error returned by [`GtkFont::load_font`](crate::GtkFont::load_font).
#[derive(Debug, Clone, thiserror::Error)]
pub enum LoadFontError {
    /// Failed to register the font GResource bundle.
    #[error("failed to register font GResource: {0}")]
    RegisterFont(#[from] RegisterFontError),
    /// Failed to look up the font resource data.
    #[error("failed to look up font resource: {0}")]
    ResourceLookup(#[from] gio::glib::Error),
    /// Failed to write the font to a temporary file.
    /// Wrapped in `Arc` so `LoadFontError` stays `Clone`.
    #[error("failed to write font to temp file: {0}")]
    Io(#[from] Arc<std::io::Error>),
    /// No font GResource is available for this font family.
    #[error("no font GResource available for this font family")]
    NoFontGresource,
}

impl From<std::io::Error> for LoadFontError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(Arc::new(error))
    }
}
