//! `FontFamilyConfig` for the Tabler Icons font family.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

/// Tabler codepoint ranges: the Unicode Private Use Area plus the
/// supplementary range upstream allocates for newer icons beyond U+FFFF.
const TABLER_RANGES: &[CodePointRange] = &[
    CodePointRange::new(CodePoint::from_char('\u{E000}'), CodePoint::from_char('\u{F8FF}')),
    CodePointRange::new(CodePoint::from_char('\u{10000}'), CodePoint::from_char('\u{103FF}')),
];

/// Build-time configuration for the Tabler Icons font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline and the generic
/// [`IconName`](fonts_rs_model::IconName) normalization (`tabler-`
/// prefix, `-symbolic` suffix).
pub struct TablerConfig;

impl FontFamilyConfig for TablerConfig {
    const FONT_FAMILY_NAME: &'static str = "tabler";

    const FAMILY_DISPLAY_NAME: &'static str = "Tabler Icons";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", TablerConfig::FONT_FAMILY_NAME);

    const CODEPOINT_RANGES: &[CodePointRange] = TABLER_RANGES;

    const ICON_PREFIX: &'static str = "tabler-";
}
