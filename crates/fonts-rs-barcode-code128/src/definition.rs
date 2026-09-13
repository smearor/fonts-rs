//! Code 128 font family marker type and `FontDefinition` implementation.
//!
//! This module is shared between `build.rs` (via `#[path]`) and `src/lib.rs`
//! to avoid code duplication.

use const_format::concatcp;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamily;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "LibreBarcode128-Regular.ttf";

/// Marker type identifying Libre Barcode Code 128 in `GlyphName<Code128>`.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `GlyphName<Code128>` is distinct from `GlyphName<Code39>`
/// at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Code128 {}

impl FontFamily for Code128 {}

impl sealed::Sealed for Code128 {}

/// Convenience type alias for Code 128 glyph names.
pub type Code128Name = GlyphName<Code128>;

/// Build-time configuration for the Libre Barcode Code 128 font family.
///
/// Implements [`FontDefinition`] to plug into the generic
/// [`FontDefinition::export_glyphs`] pipeline.
pub struct Code128Definition;

impl FontFamilyConfig for Code128Definition {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/barcode_code128");

    const ICONS_CONTEXT: &'static str = "glyphs";

    /// ASCII printable range (U+0020–U+007E) covers all Code 128 glyphs.
    const CODEPOINT_RANGES: &[CodePointRange] = ASCII_PRINTABLE_RANGE;
}

impl FontDefinition for Code128Definition {
    type Name = Code128Name;

    type Family = Code128;

    /// Code 128 glyph names are `uniXXXX.code.name` format.
    /// We extract the semantic name after `.code.`: `uni0030.code.zero` → `zero`.
    /// For names without `.code.` (e.g. `space`, `CR`), we use `normalize_to_kebab`.
    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        let name = if let Some(pos) = raw_glyph_name.find(".code.") {
            &raw_glyph_name[pos + 6..]
        } else {
            raw_glyph_name
        };
        let kebab = normalize_to_kebab(name)?;
        Some(GlyphName::new(kebab))
    }

    /// Only skip `NULL` (the .null glyph). Code 128 uses `uniXXXX.code.name`
    /// names which must NOT be skipped.
    fn should_skip(raw_glyph_name: &str) -> bool {
        raw_glyph_name == "NULL"
    }
}
