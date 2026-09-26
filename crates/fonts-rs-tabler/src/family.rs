//! Runtime font family API for Tabler Icons.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::TablerConfig;
use crate::naming::Tabler;
use crate::naming::TablerIconName;

fonts_rs_generator::impl_font_family!(
    Tabler,
    TablerConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = TablerIconName,
    css_font_family = "tabler-icons",
    glyph_prefix = "tabler",
    capabilities = [icon_theme, pango, html],
);
