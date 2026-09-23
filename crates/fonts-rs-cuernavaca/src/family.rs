//! Runtime font family API for Cuernavaca.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::CuernavacaConfig;
use crate::naming::Cuernavaca;
use crate::naming::CuernavacaName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Cuernavaca,
    CuernavacaConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = CuernavacaName,
    css_font_family = "ChessCuernavaca",
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
