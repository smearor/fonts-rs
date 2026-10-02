//! The [`GtkFont`] trait: GResource-backed font family registration.

use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::FontError;
use crate::LoadFontError;
use crate::RegisterFontError;
use crate::RegisterGlyphError;

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
/// - [`load_font_from_gresource`](Self::load_font_from_gresource):
///   Provided helper - shared `load_font()` body for families whose
///   font GResource bundles one or more TTFs.
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
    /// `TypeId::of::<Self>()` instead - one entry per family, no
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
    /// Equivalent to [`ensure_registered`](Self::ensure_registered) -
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

    /// Extracts one TTF from the registered font GResource into a
    /// process-lifetime temp file and returns the cached path.
    ///
    /// Shared implementation of the [`load_font`](Self::load_font)
    /// bodies of multi-TTF families: [`register_font`](Self::register_font)
    /// must have succeeded already (the caller's `load_font` takes care
    /// of it). Results are cached per `resource_path`; `temp_tag`
    /// (typically `env!("CARGO_PKG_NAME")` at the call site) namespaces
    /// the temp file so faces of different families never collide.
    ///
    /// Provided method - implementors are not expected to override it.
    ///
    /// # Errors
    ///
    /// Returns a [`LoadFontError`] if the resource lookup or the
    /// temp-file write fails.
    fn load_font_from_gresource(resource_path: &str, temp_tag: &str) -> Result<&'static std::path::Path, LoadFontError> {
        static CACHE: OnceLock<Mutex<HashMap<String, &'static std::path::Path>>> = OnceLock::new();
        let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = cache.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(path) = guard.get(resource_path) {
            return Ok(path);
        }
        let bytes = gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE)?;
        let stem = std::path::Path::new(resource_path).file_stem().and_then(|s| s.to_str()).unwrap_or("font");
        let temp_path = std::env::temp_dir().join(format!("fonts-rs-{temp_tag}-{stem}-{}.ttf", std::process::id()));
        std::fs::write(&temp_path, bytes.as_ref())?;
        // Leak the path: the temp file must live for the process
        // lifetime (pango/ab_glyph read it on demand), so a shared
        // `&'static Path` cache entry is intentional.
        let path: &'static std::path::Path = Box::leak(temp_path.into_boxed_path());
        guard.insert(resource_path.to_string(), path);
        Ok(path)
    }
}
