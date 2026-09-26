//! Software rendering capability.
//!
//! Provides the [`SoftwareRender`] trait for font families loadable via
//! `ab_glyph` - the only capability trait without a `GtkFont` bound,
//! usable in pure-Rust renderers.

use crate::FontFamily;

/// Software rendering via `ab_glyph` (feature `render`).
///
/// Emitted per family by `impl_font_loader!` - the font loading itself
/// stays per-crate because the TTF source differs (`embed-fonts` vs.
/// `resources/` directory).
pub trait SoftwareRender: FontFamily {
    /// Variant selector - `()` for single-font families, a per-family
    /// enum for multi-TTF families (e.g. `NerdFontVariant`).
    type Variant: Copy + Default;

    /// Returns the cached `ab_glyph` font for `variant`.
    ///
    /// With `embed-fonts` the TTF is compiled into the binary; otherwise
    /// it is loaded from `resources/` relative to the working directory.
    fn ab_glyph(variant: Self::Variant) -> Option<&'static ab_glyph::FontVec>;

    /// Convenience: the default variant. For single-font families this
    /// is the only call needed (`Code39::font()`).
    fn font() -> Option<&'static ab_glyph::FontVec> {
        Self::ab_glyph(Self::Variant::default())
    }
}
