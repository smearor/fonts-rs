//! Font loading with `ab_glyph` for software rendering.
//!
//! The DSEG7 font is loaded from disk or embedded via
//! `embed-fonts` feature and cached in `OnceLock` for the process lifetime.
//!
//! The active variant's TTF path is determined at build time by `build.rs`
//! and exposed via the `DSEG7_FONT_PATH` environment variable.

use std::sync::OnceLock;

use ab_glyph::FontVec;

/// Cached DSEG7 font loaded from disk or embedded.
static FONT: OnceLock<Option<FontVec>> = OnceLock::new();

/// Get the cached DSEG7 font, loading from disk on first access.
pub fn font() -> Option<&'static FontVec> {
    FONT.get_or_init(|| {
        #[cfg(feature = "embed-fonts")]
        {
            let data = include_bytes!(env!("DSEG7_FONT_PATH")).to_vec();
            FontVec::try_from_vec(data).ok()
        }
        #[cfg(not(feature = "embed-fonts"))]
        {
            std::fs::read(env!("DSEG7_FONT_PATH"))
                .ok()
                .and_then(|data| FontVec::try_from_vec(data).ok())
        }
    })
    .as_ref()
}
