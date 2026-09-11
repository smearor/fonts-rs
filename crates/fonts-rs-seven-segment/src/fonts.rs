//! Font loading with `ab_glyph` for software rendering.
//!
//! The DSEG7 Classic font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.

use std::sync::OnceLock;

use ab_glyph::FontVec;

/// Relative path to the DSEG7 Classic TTF file.
#[cfg(not(feature = "embed-fonts"))]
const FONT_RELATIVE: &str = "resources/DSEG7Classic-Regular.ttf";

/// Cached DSEG7 Classic loaded from disk or embedded.
static FONT: OnceLock<Option<FontVec>> = OnceLock::new();

/// Get the cached DSEG7 Classic font, loading from disk on first access.
pub fn font() -> Option<&'static FontVec> {
    FONT.get_or_init(|| {
        #[cfg(feature = "embed-fonts")]
        {
            let data = include_bytes!("../resources/DSEG7Classic-Regular.ttf").to_vec();
            FontVec::try_from_vec(data).ok()
        }
        #[cfg(not(feature = "embed-fonts"))]
        {
            std::fs::read(FONT_RELATIVE)
                .ok()
                .and_then(|data| FontVec::try_from_vec(data).ok())
        }
    })
    .as_ref()
}
