//! Static map from alias names to canonical glyph names.

use std::ops::Deref;

use phf::Map;

/// A static map from alias names to canonical glyph names.
///
/// Wraps `phf::Map<&'static str, &'static str>` with idiomatic access
/// methods. Constructed at build time via `phf::phf_map!`.
///
/// # Examples
///
/// ```ignore
/// pub static ALIASES: GlyphAliasMap = GlyphAliasMap(phf::phf_map! {
///     "nf-dev-arm64-symbolic" => "nf-dev-aarch64-symbolic",
/// });
/// ```
pub struct GlyphAliasMap(pub Map<&'static str, &'static str>);

impl GlyphAliasMap {
    /// Returns the canonical name for `alias`, or `None` if not an alias.
    pub fn resolve(&self, alias: impl AsRef<str>) -> Option<&'static str> {
        self.0.get(alias.as_ref()).copied()
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over `(alias, canonical_name)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.0.entries().map(|(alias, canonical)| (*alias, *canonical))
    }
}

impl Deref for GlyphAliasMap {
    type Target = Map<&'static str, &'static str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_ALIAS_MAP: GlyphAliasMap = GlyphAliasMap(phf::phf_map! {
        "nf-dev-arm64-symbolic" => "nf-dev-aarch64-symbolic",
    });

    #[test]
    fn resolve_known() {
        assert_eq!(TEST_ALIAS_MAP.resolve("nf-dev-arm64-symbolic"), Some("nf-dev-aarch64-symbolic"));
    }

    #[test]
    fn resolve_unknown() {
        assert_eq!(TEST_ALIAS_MAP.resolve("nonexistent"), None);
    }

    #[test]
    fn len() {
        assert_eq!(TEST_ALIAS_MAP.len(), 1);
    }

    #[test]
    fn is_not_empty() {
        assert!(!TEST_ALIAS_MAP.is_empty());
    }

    #[test]
    fn iter() {
        let entries: Vec<_> = TEST_ALIAS_MAP.iter().collect();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ("nf-dev-arm64-symbolic", "nf-dev-aarch64-symbolic"));
    }
}
