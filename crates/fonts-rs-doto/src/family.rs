//! Runtime font family API for Doto.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::DotoConfig;
use crate::naming::Doto;
use crate::naming::DotoName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Doto,
    DotoConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = DotoName,
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
