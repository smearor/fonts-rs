//! GTK font registration trait.
//!
//! Provides the [`GtkFont`] trait for font families with GResource-backed
//! registration. Each font crate implements this trait on its marker type
//! via the `impl_gtk_font!` macro in `fonts-rs-generator`.

/// Error returned by [`GtkFont::register_glyphs`].
#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to register icons GResource: {0}")]
pub struct RegisterGlyphError(#[from] gio::glib::Error);

/// Error returned by [`GtkFont::register_font`].
#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to register font GResource: {0}")]
pub struct RegisterFontError(#[from] gio::glib::Error);

/// Error returned by [`GtkFont::load_font`].
#[derive(Debug, thiserror::Error)]
pub enum LoadFontError {
    /// Failed to register the font GResource bundle.
    #[error("failed to register font GResource: {0}")]
    RegisterFont(#[from] RegisterFontError),
    /// Failed to look up the font resource data.
    #[error("failed to look up font resource: {0}")]
    ResourceLookup(#[from] gio::glib::Error),
    /// Failed to write the font to a temporary file.
    #[error("failed to write font to temp file: {0}")]
    Io(#[from] std::io::Error),
    /// No font GResource is available for this font family.
    #[error("no font GResource available for this font family")]
    NoFontGresource,
}

/// Trait for GTK font families with GResource-backed registration.
///
/// Implemented by font family marker types via the `impl_gtk_font!` macro
/// in `fonts-rs-generator`. Each method is a static (no `&self`) method
/// callable as e.g. `Ean13::register_glyphs()`.
///
/// # Methods
///
/// - [`register_glyphs`](Self::register_glyphs): Registers the SVG icon
///   GResource bundle (`icons.gresource`).
/// - [`register_font`](Self::register_font): Registers the TTF font
///   GResource bundle (`font.gresource`). No-op for fonts without a TTF
///   GResource.
/// - [`load_font`](Self::load_font): Loads the TTF from the GResource into
///   a temp file and returns the path. Returns an error for fonts without
///   a TTF GResource.
pub trait GtkFont {
    /// Registers embedded SVG glyphs as a GResource.
    ///
    /// # Errors
    ///
    /// Returns a [`RegisterGlyphError`] if resource registration fails.
    fn register_glyphs() -> Result<(), RegisterGlyphError>;

    /// Registers the TTF font as a GResource.
    ///
    /// No-op for font families without a `font.gresource` bundle.
    ///
    /// # Errors
    ///
    /// Returns a [`RegisterFontError`] if resource registration fails.
    fn register_font() -> Result<(), RegisterFontError>;

    /// Loads the TTF font from the registered GResource into a temp file
    /// and returns the path.
    ///
    /// The temp file is kept alive for the process lifetime. Use the
    /// returned path with `FontMapExt::add_font_file`.
    ///
    /// # Errors
    ///
    /// Returns a [`LoadFontError`] if the font cannot be registered,
    /// looked up, or written.
    fn load_font() -> Result<&'static std::path::Path, LoadFontError>;
}
