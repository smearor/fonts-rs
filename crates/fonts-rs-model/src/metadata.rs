//! Metadata newtypes and registry trait for font glyph search.
//!
//! Provides [`GlyphKeywordMap`] and [`GlyphCategoryMap`] — newtype wrappers
//! around `phf::Map` that enable idiomatic access (Deref, iteration, len, get).
//!
//! The [`GlyphMetadata`] trait unifies per-font-family metadata access:
//! each font crate implements it with its build-time generated static maps.

use std::ops::Deref;
use phf::Map;
use crate::GlyphCategory;
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

static EMPTY_ALIASES: GlyphAliasMap = GlyphAliasMap(phf::phf_map! {});

/// Metadata registry for a font family.
///
/// Each font family crate implements this trait with its build-time
/// generated static maps, providing unified keyword, category, and
/// search access.
///
/// # Examples
///
/// ```ignore
/// pub struct NotoEmojiMetadata;
///
/// impl GlyphMetadata for NotoEmojiMetadata {
///     fn keyword_map(&self) -> &GlyphKeywordMap { &KEYWORDS }
///     fn category_map(&self) -> &GlyphCategoryMap { &CATEGORIES }
/// }
/// ```
pub trait GlyphMetadata {
    /// Returns the static keyword map.
    fn keyword_map(&self) -> &GlyphKeywordMap;

    /// Returns the static category map.
    fn category_map(&self) -> &GlyphCategoryMap;

    /// Returns the static alias map.
    ///
    /// Defaults to an empty map for font families without aliases.
    fn alias_map(&self) -> &GlyphAliasMap {
        &EMPTY_ALIASES
    }

    /// Returns keywords for `glyph_name`, or an empty slice if none.
    fn keywords(&self, glyph_name: impl AsRef<str>) -> &'static [GlyphKeyword] {
        self.keyword_map().keywords_for(glyph_name.as_ref())
    }

    /// Returns categories for `glyph_name`, or an empty slice if none.
    fn categories(&self, glyph_name: impl AsRef<str>) -> &'static [GlyphCategory] {
        self.category_map().categories_for(glyph_name.as_ref())
    }

    /// Returns the canonical glyph name for an alias, if available.
    fn resolve_alias(&self, alias: impl AsRef<str>) -> Option<&'static str> {
        self.alias_map().resolve(alias.as_ref())
    }

    /// Search glyphs by keyword, category, alias, or name.
    ///
    /// Performs a case-insensitive substring match against glyph names,
    /// keywords, categories, and aliases. Returns matching glyph names
    /// sorted alphabetically.
    fn search(&self, query: &str) -> Vec<&'static str> {
        if query.is_empty() {
            return Vec::new();
        }

        let query = query.to_lowercase();
        let mut results = Vec::new();

        for (glyph_name, keywords) in self.keyword_map().iter() {
            let name_lower = glyph_name.to_lowercase();
            if name_lower.contains(&query) {
                results.push(glyph_name);
                continue;
            }

            if keywords.iter().any(|kw| kw.as_str().to_lowercase().contains(&query)) {
                results.push(glyph_name);
                continue;
            }

            let cats = self.category_map().categories_for(glyph_name);
            if cats.iter().any(|cat| cat.as_str().to_lowercase().contains(&query)) {
                results.push(glyph_name);
                continue;
            }
        }

        // Include glyphs that have no keywords but match by name
        for (glyph_name, _) in self.category_map().iter() {
            if !results.contains(&glyph_name) {
                let name_lower = glyph_name.to_lowercase();
                if name_lower.contains(&query) {
                    results.push(glyph_name);
                }
            }
        }

        // Search aliases: if the query matches an alias name,
        // include the canonical glyph name in the results.
        for (alias_name, canonical_name) in self.alias_map().iter() {
            if alias_name.to_lowercase().contains(&query) {
                results.push(canonical_name);
            }
        }

        results.sort_unstable();
        results.dedup();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_KEYWORDS: &[GlyphKeyword] = &[GlyphKeyword::new("gamepad"), GlyphKeyword::new("controller")];
    static TEST_CATEGORIES: &[GlyphCategory] = &[GlyphCategory::new("Childhood")];

    static TEST_KW_MAP: GlyphKeywordMap = GlyphKeywordMap(phf::phf_map! {
        "nf-fa-gamepad-symbolic" => TEST_KEYWORDS,
        "nf-fa-star-symbolic" => &[GlyphKeyword::new("star"), GlyphKeyword::new("favorite")],
    });

    static TEST_CAT_MAP: GlyphCategoryMap = GlyphCategoryMap(phf::phf_map! {
        "nf-fa-gamepad-symbolic" => TEST_CATEGORIES,
        "nf-fa-star-symbolic" => &[GlyphCategory::new("Childhood"), GlyphCategory::new("Shapes")],
    });

    static TEST_ALIAS_MAP: GlyphAliasMap = GlyphAliasMap(phf::phf_map! {
        "nf-dev-arm64-symbolic" => "nf-dev-aarch64-symbolic",
    });

    struct TestMetadata;
    struct TestMetadataWithAliases;

    impl GlyphMetadata for TestMetadata {
        fn keyword_map(&self) -> &GlyphKeywordMap {
            &TEST_KW_MAP
        }
        fn category_map(&self) -> &GlyphCategoryMap {
            &TEST_CAT_MAP
        }
    }

    impl GlyphMetadata for TestMetadataWithAliases {
        fn keyword_map(&self) -> &GlyphKeywordMap {
            &TEST_KW_MAP
        }
        fn category_map(&self) -> &GlyphCategoryMap {
            &TEST_CAT_MAP
        }
        fn alias_map(&self) -> &GlyphAliasMap {
            &TEST_ALIAS_MAP
        }
    }

    // --- GlyphKeywordMap tests ---

    #[test]
    fn keyword_map_keywords_for_known() {
        let kws = TEST_KW_MAP.keywords_for("nf-fa-gamepad-symbolic");
        assert_eq!(kws.len(), 2);
        assert!(kws.iter().any(|kw| kw == "gamepad"));
        assert!(kws.iter().any(|kw| kw == "controller"));
    }

    #[test]
    fn keyword_map_keywords_for_unknown() {
        let kws = TEST_KW_MAP.keywords_for("unknown-glyph");
        assert!(kws.is_empty());
    }

    #[test]
    fn keyword_map_len() {
        assert_eq!(TEST_KW_MAP.len(), 2);
    }

    #[test]
    fn keyword_map_is_not_empty() {
        assert!(!TEST_KW_MAP.is_empty());
    }

    #[test]
    fn keyword_map_iter() {
        let entries: Vec<_> = TEST_KW_MAP.iter().collect();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn keyword_map_deref() {
        assert!(TEST_KW_MAP.contains_key("nf-fa-gamepad-symbolic"));
    }

    // --- GlyphCategoryMap tests ---

    #[test]
    fn category_map_categories_for_known() {
        let cats = TEST_CAT_MAP.categories_for("nf-fa-gamepad-symbolic");
        assert_eq!(cats.len(), 1);
        assert!(cats.iter().any(|cat| cat == "Childhood"));
    }

    #[test]
    fn category_map_categories_for_unknown() {
        let cats = TEST_CAT_MAP.categories_for("unknown-glyph");
        assert!(cats.is_empty());
    }

    #[test]
    fn category_map_len() {
        assert_eq!(TEST_CAT_MAP.len(), 2);
    }

    #[test]
    fn category_map_iter() {
        let entries: Vec<_> = TEST_CAT_MAP.iter().collect();
        assert_eq!(entries.len(), 2);
    }

    // --- GlyphAliasMap tests ---

    #[test]
    fn alias_map_resolve_known() {
        assert_eq!(TEST_ALIAS_MAP.resolve("nf-dev-arm64-symbolic"), Some("nf-dev-aarch64-symbolic"));
    }

    #[test]
    fn alias_map_resolve_unknown() {
        assert_eq!(TEST_ALIAS_MAP.resolve("nonexistent"), None);
    }

    #[test]
    fn alias_map_len() {
        assert_eq!(TEST_ALIAS_MAP.len(), 1);
    }

    #[test]
    fn alias_map_is_not_empty() {
        assert!(!TEST_ALIAS_MAP.is_empty());
    }

    #[test]
    fn alias_map_iter() {
        let entries: Vec<_> = TEST_ALIAS_MAP.iter().collect();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ("nf-dev-arm64-symbolic", "nf-dev-aarch64-symbolic"));
    }

    // --- GlyphMetadata trait: keywords / categories ---

    #[test]
    fn trait_keywords_known() {
        let md = TestMetadata;
        let kws = md.keywords("nf-fa-gamepad-symbolic");
        assert!(kws.iter().any(|kw| kw == "gamepad"));
    }

    #[test]
    fn trait_keywords_unknown() {
        let md = TestMetadata;
        assert!(md.keywords("unknown").is_empty());
    }

    #[test]
    fn trait_categories_known() {
        let md = TestMetadata;
        let cats = md.categories("nf-fa-gamepad-symbolic");
        assert!(cats.iter().any(|cat| cat == "Childhood"));
    }

    #[test]
    fn trait_categories_unknown() {
        let md = TestMetadata;
        assert!(md.categories("unknown").is_empty());
    }

    #[test]
    fn trait_keywords_accepts_string() {
        let md = TestMetadata;
        let name = String::from("nf-fa-gamepad-symbolic");
        let kws = md.keywords(&name);
        assert!(!kws.is_empty());
    }

    // --- GlyphMetadata trait: alias_map / resolve_alias ---

    #[test]
    fn trait_default_alias_map_is_empty() {
        let md = TestMetadata;
        assert!(md.alias_map().is_empty());
    }

    #[test]
    fn trait_default_resolve_alias_returns_none() {
        let md = TestMetadata;
        assert_eq!(md.resolve_alias("nf-dev-arm64-symbolic"), None);
    }

    #[test]
    fn trait_resolve_alias_known() {
        let md = TestMetadataWithAliases;
        assert_eq!(md.resolve_alias("nf-dev-arm64-symbolic"), Some("nf-dev-aarch64-symbolic"));
    }

    #[test]
    fn trait_resolve_alias_unknown() {
        let md = TestMetadataWithAliases;
        assert_eq!(md.resolve_alias("nonexistent"), None);
    }

    // --- GlyphMetadata trait: search ---

    #[test]
    fn search_empty_query_returns_empty() {
        let md = TestMetadata;
        assert!(md.search("").is_empty());
    }

    #[test]
    fn search_by_name() {
        let md = TestMetadata;
        let results = md.search("gamepad");
        assert!(results.contains(&"nf-fa-gamepad-symbolic"));
    }

    #[test]
    fn search_by_name_case_insensitive() {
        let md = TestMetadata;
        let lower = md.search("gamepad");
        let upper = md.search("GAMEPAD");
        assert_eq!(lower, upper);
    }

    #[test]
    fn search_by_keyword() {
        let md = TestMetadata;
        let results = md.search("controller");
        assert!(results.contains(&"nf-fa-gamepad-symbolic"));
    }

    #[test]
    fn search_by_category() {
        let md = TestMetadata;
        let results = md.search("Childhood");
        assert!(results.contains(&"nf-fa-gamepad-symbolic"));
        assert!(results.contains(&"nf-fa-star-symbolic"));
    }

    #[test]
    fn search_by_category_case_insensitive() {
        let md = TestMetadata;
        let results = md.search("childhood");
        assert!(results.contains(&"nf-fa-gamepad-symbolic"));
    }

    #[test]
    fn search_results_are_sorted() {
        let md = TestMetadata;
        let results = md.search("symbolic");
        let mut sorted = results.clone();
        sorted.sort_unstable();
        assert_eq!(results, sorted);
    }

    #[test]
    fn search_results_have_no_duplicates() {
        let md = TestMetadata;
        let results = md.search("fa");
        let mut deduped = results.clone();
        deduped.sort_unstable();
        deduped.dedup();
        assert_eq!(results.len(), deduped.len());
    }

    #[test]
    fn search_by_alias() {
        let md = TestMetadataWithAliases;
        let results = md.search("arm64");
        assert!(results.contains(&"nf-dev-aarch64-symbolic"));
    }

    #[test]
    fn search_by_alias_case_insensitive() {
        let md = TestMetadataWithAliases;
        let results = md.search("ARM64");
        assert!(results.contains(&"nf-dev-aarch64-symbolic"));
    }

    #[test]
    fn search_no_match_returns_empty() {
        let md = TestMetadata;
        assert!(md.search("nonexistent-term-xyz").is_empty());
    }

    #[test]
    fn search_matches_partial_name() {
        let md = TestMetadata;
        let results = md.search("star");
        assert!(results.contains(&"nf-fa-star-symbolic"));
    }
}
