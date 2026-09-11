//! Reverse codepoint map construction from font glyph tables.

use std::collections::HashMap;

use read_fonts::ReadError;
use skrifa::FontRef;
use skrifa::GlyphId;
use skrifa::MetadataProvider;

use fonts_rs_model::CodePoint;

use crate::font_definition::FontDefinition;

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

    /// Returns a reference to the underlying `FontRef`.
    pub fn as_ref(&self) -> &FontRef<'a> {
        &self.inner
    }

    /// Build a reverse cmap (GlyphId -> CodePoint) by probing Unicode codepoints.
    ///
    /// Probes the codepoint ranges defined by [`FontDefinition::CODEPOINT_RANGES`]
    /// to map each glyph in the font to its Unicode codepoint.
    pub fn build_reverse_cmap<F: FontDefinition>(&self) -> HashMap<GlyphId, CodePoint> {
        let mut map = HashMap::new();
        for &(start, end) in F::CODEPOINT_RANGES {
            map.extend(self.probe_range(start, end));
        }
        map
    }

    /// Probe a contiguous range of Unicode codepoints and return matching glyphs.
    fn probe_range(&self, start: u32, end: u32) -> HashMap<GlyphId, CodePoint> {
        let charmap = self.inner.charmap();
        let mut map = HashMap::new();
        for codepoint in start..=end {
            if let Some(ch) = char::from_u32(codepoint)
                && let Some(glyph_id) = charmap.map(ch)
            {
                map.insert(glyph_id, CodePoint::from(ch));
            }
        }
        map
    }
}

impl<'a> std::ops::Deref for Font<'a> {
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
