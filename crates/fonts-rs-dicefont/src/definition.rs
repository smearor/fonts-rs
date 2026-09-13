//! `FontFamilyConfig` for the Dicefont font family.

use const_format::concatcp;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// PUA range (U+F000–U+FFFF) covering all dicefont glyphs.
const CODEPOINT_RANGES: &[CodePointRange] = &[CodePointRange::from_char('\u{F000}', '\u{FFFF}')];

/// Build-time configuration for the Dicefont font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct DicefontConfig;

impl FontFamilyConfig for DicefontConfig {
    const FONT_FAMILY_NAME: &'static str = "dicefont";

    const FAMILY_DISPLAY_NAME: &'static str = "Dicefont";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", DicefontConfig::FONT_FAMILY_NAME);

    const CODEPOINT_RANGES: &[CodePointRange] = CODEPOINT_RANGES;
}
