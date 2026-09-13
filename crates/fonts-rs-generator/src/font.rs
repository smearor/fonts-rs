//! Font parsing utilities for glyph export.

use std::collections::HashMap;
use std::ops::Deref;

use read_fonts::ReadError;
use skrifa::FontRef;
use skrifa::GlyphId;
use skrifa::MetadataProvider;
use skrifa::instance::Location;

use fonts_rs_model::AxisValues;
use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointRange;

use crate::font_family_config::FontFamilyConfig;

/// A newtype wrapper around `skrifa::FontRef` providing font utility methods.
///
/// Implements `Deref` to `FontRef` so all underlying skrifa methods
/// (e.g. `glyph_names()`, `outline_glyphs()`) remain accessible.
pub struct Font<'a> {
    inner: FontRef<'a>,
}

impl<'a> Font<'a> {
    /// Create a `Font` from parsed font data.
    ///
    /// # Errors
    ///
    /// Returns an error if the font data cannot be parsed.
    pub fn from_data(data: &'a [u8]) -> Result<Self, ReadError> {
        Ok(Self {
            inner: FontRef::from_index(data, 0)?,
        })
    }

    /// Create a `Location` from user-space variation axis settings.
    ///
    /// Converts user coordinates (e.g. `wght=700.0, ROND=50.0`) to normalized
    /// coordinates for use with `glyph_to_svg_at` / `glyph_to_svg_full_height_at`.
    ///
    /// For non-variable fonts, returns the default location.
    pub fn location(&self, settings: &AxisValues) -> Location {
        self.inner.axes().location(settings.iter().map(|av| (av.axis.as_str(), av.value)))
    }

    /// Build a reverse cmap (GlyphId -> CodePoint) by probing Unicode codepoints.
    ///
    /// Probes the codepoint ranges defined by [`FontFamilyConfig::CODEPOINT_RANGES`]
    /// to map each glyph in the font to its Unicode codepoint.
    pub fn build_reverse_cmap<F: FontFamilyConfig>(&self) -> HashMap<GlyphId, CodePoint> {
        self.build_reverse_cmap_with_ranges(F::CODEPOINT_RANGES)
    }

    /// Build a reverse cmap (GlyphId -> CodePoint) using explicit codepoint ranges.
    ///
    /// Like [`build_reverse_cmap`](Self::build_reverse_cmap) but takes ranges
    /// directly instead of requiring a `FontFamilyConfig` impl.
    pub fn build_reverse_cmap_with_ranges(&self, ranges: &[CodePointRange]) -> HashMap<GlyphId, CodePoint> {
        let mut map = HashMap::new();
        for range in ranges {
            map.extend(self.probe_range(*range));
        }
        map
    }

    /// Probe a contiguous range of Unicode codepoints and return matching glyphs.
    fn probe_range(&self, range: CodePointRange) -> HashMap<GlyphId, CodePoint> {
        let charmap = self.inner.charmap();
        let mut map = HashMap::new();
        for codepoint in range.start().as_char() as u32..=range.end().as_char() as u32 {
            if let Some(ch) = char::from_u32(codepoint)
                && let Some(glyph_id) = charmap.map(ch)
            {
                map.insert(glyph_id, CodePoint::from(ch));
            }
        }
        map
    }
}

impl<'a> Deref for Font<'a> {
    type Target = FontRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> From<FontRef<'a>> for Font<'a> {
    fn from(inner: FontRef<'a>) -> Self {
        Self { inner }
    }
}
