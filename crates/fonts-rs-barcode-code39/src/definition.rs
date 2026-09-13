//! Code 39 font family marker type and `FontDefinition` implementation.
//!
//! This module is shared between `build.rs` (via `#[path]`) and `src/lib.rs`
//! to avoid code duplication.

use const_format::concatcp;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamily;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "LibreBarcode39-Regular.ttf";

/// Marker type identifying Libre Barcode Code 39 in `GlyphName<Code39>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Code39>` is distinct from `GlyphName<Code128>`
/// at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Code39 {}

impl FontFamily for Code39 {}

impl sealed::Sealed for Code39 {}

/// Convenience type alias for Code 39 glyph names.
pub type Code39Name = GlyphName<Code39>;

/// Build-time configuration for the Libre Barcode Code 39 font family.
///
/// Implements [`FontDefinition`] to plug into the generic
/// [`FontDefinition::export_glyphs`] pipeline.
pub struct Code39Definition;

impl FontDefinition for Code39Definition {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/barcode_code39");

    const ICONS_CONTEXT: &'static str = "glyphs";

    /// ASCII printable range (U+0020–U+007E) covers all Code 39 glyphs.
    const CODEPOINT_RANGES: &[CodePointRange] = ASCII_PRINTABLE_RANGE;

    type Name = Code39Name;

    type Family = Code39;

    /// Code 39 glyph names are `uniXXXX` format (e.g. `uni0030` for '0').
    /// We keep them as-is in kebab-case: `uni0030` → `uni0030`.
    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        let kebab = normalize_to_kebab(raw_glyph_name)?;
        Some(GlyphName::new(kebab))
    }

    /// Only skip `NULL` (the .null glyph). Code 39 uses `uniXXXX` names
    /// which must NOT be skipped.
    fn should_skip(raw_glyph_name: &str) -> bool {
        raw_glyph_name == "NULL"
    }
}
