//! Error type for [`GtkFont::register_glyphs`](crate::GtkFont::register_glyphs).

/// Error returned by [`GtkFont::register_glyphs`](crate::GtkFont::register_glyphs).
#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to register icons GResource: {0}")]
pub struct RegisterGlyphError(#[from] gio::glib::Error);
