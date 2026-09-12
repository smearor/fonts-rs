//! Font family marker type for Bravura.
//!
//! The `Bravura` marker is used as the phantom type parameter in
//! `GlyphName<Bravura>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Bravura in `GlyphName<Bravura>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Bravura>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Bravura {}

impl FontFamily for Bravura {}

impl sealed::Sealed for Bravura {}

/// Convenience type alias for Bravura glyph names.
pub type BravuraName = GlyphName<Bravura>;
