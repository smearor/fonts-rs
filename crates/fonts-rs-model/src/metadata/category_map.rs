//! Static map from glyph names to category slices.

use std::ops::Deref;

use phf::Map;

use crate::GlyphCategory;

/// A static map from glyph names to category slices.
///
/// Wraps `phf::Map<&'static str, &'static [GlyphCategory]>` with idiomatic
/// access methods. Constructed at build time via `phf::phf_map!`.
///
/// # Examples
///
/// ```ignore
/// pub static CATEGORIES: GlyphCategoryMap = GlyphCategoryMap(phf::phf_map! {
///     "nf-fa-gamepad-symbolic" => &[GlyphCategory::new("Childhood")],
/// });
/// ```
pub struct GlyphCategoryMap(pub Map<&'static str, &'static [GlyphCategory]>);

impl GlyphCategoryMap {
    /// Returns categories for `glyph_name`, or an empty slice if none.
    pub fn categories_for(&self, glyph_name: impl AsRef<str>) -> &'static [GlyphCategory] {
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

    /// Returns an iterator over `(glyph_name, categories)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static [GlyphCategory])> {
        self.0.entries().map(|(name, cats)| (*name, *cats))
    }
}

impl Deref for GlyphCategoryMap {
    type Target = Map<&'static str, &'static [GlyphCategory]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CATEGORIES: &[GlyphCategory] = &[GlyphCategory::new("Childhood")];

    static TEST_CAT_MAP: GlyphCategoryMap = GlyphCategoryMap(phf::phf_map! {
        "nf-fa-gamepad-symbolic" => TEST_CATEGORIES,
        "nf-fa-star-symbolic" => &[GlyphCategory::new("Childhood"), GlyphCategory::new("Shapes")],
    });

    #[test]
    fn categories_for_known() {
        let cats = TEST_CAT_MAP.categories_for("nf-fa-gamepad-symbolic");
        assert_eq!(cats.len(), 1);
        assert!(cats.iter().any(|cat| cat == "Childhood"));
    }

    #[test]
    fn categories_for_unknown() {
        let cats = TEST_CAT_MAP.categories_for("unknown-glyph");
        assert!(cats.is_empty());
    }

    #[test]
    fn len() {
        assert_eq!(TEST_CAT_MAP.len(), 2);
    }

    #[test]
    fn iter() {
        let entries: Vec<_> = TEST_CAT_MAP.iter().collect();
        assert_eq!(entries.len(), 2);
    }
}
