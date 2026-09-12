//! EAN13 font family marker type and `FontDefinition` implementation.
//!
//! This module is shared between `build.rs` (via `#[path]`) and `src/lib.rs`
//! to avoid code duplication.

use const_format::concatcp;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_model::FontFamily;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// Marker type identifying Libre Barcode EAN13 in `GlyphName<Ean13>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Ean13>` is distinct from `GlyphName<Code39>`
/// at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Ean13 {}

impl FontFamily for Ean13 {}

impl sealed::Sealed for Ean13 {}

/// Convenience type alias for EAN13 glyph names.
pub type Ean13Name = GlyphName<Ean13>;

/// Build-time configuration for the Libre Barcode EAN13 font family.
///
/// Implements [`FontDefinition`] to plug into the generic
/// [`FontDefinition::export_glyphs`] pipeline.
pub struct Ean13Definition;

impl FontDefinition for Ean13Definition {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/barcode_ean13");

    const ICONS_CONTEXT: &'static str = "glyphs";

    /// ASCII printable range (U+0020–U+007E) covers all EAN13 glyphs.
    const CODEPOINT_RANGES: &[(u32, u32)] = &[(0x20, 0x7E)];

    type Name = Ean13Name;

    type Family = Ean13;

    /// EAN13 glyph names use dot-separated semantic names:
    /// `zero.compatibility` → `zero-compatibility`,
    /// `guard.normal` → `guard-normal`,
    /// `addOn.guard.twoDigit` → `addon-guard-twodigit`.
    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        let kebab = normalize_to_kebab(raw_glyph_name)?;
        Some(GlyphName::new(kebab))
    }

    /// Only skip `NULL` (the .null glyph). EAN13 uses semantic names
    /// which must NOT be skipped.
    fn should_skip(raw_glyph_name: &str) -> bool {
        raw_glyph_name == "NULL"
    }
}
