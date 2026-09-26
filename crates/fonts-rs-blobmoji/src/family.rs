//! Runtime font family API for Blobmoji.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::BlobmojiConfig;
use crate::naming::Blobmoji;
use crate::naming::BlobmojiName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Blobmoji,
    BlobmojiConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = BlobmojiName,
    glyph_prefix = variant::GLYPH_PREFIX,
    gresource_prefix = variant::GRESOURCE_PREFIX,
    capabilities = [icon_theme, pango, html],
);
