//! `FontFamilyConfig` for the Noto Emoji font family.

use const_format::concatcp;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::ICONS_CONTEXT_EMOJI;

/// Emoji codepoint ranges (supplementary planes + selected BMP ranges).
const EMOJI_RANGES: &[CodePointRange] = &[
    CodePointRange::new(fonts_rs_model::CodePoint::from_char('\u{1F000}'), fonts_rs_model::CodePoint::from_char('\u{1FAFF}')),
    CodePointRange::new(fonts_rs_model::CodePoint::from_char('\u{2600}'), fonts_rs_model::CodePoint::from_char('\u{27BF}')),
];

/// Build-time configuration for the Noto Emoji font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct NotoEmojiConfig;

impl FontFamilyConfig for NotoEmojiConfig {
    const FONT_FAMILY_NAME: &'static str = "noto-emoji";

    const FAMILY_DISPLAY_NAME: &'static str = "Noto Emoji";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", NotoEmojiConfig::FONT_FAMILY_NAME);

    const ICONS_CONTEXT: &'static str = ICONS_CONTEXT_EMOJI;

    const CODEPOINT_RANGES: &[CodePointRange] = EMOJI_RANGES;
}
