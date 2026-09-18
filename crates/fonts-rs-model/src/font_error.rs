//! Unified error type for the high-level font API.
//!
//! [`FontError`] wraps the registration and loading error types at the
//! capability-trait surface, so callers handle a single error type.

use std::sync::Arc;

use crate::LoadFontError;
use crate::RegisterFontError;
use crate::RegisterGlyphError;

/// Error returned by font family capability methods.
///
/// `Clone` so that [`GtkFont::ensure_registered`](crate::GtkFont::ensure_registered)
/// can cache `Result<(), FontError>` in a `OnceLock` and hand out clones.
#[derive(Debug, Clone, thiserror::Error)]
pub enum FontError {
    /// Glyph GResource registration failed.
    #[error(transparent)]
    RegisterGlyphs(#[from] RegisterGlyphError),

    /// Font GResource registration failed.
    #[error(transparent)]
    RegisterFont(#[from] RegisterFontError),

    /// TTF could not be loaded from the GResource.
    #[error(transparent)]
    LoadFont(#[from] LoadFontError),

    /// Filesystem error during HTML export or temp-file creation.
    /// Wrapped in `Arc` so `FontError` stays `Clone`.
    #[error(transparent)]
    Io(#[from] Arc<std::io::Error>),

    /// GTK or Pango operation failed (init, font map, icon theme).
    #[error("GTK operation failed: {0}")]
    Gtk(#[from] gio::glib::Error),
}

impl From<std::io::Error> for FontError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(Arc::new(error))
    }
}
