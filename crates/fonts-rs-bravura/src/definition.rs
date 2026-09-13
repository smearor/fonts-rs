//! `FontFamilyConfig` for the Bravura SMuFL music font family.

use const_format::concatcp;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::PUA_RANGE;

/// Build-time configuration for the Bravura font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct BravuraConfig;

impl FontFamilyConfig for BravuraConfig {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/bravura");

    const CODEPOINT_RANGES: &[CodePointRange] = &[PUA_RANGE];
}
