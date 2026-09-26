//! Runtime font family API for DSEG7.
//!
//! Emits the `FontFamilyConfig` delegation, the `FontFamily` core impl
//! wiring the generated phf maps, the declared capability impls, and
//! inherent forwarding methods via [`impl_font_family!`].

use crate::codepoint_map;
use crate::definition::SevenSegmentConfig;
use crate::naming::SevenSegment;
use crate::naming::SevenSegmentName;
use crate::variant;

fonts_rs_generator::impl_font_family!(
    SevenSegment,
    SevenSegmentConfig,
    glyphs = codepoint_map::GLYPHS,
    reverse = codepoint_map::REVERSE_GLYPHS,
    name = SevenSegmentName,
    css_font_family = "DSEG7 Classic",
    glyph_prefix = variant::GLYPH_PREFIX,
    gresource_prefix = variant::GRESOURCE_PREFIX,
    capabilities = [icon_theme, pango, html],
);
