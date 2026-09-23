//! Runtime font family API for EAN-13.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::Ean13;
use crate::definition::Ean13Definition;
use crate::definition::Ean13Name;

fonts_rs_generator::impl_font_family!(
    Ean13,
    Ean13Definition,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = Ean13Name,
    css_font_family = "LibreBarcodeEAN13Text",
    capabilities = [icon_theme, pango, html],
);
