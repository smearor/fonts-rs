//! GTK font registration trait.
//!
//! Provides the [`GtkFont`] trait for font families with GResource-backed
//! registration. Each font crate implements this trait on its marker type
//! via the `impl_gtk_font!` macro in `fonts-rs-generator`.

use crate::FontError;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

/// Error returned by [`GtkFont::register_glyphs`].
#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to register icons GResource: {0}")]
pub struct RegisterGlyphError(#[from] gio::glib::Error);

/// Error returned by [`GtkFont::register_font`].
#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to register font GResource: {0}")]
pub struct RegisterFontError(#[from] gio::glib::Error);

/// Error returned by [`GtkFont::load_font`].
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
/// - [`ensure_registered`](Self::ensure_registered): Lazy, idempotent
///   registration of all the family's GResource bundles, cached per
///   family. Called automatically by every capability method.
/// - [`install`](Self::install): Explicit fail-fast setup for application
///   startup.
///
/// The `'static` supertrait allows [`ensure_registered`](Self::ensure_registered)
/// to key its global cache by `TypeId::of::<Self>()`.
pub trait GtkFont: 'static {
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

    /// Idempotent registration of all GResource bundles this family has.
    ///
    /// Called lazily by every capability method. The result is cached per
    /// family in a global `OnceLock` keyed by `TypeId`: subsequent calls
    /// return the cached `Ok` or the cached error. Safe to call from any
    /// thread.
    ///
    /// A `static OnceLock` inside this default body would be shared
    /// across *all* impls, so the cache is a `HashMap` keyed by
    /// `TypeId::of::<Self>()` instead — one entry per family, no
    /// macro-generated statics required.
    ///
    /// # Errors
    ///
    /// Returns the cached [`FontError`] if registration failed.
    fn ensure_registered() -> Result<(), FontError> {
        static CACHE: OnceLock<Mutex<HashMap<TypeId, Result<(), FontError>>>> = OnceLock::new();
        let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = cache.lock().unwrap_or_else(|e| e.into_inner());
        guard.entry(TypeId::of::<Self>()).or_insert_with(|| Self::do_register()).clone()
    }

    /// Explicit fail-fast setup for application startup.
    ///
    /// Equivalent to [`ensure_registered`](Self::ensure_registered) —
    /// exists to make intent visible at the call site
    /// (`Code39::install()?` in `main()`).
    ///
    /// # Errors
    ///
    /// Returns the cached [`FontError`] if registration failed.
    fn install() -> Result<(), FontError> {
        Self::ensure_registered()
    }

    /// Performs the actual registration of all the family's GResource
    /// bundles. Called once per family by
    /// [`ensure_registered`](Self::ensure_registered).
    ///
    /// Default: [`register_glyphs`](Self::register_glyphs) +
    /// [`register_font`](Self::register_font). Families with additional
    /// bundles override this method.
    #[doc(hidden)]
    fn do_register() -> Result<(), FontError> {
        Self::register_glyphs()?;
        Self::register_font()?;
        Ok(())
    }
}
