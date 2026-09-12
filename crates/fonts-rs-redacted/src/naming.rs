//! Font family marker type for Redacted.
//!
//! The `Redacted` marker is used as the phantom type parameter in
//! `GlyphName<Redacted>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamily;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Redacted in `GlyphName<Redacted>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Redacted>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Redacted {}

impl FontFamily for Redacted {}

impl sealed::Sealed for Redacted {}

/// Convenience type alias for Redacted glyph names.
pub type RedactedName = GlyphName<Redacted>;
