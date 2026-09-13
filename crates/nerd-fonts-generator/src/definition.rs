//! `NerdFontsDefinition` — `FontDefinition` implementation for Nerd Fonts.
//!
//! This module implements the generic [`FontDefinition`] trait for the Nerd Fonts
//! font family, allowing the `FontDefinition::export_glyphs` pipeline to be used.

use fonts_rs_generator::FontDefinition;
use fonts_rs_model::BMP_RANGE;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontFamily;
use fonts_rs_model::SUPPLEMENTARY_PUA_RANGE;
use fonts_rs_model::sealed;
use nerd_fonts_model::IconName;
use nerd_fonts_model::paths::GRESOURCE_PREFIX;

/// Marker enum for the Nerd Fonts font family.
///
/// Used as the `FontFamily` phantom type parameter. Zero-sized, exists
/// only at the type level for compile-time type safety.
pub enum NerdFonts {}

impl sealed::Sealed for NerdFonts {}

impl FontFamily for NerdFonts {}

/// Build-time configuration for the Nerd Fonts font family.
///
/// Implements [`FontDefinition`] to plug into the generic
/// [`FontDefinition::export_glyphs`] pipeline.
pub struct NerdFontsDefinition;

impl FontDefinition for NerdFontsDefinition {
    const GRESOURCE_PREFIX: &'static str = GRESOURCE_PREFIX;

    const ICONS_CONTEXT: &'static str = "glyphs";

    /// Unicode codepoint ranges to probe for reverse cmap construction.
    ///
    /// Nerd Fonts uses two disjoint ranges:
    ///
    /// - **BMP (`U+0000`–`U+FFFF`)**: Covers legacy icon codepoints in the
    ///   Basic Multilingual Plane, including the PUA range `U+E000`–`U+F8FF`
    ///   where most Font Awesome and Material Design icons are mapped.
    /// - **Supplementary PUA (`U+F0001`–`U+10FFFF`)**: Covers the
    ///   Supplementary Private Use Area, where Nerd Fonts 3+ places
    ///   additional icon codepoints that don't fit in the BMP PUA.
    ///
    /// At ~16ns per `glyph_index` lookup, probing both ranges takes ~17ms.
    const CODEPOINT_RANGES: &[CodePointRange] = &[BMP_RANGE, SUPPLEMENTARY_PUA_RANGE];

    type Name = IconName;

    type Family = NerdFonts;

    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        IconName::from_glyph_name(raw_glyph_name)
    }
}
