//! `FontFamilyConfig` for the Chess Cuernavaca font family.

use const_format::concatcp;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Build-time configuration for the Chess Cuernavaca font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct CuernavacaConfig;

impl FontFamilyConfig for CuernavacaConfig {
    const FONT_FAMILY_NAME: &'static str = "cuernavaca";

    const FAMILY_DISPLAY_NAME: &'static str = "Cuernavaca";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", CuernavacaConfig::FONT_FAMILY_NAME);

    const CODEPOINT_RANGES: &[CodePointRange] = ASCII_PRINTABLE_RANGE;
}
