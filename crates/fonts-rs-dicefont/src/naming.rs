//! Font family marker type for Dicefont.
//!
//! The `Dicefont` marker is used as the phantom type parameter in
//! `GlyphName<Dicefont>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Dicefont in `GlyphName<Dicefont>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Dicefont>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dicefont {}

impl FontFamily for Dicefont {}

impl sealed::Sealed for Dicefont {}

/// Convenience type alias for Dicefont glyph names.
pub type DicefontName = GlyphName<Dicefont>;
