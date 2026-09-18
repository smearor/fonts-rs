//! `FontFamilyConfig` for the Nerd Fonts font family.

use fonts_rs_model::BMP_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::SUPPLEMENTARY_PUA_RANGE;
use nerd_fonts_model::paths::GRESOURCE_PREFIX;

/// Build-time configuration for the Nerd Fonts font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct NerdFontsConfig;

impl FontFamilyConfig for NerdFontsConfig {
    const FONT_FAMILY_NAME: &'static str = "nerd_fonts";

    const FAMILY_DISPLAY_NAME: &'static str = "Nerd Fonts";

    const GRESOURCE_PREFIX: &'static str = GRESOURCE_PREFIX;

    const CODEPOINT_RANGES: &[CodePointRange] = &[BMP_RANGE, SUPPLEMENTARY_PUA_RANGE];
}
