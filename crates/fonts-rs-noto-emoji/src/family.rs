//! Runtime font family API for Noto Emoji.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::NotoEmojiConfig;
use crate::naming::NotoEmoji;
use crate::naming::NotoEmojiName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    NotoEmoji,
    NotoEmojiConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = NotoEmojiName,
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
