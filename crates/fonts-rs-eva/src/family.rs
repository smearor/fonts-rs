//! Runtime font family API for Eva Icons.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::EvaConfig;
use crate::naming::Eva;
use crate::naming::EvaIconName;

fonts_rs_generator::impl_font_family!(
    Eva,
    EvaConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = EvaIconName,
    css_font_family = "Eva-Icons",
    glyph_prefix = "eva",
    capabilities = [icon_theme, pango, html],
);
