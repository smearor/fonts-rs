//! Font family marker type for Doto.
//!
//! The `Doto` marker is used as the phantom type parameter in
//! `GlyphName<Doto>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Doto in `GlyphName<Doto>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Doto>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Doto {}

impl FontFamily for Doto {}

impl sealed::Sealed for Doto {}

/// Convenience type alias for Doto glyph names.
pub type DotoName = GlyphName<Doto>;
