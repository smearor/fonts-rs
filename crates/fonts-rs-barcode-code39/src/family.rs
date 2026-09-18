//! Runtime font family API for Code 39.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::Code39;
use crate::Code39Definition;
use crate::Code39Name;
use crate::codepoint_map;

fonts_rs_generator::impl_font_family!(
    Code39,
    Code39Definition,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = Code39Name,
    css_font_family = "LibreBarcode39",
    capabilities = [icon_theme, pango, html],
);
