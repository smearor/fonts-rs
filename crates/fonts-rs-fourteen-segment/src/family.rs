//! Runtime font family API for DSEG14.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::FourteenSegmentConfig;
use crate::naming::FourteenSegment;
use crate::naming::FourteenSegmentName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    FourteenSegment,
    FourteenSegmentConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = FourteenSegmentName,
    css_font_family = "DSEG14 Classic",
    glyph_prefix = variant::GLYPH_PREFIX,
    capabilities = [icon_theme, pango, html],
);
