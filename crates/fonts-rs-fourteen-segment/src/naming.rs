//! Font family marker type for DSEG14.
//!
//! The `FourteenSegment` marker is used as the phantom type parameter in
//! `GlyphName<FourteenSegment>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying DSEG14 in `GlyphName<FourteenSegment>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<FourteenSegment>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FourteenSegment {}

impl FontFamily for FourteenSegment {}

impl sealed::Sealed for FourteenSegment {}

/// Convenience type alias for fourteen-segment glyph names.
pub type FourteenSegmentName = GlyphName<FourteenSegment>;
