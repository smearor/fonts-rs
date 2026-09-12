//! Font family marker type for DSEG7.
//!
//! The `SevenSegment` marker is used as the phantom type parameter in
//! `GlyphName<SevenSegment>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying DSEG7 in `GlyphName<SevenSegment>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<SevenSegment>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SevenSegment {}

impl FontFamily for SevenSegment {}

impl sealed::Sealed for SevenSegment {}

/// Convenience type alias for seven-segment glyph names.
pub type SevenSegmentName = GlyphName<SevenSegment>;
