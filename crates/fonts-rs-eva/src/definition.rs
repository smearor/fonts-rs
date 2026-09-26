//! `FontFamilyConfig` for the Eva Icons font family.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Eva codepoint range: the Private Use Area block used by the upstream
/// webfont (`\ea01`–`\ebea` in `eva-icons.css`).
const EVA_RANGES: &[CodePointRange] = &[CodePointRange::new(CodePoint::from_char('\u{EA00}'), CodePoint::from_char('\u{EBFF}'))];

/// Build-time configuration for the Eva Icons font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline and the generic
/// [`IconName`](fonts_rs_model::IconName) normalization (`eva-`
/// prefix, `-symbolic` suffix).
pub struct EvaConfig;

impl FontFamilyConfig for EvaConfig {
    const FONT_FAMILY_NAME: &'static str = "eva";

    const FAMILY_DISPLAY_NAME: &'static str = "Eva Icons";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", EvaConfig::FONT_FAMILY_NAME);

    const CODEPOINT_RANGES: &[CodePointRange] = EVA_RANGES;

    const ICON_PREFIX: &'static str = "eva-";
}
