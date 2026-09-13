//! `FontFamilyConfig` for the Doto variable font family.

use const_format::concatcp;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Build-time configuration for the Doto font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct DotoConfig;

impl FontFamilyConfig for DotoConfig {
    const FONT_FAMILY_NAME: &'static str = "doto";

    const FAMILY_DISPLAY_NAME: &'static str = "Doto";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", DotoConfig::FONT_FAMILY_NAME);

    const CODEPOINT_RANGES: &[CodePointRange] = ASCII_PRINTABLE_RANGE;
}
