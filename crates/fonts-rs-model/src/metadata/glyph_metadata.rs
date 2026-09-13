//! Metadata registry trait for font glyph search.

use crate::GlyphAliasMap;
use crate::GlyphCategory;
use crate::GlyphCategoryMap;
use crate::GlyphKeyword;
use crate::GlyphKeywordMap;

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
