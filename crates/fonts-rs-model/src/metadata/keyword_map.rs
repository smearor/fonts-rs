//! Static map from glyph names to keyword slices.

use std::ops::Deref;

use phf::Map;

use crate::GlyphKeyword;

/// A static map from glyph names to keyword slices.
///
/// Wraps `phf::Map<&'static str, &'static [GlyphKeyword]>` with idiomatic
/// access methods. Constructed at build time via `phf::phf_map!`.
///
/// # Examples
///
/// ```ignore
/// pub static KEYWORDS: GlyphKeywordMap = GlyphKeywordMap(phf::phf_map! {
///     "nf-fa-gamepad-symbolic" => &[GlyphKeyword::new("gamepad")],
/// });
/// ```
pub struct GlyphKeywordMap(pub Map<&'static str, &'static [GlyphKeyword]>);

impl GlyphKeywordMap {
    /// Returns keywords for `glyph_name`, or an empty slice if none.
    pub fn keywords_for(&self, glyph_name: impl AsRef<str>) -> &'static [GlyphKeyword] {
        self.0.get(glyph_name.as_ref()).copied().unwrap_or(&[])
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over `(glyph_name, keywords)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static [GlyphKeyword])> {
        self.0.entries().map(|(name, kws)| (*name, *kws))
    }
}

impl Deref for GlyphKeywordMap {
    type Target = Map<&'static str, &'static [GlyphKeyword]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_KEYWORDS: &[GlyphKeyword] = &[GlyphKeyword::new("gamepad"), GlyphKeyword::new("controller")];

    static TEST_KW_MAP: GlyphKeywordMap = GlyphKeywordMap(phf::phf_map! {
        "nf-fa-gamepad-symbolic" => TEST_KEYWORDS,
        "nf-fa-star-symbolic" => &[GlyphKeyword::new("star"), GlyphKeyword::new("favorite")],
    });

    #[test]
    fn keywords_for_known() {
        let kws = TEST_KW_MAP.keywords_for("nf-fa-gamepad-symbolic");
        assert_eq!(kws.len(), 2);
        assert!(kws.iter().any(|kw| kw == "gamepad"));
        assert!(kws.iter().any(|kw| kw == "controller"));
    }

    #[test]
    fn keywords_for_unknown() {
        let kws = TEST_KW_MAP.keywords_for("unknown-glyph");
        assert!(kws.is_empty());
    }

    #[test]
    fn len() {
        assert_eq!(TEST_KW_MAP.len(), 2);
    }

    #[test]
    fn is_not_empty() {
        assert!(!TEST_KW_MAP.is_empty());
    }

    #[test]
    fn iter() {
        let entries: Vec<_> = TEST_KW_MAP.iter().collect();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn deref() {
        assert!(TEST_KW_MAP.contains_key("nf-fa-gamepad-symbolic"));
    }
}
