//! Icon metadata: keywords and categories for search functionality.
//!
//! Provides keyword and category lookup for Nerd Font icons, enabling
//! search and filtering beyond simple name matching.
//!
//! Keywords and categories are sourced from upstream icon set metadata
//! (Font Awesome, Material Design, Devicon, Octicons) when available,
//! and from icon name segments for other sets.
//!
//! This module is only available when the `metadata` feature is enabled.

use fonts_rs_model::GlyphAliasMap;
use fonts_rs_model::GlyphCategory;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;

// Include the build-time generated phf::Map constants.
// The generated code references GlyphKeyword, GlyphCategory, and the Map newtypes.
include!(concat!(env!("OUT_DIR"), "/metadata.rs"));

/// Metadata registry for the Nerd Fonts icon collection.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from upstream icon set metadata (Font Awesome, Material Design,
/// Devicon, Octicons).
pub struct NerdFontsMetadata;

impl GlyphMetadata for NerdFontsMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }

    fn alias_map(&self) -> &GlyphAliasMap {
        &ALIASES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nerd_fonts_model::IconName;

    fn parse(name: &str) -> IconName {
        IconName::parse(name).unwrap_or_else(|| panic!("failed to parse icon name: {name}"))
    }

    #[test]
    fn icon_keywords_known_icon() {
        let name = parse("nf-fa-gamepad-symbolic");
        let kws = NerdFontsMetadata.keywords(&name);
        assert!(kws.iter().any(|kw| kw == "gamepad"), "keywords should contain 'gamepad'");
    }

    #[test]
    fn icon_keywords_fa_search_terms() {
        // Font Awesome provides search.terms for gamepad: "controller", "video game"
        let name = parse("nf-fa-gamepad-symbolic");
        let kws = NerdFontsMetadata.keywords(&name);
        assert!(
            kws.iter().any(|kw| kw == "controller") || kws.iter().any(|kw| kw == "video game"),
            "FA search terms should be present for gamepad"
        );
    }

    #[test]
    fn icon_keywords_material_design_tags() {
        // Material Design provides tags for home: "house", "building"
        let name = parse("nf-md-home-symbolic");
        let kws = NerdFontsMetadata.keywords(&name);
        assert!(
            kws.iter().any(|kw| kw == "house") || kws.iter().any(|kw| kw == "building"),
            "MD tags should be present for home, got: {:?}",
            kws
        );
    }

    #[test]
    fn icon_keywords_devicon_tags() {
        // Devicon provides tags for icons, e.g. programming language tags
        let name = parse("nf-dev-python-symbolic");
        let kws = NerdFontsMetadata.keywords(&name);
        let has_tags = kws.iter().any(|kw| kw.as_str().contains("programming") || kw.as_str().contains("language"));
        assert!(has_tags, "Devicon tags should be present for python, got: {:?}", kws);
    }

    #[test]
    fn icon_keywords_octicons_keywords() {
        // Octicons provides keywords for alert: "warning", "triangle"
        let name = parse("nf-oct-alert-symbolic");
        let kws = NerdFontsMetadata.keywords(&name);
        assert!(
            kws.iter().any(|kw| kw == "warning") || kws.iter().any(|kw| kw == "triangle"),
            "Octicons keywords should be present for alert, got: {:?}",
            kws
        );
    }

    #[test]
    fn icon_keywords_short_name() {
        let name = parse("fa-gamepad");
        let kws = NerdFontsMetadata.keywords(&name);
        assert!(kws.iter().any(|kw| kw == "gamepad"), "keywords should contain 'gamepad' for short name");
    }

    #[test]
    fn icon_keywords_multi_segment() {
        let name = parse("nf-md-alert-circle-outline-symbolic");
        let kws = NerdFontsMetadata.keywords(&name);
        assert!(kws.iter().any(|kw| kw == "alert"), "keywords should contain 'alert'");
        assert!(kws.iter().any(|kw| kw == "circle"), "keywords should contain 'circle'");
    }

    #[test]
    fn icon_categories_fa_gamepad() {
        // Font Awesome categories.yml maps gamepad to "Childhood"
        let name = parse("nf-fa-gamepad-symbolic");
        let cats = NerdFontsMetadata.categories(&name);
        assert!(
            cats.iter().any(|cat| cat == "Childhood"),
            "categories should contain 'Childhood' for gamepad, got: {:?}",
            cats
        );
    }

    #[test]
    fn icon_categories_material_design() {
        // Material Design metadata from Google maps "home" to category "action"
        let name = parse("nf-md-home-symbolic");
        let cats = NerdFontsMetadata.categories(&name);
        assert!(cats.iter().any(|cat| cat == "action"), "categories should contain 'action' for home, got: {:?}", cats);
    }

    #[test]
    fn icon_categories_short_name() {
        let name = parse("fa-gamepad");
        let cats = NerdFontsMetadata.categories(&name);
        assert!(cats.iter().any(|cat| cat == "Childhood"), "categories should work with short name, got: {:?}", cats);
    }

    #[test]
    fn search_icons_by_keyword() {
        let results = NerdFontsMetadata.search("gamepad");
        assert!(!results.is_empty(), "search for 'gamepad' should find icons");
        assert!(results.iter().any(|name| name.contains("gamepad")), "results should contain gamepad icon");
    }

    #[test]
    fn search_icons_by_category() {
        // "Childhood" is a real FA category that includes gamepad
        let results = NerdFontsMetadata.search("Childhood");
        assert!(!results.is_empty(), "search for 'Childhood' should find icons");
        assert!(results.iter().any(|name| name.contains("gamepad")), "results should contain gamepad icon");
    }

    #[test]
    fn search_icons_case_insensitive() {
        let lower = NerdFontsMetadata.search("gamepad");
        let upper = NerdFontsMetadata.search("GAMEPAD");
        assert_eq!(lower, upper, "search should be case-insensitive");
    }

    #[test]
    fn search_icons_empty_query() {
        let results = NerdFontsMetadata.search("");
        assert!(results.is_empty(), "empty query should return no results");
    }

    #[test]
    fn search_icons_no_duplicates() {
        let results = NerdFontsMetadata.search("fa");
        let mut sorted = results.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(results.len(), sorted.len(), "search results should not contain duplicates");
    }

    #[test]
    fn search_icons_results_are_sorted() {
        let results = NerdFontsMetadata.search("icon");
        let mut sorted = results.clone();
        sorted.sort_unstable();
        assert_eq!(results, sorted, "search results should be sorted");
    }
}
