//! Runtime font family API for Dicefont.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::DicefontConfig;
use crate::naming::Dicefont;
use crate::naming::DicefontName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    Dicefont,
    DicefontConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = DicefontName,
    css_font_family = "dicefont",
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
