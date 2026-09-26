//! `FontFamilyConfig` for the Blobmoji font family.

use const_format::concatcp;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::ICONS_CONTEXT_EMOJI;

/// Emoji codepoint ranges — same coverage as Noto Emoji.
const EMOJI_RANGES: &[CodePointRange] = &[
    CodePointRange::new(CodePoint::from_char('\u{2600}'), CodePoint::from_char('\u{26FF}')),
    CodePointRange::new(CodePoint::from_char('\u{2700}'), CodePoint::from_char('\u{27BF}')),
    CodePointRange::new(CodePoint::from_char('\u{1F300}'), CodePoint::from_char('\u{1F5FF}')),
    CodePointRange::new(CodePoint::from_char('\u{1F600}'), CodePoint::from_char('\u{1F64F}')),
    CodePointRange::new(CodePoint::from_char('\u{1F680}'), CodePoint::from_char('\u{1F6FF}')),
    CodePointRange::new(CodePoint::from_char('\u{1F700}'), CodePoint::from_char('\u{1F77F}')),
    CodePointRange::new(CodePoint::from_char('\u{1F780}'), CodePoint::from_char('\u{1F7FF}')),
    CodePointRange::new(CodePoint::from_char('\u{1F900}'), CodePoint::from_char('\u{1F9FF}')),
    CodePointRange::new(CodePoint::from_char('\u{1FA70}'), CodePoint::from_char('\u{1FAFF}')),
];

/// Build-time configuration for the Blobmoji font family.
///
/// Implements [`FontFamilyConfig`] to provide compile-time constants
/// for the `ExportConfig`-based build pipeline.
pub struct BlobmojiConfig;

impl FontFamilyConfig for BlobmojiConfig {
    const FONT_FAMILY_NAME: &'static str = "blobmoji";

    const FAMILY_DISPLAY_NAME: &'static str = "Blobmoji";

    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/", BlobmojiConfig::FONT_FAMILY_NAME);

    const ICONS_CONTEXT: &'static str = ICONS_CONTEXT_EMOJI;

    const CODEPOINT_RANGES: &[CodePointRange] = EMOJI_RANGES;
}
