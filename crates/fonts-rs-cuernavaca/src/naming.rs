//! Font family marker type for Chess Cuernavaca.
//!
//! The `Cuernavaca` marker is used as the phantom type parameter in
//! `GlyphName<Cuernavaca>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Cuernavaca in `GlyphName<Cuernavaca>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Cuernavaca>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cuernavaca {}

impl FontFamily for Cuernavaca {}

impl sealed::Sealed for Cuernavaca {}

/// Convenience type alias for Cuernavaca glyph names.
pub type CuernavacaName = GlyphName<Cuernavaca>;
