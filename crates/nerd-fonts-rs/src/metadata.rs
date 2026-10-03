//! Icon metadata: keywords and categories for search functionality.
//!
//! Provides keyword and category lookup for Nerd Font icons, enabling
//! search and filtering beyond simple name matching.
//!
//! Keywords and categories are sourced from upstream icon set metadata
//! (Font Awesome, Material Design, Devicon, Octicons) when available,
//! and from icon name segments for other sets.
//!
//! [`GlyphOrdering`] groups glyphs by their [`IconSet`] (the cheat
//! sheet's group dropdown + sidebar section). Nerd Fonts has no variant
//! or tone structure - icon sets are disjoint namespaces.
//!
//! This module is only available when the `metadata` feature is enabled.

use std::collections::HashMap;
use std::sync::OnceLock;

use fonts_rs_model::GlyphAliasMap;
use fonts_rs_model::GlyphCategory;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphGroup;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;
use fonts_rs_model::GlyphOrdering;
use fonts_rs_model::GroupLabel;
use nerd_fonts_model::IconSet;

use crate::icons::codepoint_map::ICONS;

// Include the build-time generated phf::Map constants.
// The generated code references GlyphKeyword, GlyphCategory, and the Map newtypes.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));

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

/// Ordering tables derived once from the generated codepoint map.
///
/// Nerd Fonts has no upstream ordering JSON, so the tables are computed
/// lazily from `ICONS` (all exported names) and `IconSet::detect`
/// instead of being codegen'd.
struct OrderingTables {
    /// Group-major index: sets in alphabetical label order ("Other"
    /// last), glyphs alphabetical within each set - turns search
    /// results into a grouped picker.
    order: HashMap<&'static str, u32>,
    /// Icon-set label per glyph name.
    group_of: HashMap<&'static str, GroupLabel>,
    /// Non-empty sets, alphabetical by label ("Other" last).
    group_labels: &'static [GroupLabel],
    /// One group per set with alphabetically sorted glyph names.
    groups: &'static [GlyphGroup],
}

/// Returns the process-cached [`OrderingTables`].
fn tables() -> &'static OrderingTables {
    static TABLES: OnceLock<OrderingTables> = OnceLock::new();
    TABLES.get_or_init(build_tables)
}

/// Builds the ordering tables from the generated codepoint map.
///
/// The slices are intentionally leaked: [`GlyphOrdering`] returns
/// `&'static` references, so the tables live for the process lifetime.
fn build_tables() -> OrderingTables {
    let mut by_label: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let mut group_of: HashMap<&'static str, GroupLabel> = HashMap::new();
    for (_, name) in ICONS.entries() {
        let label = IconSet::detect(name).label();
        by_label.entry(label).or_default().push(*name);
        group_of.insert(*name, GroupLabel::new(label));
    }

    // Alphabetical by label; "Other" sinks to the end.
    let mut labels: Vec<&'static str> = by_label.keys().copied().collect();
    labels.sort_unstable();
    if let Some(position) = labels.iter().position(|label| *label == IconSet::Other.label()) {
        let other = labels.remove(position);
        labels.push(other);
    }

    let mut order = HashMap::with_capacity(group_of.len());
    let mut groups = Vec::with_capacity(labels.len());
    let mut index = 0u32;
    for &label in &labels {
        let mut glyphs = by_label.remove(label).unwrap_or_default();
        glyphs.sort_unstable();
        for &glyph in &glyphs {
            order.insert(glyph, index);
            index += 1;
        }
        groups.push(GlyphGroup::new(GroupLabel::new(label), Box::leak(glyphs.into_boxed_slice())));
    }

    OrderingTables {
        order,
        group_of,
        group_labels: Box::leak(labels.iter().map(|label| GroupLabel::new(label)).collect::<Vec<_>>().into_boxed_slice()),
        groups: Box::leak(groups.into_boxed_slice()),
    }
}

impl GlyphOrdering for NerdFontsMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Icon set of a glyph: `nf-fa-*` -> "Font Awesome".
    fn group(&self, name: &str) -> Option<GroupLabel> {
        tables().group_of.get(name).copied()
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        tables().group_labels
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        tables().groups
    }

    /// Icon sets are disjoint namespaces, not variant pairs - there is
    /// no base/variant relationship.
    fn base_of(&self, _name: &str) -> Option<&'static str> {
        None
    }

    fn collapse_variants(&self) -> bool {
        false
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

    #[test]
    fn ordering_group_font_awesome() {
        let group = NerdFontsMetadata.group("nf-fa-gamepad-symbolic");
        assert_eq!(group.map(|g| g.as_str().to_string()).as_deref(), Some("Font Awesome"));
    }

    #[test]
    fn ordering_group_labels_nonempty() {
        let labels = NerdFontsMetadata.group_labels();
        assert!(labels.len() > 10, "expected many icon-set groups, got {}", labels.len());
        assert!(labels.iter().any(|l| l.as_str() == "Font Awesome"));
    }

    #[test]
    fn ordering_groups_cover_all_glyphs() {
        let total: usize = NerdFontsMetadata.groups().iter().map(|g| g.glyphs.len()).sum();
        let all = crate::icons::codepoint_map::ICONS.entries().count();
        assert_eq!(total, all, "groups should partition every glyph");
    }

    #[test]
    fn ordering_index_matches_group_order() {
        // Every glyph in an earlier group sorts before all glyphs of a
        // later group (group-major ordering).
        let groups = NerdFontsMetadata.groups();
        let mut previous_max = None;
        for group in groups {
            let indices: Vec<u32> = group.glyphs.iter().filter_map(|name| NerdFontsMetadata.order_index(name)).collect();
            assert_eq!(indices.len(), group.glyphs.len(), "every group member needs an order_index");
            if let Some(max) = previous_max {
                let min = *indices.iter().min().unwrap_or(&0);
                assert!(min > max, "group {} overlaps the previous range", group.name.as_str());
            }
            previous_max = indices.iter().max().copied();
        }
    }

    #[test]
    fn ordering_has_no_variants() {
        assert_eq!(NerdFontsMetadata.base_of("nf-fa-gamepad-symbolic"), None);
        assert!(!NerdFontsMetadata.collapse_variants());
    }
}
