//! `FontFamilyConfig` for the Nerd Fonts font family.
//!
//! [`NerdFontsConfig`] lives in `nerd-fonts-model` (not `nerd-fonts-rs`)
//! so that every crate in the Nerd Fonts stack - model, generator,
//! runtime, cheat sheet - can name [`IconName`](crate::IconName), which
//! is phantom-typed over this config.

use crate::paths::GRESOURCE_PREFIX;
use crate::set::IconSet;
use fonts_rs_model::BMP_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::IconSetDetect;
use fonts_rs_model::SUPPLEMENTARY_PUA_RANGE;

/// Build-time configuration for the Nerd Fonts font family.
///
/// Supplies the constants for the generic [`IconName`](crate::IconName)
/// pipeline: the `nf-` icon prefix and `IconSet`-based validation.
pub struct NerdFontsConfig;

impl FontFamilyConfig for NerdFontsConfig {
    const FONT_FAMILY_NAME: &'static str = "nerd_fonts";

    const FAMILY_DISPLAY_NAME: &'static str = "Nerd Fonts";

    const GRESOURCE_PREFIX: &'static str = GRESOURCE_PREFIX;

    /// Nerd Fonts uses two disjoint ranges:
    ///
    /// - **BMP (`U+0000`–`U+FFFF`)**: legacy icon codepoints including
    ///   the PUA range `U+E000`–`U+F8FF`.
    /// - **Supplementary PUA (`U+F0001`–`U+10FFFF`)**: additional icon
    ///   codepoints that don't fit in the BMP PUA (Nerd Fonts 3+).
    const CODEPOINT_RANGES: &[CodePointRange] = &[BMP_RANGE, SUPPLEMENTARY_PUA_RANGE];

    /// Nerd Fonts icon names carry the `nf-` prefix.
    const ICON_PREFIX: &'static str = "nf-";

    /// A Nerd Fonts icon name is valid iff it resolves to a known
    /// [`IconSet`] - the multi-set override of the default prefix check.
    fn is_valid_icon_name(name: &str) -> bool {
        IconSet::detect(name) != IconSet::Other
    }
}

impl IconSetDetect for NerdFontsConfig {
    type Set = IconSet;

    fn detect_set(icon_name: &str) -> Self::Set {
        IconSet::detect(icon_name)
    }
}
