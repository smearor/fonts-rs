//! Font loaders for Line Awesome software rendering.
//!
//! The three upstream TTF paths are determined at build time by
//! `build.rs` and exposed via the `LINE_AWESOME_{REGULAR,SOLID,
//! BRANDS}_FONT_PATH` environment variables for pixel-drawing
//! (ab_glyph). Each face is cached in its own `OnceLock`.

use std::sync::OnceLock;

use ab_glyph::FontVec;

use crate::style::LineAwesomeStyle;

static REGULAR_FONT: OnceLock<Option<FontVec>> = OnceLock::new();
static SOLID_FONT: OnceLock<Option<FontVec>> = OnceLock::new();
static BRANDS_FONT: OnceLock<Option<FontVec>> = OnceLock::new();

/// Reads the TTF bytes for `style` from the embedded blob
/// (`embed-fonts` feature) or from disk via the build-time
/// `cargo:rustc-env` path.
fn load_bytes(style: LineAwesomeStyle) -> Option<Vec<u8>> {
    #[cfg(feature = "embed-fonts")]
    {
        let data: &[u8] = match style {
            LineAwesomeStyle::Regular => include_bytes!(env!("LINE_AWESOME_REGULAR_FONT_PATH")).as_slice(),
            LineAwesomeStyle::Solid => include_bytes!(env!("LINE_AWESOME_SOLID_FONT_PATH")).as_slice(),
            LineAwesomeStyle::Brands => include_bytes!(env!("LINE_AWESOME_BRANDS_FONT_PATH")).as_slice(),
        };
        Some(data.to_vec())
    }
    #[cfg(not(feature = "embed-fonts"))]
    {
        let path = match style {
            LineAwesomeStyle::Regular => env!("LINE_AWESOME_REGULAR_FONT_PATH"),
            LineAwesomeStyle::Solid => env!("LINE_AWESOME_SOLID_FONT_PATH"),
            LineAwesomeStyle::Brands => env!("LINE_AWESOME_BRANDS_FONT_PATH"),
        };
        std::fs::read(path).ok()
    }
}

/// Returns the cached `ab_glyph` font for `style`, loading and parsing
/// the TTF on first access.
pub fn font(style: LineAwesomeStyle) -> Option<&'static FontVec> {
    let lock = match style {
        LineAwesomeStyle::Regular => &REGULAR_FONT,
        LineAwesomeStyle::Solid => &SOLID_FONT,
        LineAwesomeStyle::Brands => &BRANDS_FONT,
    };
    lock.get_or_init(|| load_bytes(style).and_then(|data| FontVec::try_from_vec(data).ok()))
        .as_ref()
}
