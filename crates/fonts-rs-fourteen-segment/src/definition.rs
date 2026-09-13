//! `FontFamilyConfig` for the DSEG14 fourteen-segment font family.

use const_format::concatcp;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Build-time configuration for the DSEG14 font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct FourteenSegmentConfig;

impl FontFamilyConfig for FourteenSegmentConfig {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/fourteen_segment");

    const CODEPOINT_RANGES: &[CodePointRange] = ASCII_PRINTABLE_RANGE;
}
