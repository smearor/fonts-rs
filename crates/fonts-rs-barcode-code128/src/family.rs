//! Runtime font family API for Code 128.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::Code128;
use crate::definition::Code128Definition;
use crate::definition::Code128Name;

fonts_rs_generator::impl_font_family!(
    Code128,
    Code128Definition,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = Code128Name,
    css_font_family = "LibreBarcode128",
    capabilities = [icon_theme, pango, html],
);
