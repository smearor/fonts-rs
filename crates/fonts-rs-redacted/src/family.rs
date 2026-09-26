//! Runtime font family API for Redacted.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::RedactedConfig;
use crate::naming::Redacted;
use crate::naming::RedactedName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Redacted,
    RedactedConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = RedactedName,
    glyph_prefix = variant::GLYPH_PREFIX,
    gresource_prefix = variant::GRESOURCE_PREFIX,
    capabilities = [icon_theme, pango, html],
);
