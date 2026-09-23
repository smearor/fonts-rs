//! Runtime font family API for Bravura.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::BravuraConfig;
use crate::naming::Bravura;
use crate::naming::BravuraName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Bravura,
    BravuraConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = BravuraName,
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
