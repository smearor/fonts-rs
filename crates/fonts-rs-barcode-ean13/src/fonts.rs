//! Font loading with `ab_glyph` for software rendering.
//!
//! The EAN13 font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.

use std::sync::OnceLock;

use ab_glyph::FontVec;

/// Cached EAN13 font loaded from disk or embedded.
static FONT: OnceLock<Option<FontVec>> = OnceLock::new();

/// Get the cached EAN13 font, loading from disk on first access.
pub fn font() -> Option<&'static FontVec> {
    FONT.get_or_init(|| {
        #[cfg(feature = "embed-fonts")]
        {
            let data = include_bytes!("../resources/LibreBarcodeEAN13Text-Regular.ttf").to_vec();
            FontVec::try_from_vec(data).ok()
        }
        #[cfg(not(feature = "embed-fonts"))]
        {
            let path = std::env::current_dir()
                .ok()
                .map(|d| d.join("resources/LibreBarcodeEAN13Text-Regular.ttf"));
            path.and_then(|p| std::fs::read(p).ok())
                .and_then(|data| FontVec::try_from_vec(data).ok())
        }
    })
    .as_ref()
}
