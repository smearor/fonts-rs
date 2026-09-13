//! Generator for phf::Map codepoint lookup tables.

use fonts_rs_generator::CodemapGenerator;
use fonts_rs_generator::CodemapNaming;

/// Nerd Fonts naming: `ICONS` / `REVERSE_ICONS`.
pub struct IconsNaming;

impl CodemapNaming for IconsNaming {
    const FORWARD: &'static str = "ICONS";
    const REVERSE: &'static str = "REVERSE_ICONS";
}

/// Generates phf::Map constants for both codepoint-to-name and name-to-codepoint lookups.
pub type IconsCodemapGenerator = CodemapGenerator<IconsNaming>;
