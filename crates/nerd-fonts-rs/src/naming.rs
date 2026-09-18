//! Font family marker type for Nerd Fonts.
//!
//! The `NerdFonts` marker is used as the phantom type parameter in
//! `GlyphName<NerdFonts>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::sealed;

/// Marker type identifying Nerd Fonts in `GlyphName<NerdFonts>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<NerdFonts>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NerdFonts {}

impl FontFamilyMarker for NerdFonts {}

impl sealed::Sealed for NerdFonts {}
