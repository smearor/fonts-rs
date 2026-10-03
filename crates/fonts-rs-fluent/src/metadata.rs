//! Icon metadata: keywords, style categories, aliases and ordering.
//!
//! Provides keyword and alias lookup for Fluent UI icons, enabling
//! search beyond simple name matching - e.g. "wand" finds
//! `fluent-wand-symbolic` directly and via the iconify alias
//! `magic-wand`.
//!
//! Keywords are filename-derived (kebab stems split into word tokens;
//! adjacent single-char tokens merge so `i-o-t` yields `iot`).
//! Aliases come from the vendored `resources/metadata/aliases.json`
//! iconify rename table and only resolve to exported glyph names.
//! Categories carry the style ("regular" / "filled"); Fluent ships
//! no categories upstream.
//!
//! [`GlyphOrdering`] groups glyphs by style and pairs each
//! `fluent-filled-{name}-symbolic` onto its regular base
//! `fluent-{name}-symbolic` - the link powers the detail sidebar's
//! Variants section and sorts each filled glyph next to its regular
//! base (styles never collapse).
//!
//! This module is only available when the `metadata` feature is
//! enabled.

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

use crate::codepoint_map::REVERSE_GLYPHS;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));

/// Name prefix of the filled style; `fluent-` names are regular.
const FILLED_PREFIX: &str = "fluent-filled-";

/// Metadata registry for the Fluent UI font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from filename-derived keywords and the vendored alias
/// table.
pub struct FluentMetadata;

impl GlyphMetadata for FluentMetadata {
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

/// Ordering tables derived once from the generated codemap.
///
/// Fluent has no upstream ordering JSON, so the tables are computed
/// lazily from `REVERSE_GLYPHS` (all exported names) plus the
/// `fluent-filled-` style prefix instead of being codegen'd.
struct OrderingTables {
    /// Alphabetical index over the *base* glyphs (regular style) -
    /// variants sort next to their base via `base_of`.
    order: HashMap<&'static str, u32>,
    /// Style label per glyph name.
    group_of: HashMap<&'static str, GroupLabel>,
    /// `Some(regular_name)` for filled glyphs whose pair exists.
    base_of: HashMap<&'static str, &'static str>,
    /// `regular_name -> [filled_name]` for every pair.
    alternates_of: HashMap<&'static str, &'static [&'static str]>,
    /// One group per style with alphabetically sorted glyph names;
    /// `regular` first - it is the upstream default style.
    groups: &'static [GlyphGroup],
}

/// Returns the process-cached [`OrderingTables`].
fn tables() -> &'static OrderingTables {
    static TABLES: OnceLock<OrderingTables> = OnceLock::new();
    TABLES.get_or_init(build_tables)
}

/// Builds the ordering tables from the generated codemap.
fn build_tables() -> OrderingTables {
    let mut regular: Vec<&'static str> = Vec::new();
    let mut filled: Vec<&'static str> = Vec::new();
    for (&name, _) in REVERSE_GLYPHS.entries() {
        if name.starts_with(FILLED_PREFIX) {
            filled.push(name)
        } else {
            regular.push(name)
        }
    }
    regular.sort_unstable();
    filled.sort_unstable();

    let order: HashMap<&'static str, u32> = regular.iter().enumerate().map(|(i, &name)| (name, i as u32)).collect();
    let mut group_of = HashMap::with_capacity(regular.len() + filled.len());
    for &name in &regular {
        group_of.insert(name, GroupLabel::new("regular"));
    }
    for &name in &filled {
        group_of.insert(name, GroupLabel::new("filled"));
    }

    let mut base_of = HashMap::with_capacity(filled.len());
    let mut alternates_of = HashMap::with_capacity(filled.len());
    for &name in &filled {
        let stem = name.strip_prefix(FILLED_PREFIX).unwrap_or(name);
        let base_name: &'static str = Box::leak(format!("fluent-{stem}").into_boxed_str());
        if regular.contains(&base_name) {
            base_of.insert(name, base_name);
            alternates_of.insert(base_name, Box::leak(Box::new([name])) as &'static [&'static str]);
        }
    }

    let groups: &'static [GlyphGroup] = Box::leak(
        vec![
            GlyphGroup::new(GroupLabel::new("regular"), Box::leak(regular.into_boxed_slice())),
            GlyphGroup::new(GroupLabel::new("filled"), Box::leak(filled.into_boxed_slice())),
        ]
        .into_boxed_slice(),
    );

    OrderingTables {
        order,
        group_of,
        base_of,
        alternates_of,
        groups,
    }
}

impl GlyphOrdering for FluentMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Style of a glyph: `fluent-filled-{name}-symbolic` -> "filled",
    /// `fluent-{name}-symbolic` -> "regular".
    fn group(&self, name: &str) -> Option<GroupLabel> {
        tables().group_of.get(name).copied()
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        static LABELS: &[GroupLabel] = &[GroupLabel::new("regular"), GroupLabel::new("filled")];
        LABELS
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        tables().groups
    }

    /// The regular glyph a filled glyph variants:
    /// `fluent-filled-{name}-symbolic` -> `fluent-{name}-symbolic`.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        tables().base_of.get(name).copied()
    }

    /// The filled variant of a regular glyph, if it exists.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        tables().alternates_of.get(name).copied().unwrap_or(&[])
    }

    /// Both styles are first-class browse content - a whole style is
    /// not a hideable variant (unlike emoji skin tones). The pair
    /// link still powers the detail sidebar's Variants section and
    /// keeps filled glyphs sorted next to their regular base.
    fn collapse_variants(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_group_regular() {
        let group = FluentMetadata.group("fluent-add-symbolic");
        assert_eq!(group.map(|g| g.as_str()), Some("regular"));
    }

    #[test]
    fn ordering_group_filled() {
        let group = FluentMetadata.group("fluent-filled-add-symbolic");
        assert_eq!(group.map(|g| g.as_str()), Some("filled"));
    }

    #[test]
    fn ordering_base_of_pairs_filled() {
        assert_eq!(FluentMetadata.base_of("fluent-filled-add-symbolic"), Some("fluent-add-symbolic"));
    }

    #[test]
    fn ordering_alternates_contains_filled() {
        let alternates = FluentMetadata.alternates("fluent-add-symbolic");
        assert!(alternates.contains(&"fluent-filled-add-symbolic"));
    }

    #[test]
    fn ordering_group_labels_cover_styles() {
        let labels = FluentMetadata.group_labels();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].as_str(), "regular");
        assert_eq!(labels[1].as_str(), "filled");
    }

    #[test]
    fn ordering_groups_partition_all_glyphs() {
        let total: usize = FluentMetadata.groups().iter().map(|g| g.glyphs.len()).sum();
        let all = REVERSE_GLYPHS.entries().count();
        assert_eq!(total, all, "groups should partition every glyph");
    }
}
