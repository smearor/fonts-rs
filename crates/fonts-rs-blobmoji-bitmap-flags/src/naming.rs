//! Font family marker type for Blobmoji.
//!
//! The `Blobmoji` marker is used as the phantom type parameter in
//! `GlyphName<Blobmoji>`, ensuring type safety at compile time.

use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Blobmoji in `GlyphName<Blobmoji>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Blobmoji>` is distinct from
/// `GlyphName<OtherFamily>` at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Blobmoji {}

impl FontFamilyMarker for Blobmoji {}

impl sealed::Sealed for Blobmoji {}

/// Convenience type alias for Blobmoji glyph names.
pub type BlobmojiName = GlyphName<Blobmoji>;
