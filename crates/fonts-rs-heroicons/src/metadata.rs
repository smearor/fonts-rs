//! Icon metadata: keywords, style categories, aliases and ordering.
//!
//! Provides keyword and alias lookup for Heroicons icons, enabling
//! search beyond simple name matching - e.g. "stars" finds
//! `heroicons-sparkles-symbolic`.
//!
//! Data is sourced from the heroicons.com site bundle (keyword map,
//! vendored as `resources/metadata/keywords.json`) and the iconify
//! heroicons set's v1→v2 alias table (vendored as
//! `resources/metadata/aliases.json`). Categories carry the style
//! ("outline" / "solid" / "mini" / "micro").
//!
//! [`GlyphOrdering`] groups glyphs by style and pairs each
//! `heroicons-{name}-{style}-symbolic` variant onto its outline base
//! `heroicons-{name}-symbolic`, so the cheat sheet can collapse
//! variants like other style families.
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

use crate::codepoint_map::REVERSE_GLYPHS;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));

/// Non-outline styles in canonical order (upstream size progression:
/// 24px filled, 20px, 16px). Outline is the default style.
const STYLES: &[&str] = &["solid", "mini", "micro"];

/// Metadata registry for the Heroicons font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from the vendored keyword and alias tables.
pub struct HeroiconsMetadata;

impl GlyphMetadata for HeroiconsMetadata {
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
/// Heroicons has no upstream ordering JSON, so the tables are computed
/// lazily from `REVERSE_GLYPHS` (all exported names) plus the
/// `-{style}` name infix instead of being codegen'd.
struct OrderingTables {
    /// Alphabetical index over the *base* glyphs (outline style) -
    /// variants sort next to their base via `base_of`.
    order: HashMap<&'static str, u32>,
    /// Style label per glyph name.
    group_of: HashMap<&'static str, GroupLabel>,
    /// `Some(outline_name)` for style variants whose base exists.
    base_of: HashMap<&'static str, &'static str>,
    /// `outline_name -> [variant_names]` in `STYLES` order.
    alternates_of: HashMap<&'static str, &'static [&'static str]>,
    /// One group per style with alphabetically sorted glyph names;
    /// `outline` first - it is the upstream default style.
    groups: &'static [GlyphGroup],
}

/// Returns the process-cached [`OrderingTables`].
fn tables() -> &'static OrderingTables {
    static TABLES: OnceLock<OrderingTables> = OnceLock::new();
    TABLES.get_or_init(build_tables)
}

/// Splits `heroicons-{stem}[-{style}]-symbolic` into `(stem, style)`;
/// `style` is `None` for the unsuffixed outline style.
fn style_of(name: &str) -> Option<(&str, Option<&'static str>)> {
    let stem = name.strip_prefix("heroicons-")?.strip_suffix("-symbolic")?;
    for style in STYLES {
        if let Some(base) = stem.strip_suffix(&format!("-{style}")) {
            return Some((base, Some(style)));
        }
    }
    Some((stem, None))
}

/// Builds the ordering tables from the generated codemap.
fn build_tables() -> OrderingTables {
    let mut by_style: HashMap<Option<&'static str>, Vec<&'static str>> = HashMap::new();
    for (&name, _) in REVERSE_GLYPHS.entries() {
        if let Some((_, style)) = style_of(name) {
            by_style.entry(style).or_default().push(name);
        }
    }

    let mut outline = by_style.remove(&None).unwrap_or_default();
    outline.sort_unstable();
    let order: HashMap<&'static str, u32> = outline.iter().enumerate().map(|(i, &name)| (name, i as u32)).collect();

    let mut group_of = HashMap::new();
    let mut base_of = HashMap::new();
    let mut alternates: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    for &name in &outline {
        group_of.insert(name, GroupLabel::new("outline"));
    }

    let mut groups = vec![GlyphGroup::new(GroupLabel::new("outline"), Box::leak(outline.clone().into_boxed_slice()))];
    for style in STYLES {
        let mut names = by_style.remove(&Some(*style)).unwrap_or_default();
        names.sort_unstable();
        for &name in &names {
            group_of.insert(name, GroupLabel::new(style));
            let Some((stem, _)) = style_of(name) else { continue };
            let base_name: &'static str = Box::leak(format!("heroicons-{stem}-symbolic").into_boxed_str());
            if order.contains_key(base_name) {
                base_of.insert(name, base_name);
                alternates.entry(base_name).or_default().push(name);
            }
        }
        groups.push(GlyphGroup::new(GroupLabel::new(style), Box::leak(names.into_boxed_slice())));
    }

    let alternates_of: HashMap<&'static str, &'static [&'static str]> = alternates
        .into_iter()
        .map(|(base, names)| (base, Box::leak(names.into_boxed_slice()) as &'static [&'static str]))
        .collect();

    OrderingTables { order, group_of, base_of, alternates_of, groups: Box::leak(groups.into_boxed_slice()) }
}

impl GlyphOrdering for HeroiconsMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Style of a glyph: `heroicons-{name}-solid-symbolic` -> "solid",
    /// `heroicons-{name}-symbolic` -> "outline".
    fn group(&self, name: &str) -> Option<GroupLabel> {
        tables().group_of.get(name).copied()
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        static LABELS: &[GroupLabel] = &[
            GroupLabel::new("outline"),
            GroupLabel::new("solid"),
            GroupLabel::new("mini"),
            GroupLabel::new("micro"),
        ];
        LABELS
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        tables().groups
    }

    /// The outline glyph a style variant belongs to:
    /// `heroicons-{name}-solid-symbolic` -> `heroicons-{name}-symbolic`.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        tables().base_of.get(name).copied()
    }

    /// The style variants of an outline glyph (`solid`, `mini`,
    /// `micro` - in that order), if they exist.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        tables().alternates_of.get(name).copied().unwrap_or(&[])
    }

    /// All styles are first-class browse content - a whole style is
    /// not a hideable variant (unlike emoji skin tones). The pair
    /// link still powers the detail sidebar's Variants section and
    /// keeps style glyphs sorted next to their outline base.
    fn collapse_variants(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_group_outline() {
        let group = HeroiconsMetadata.group("heroicons-bell-symbolic");
        assert_eq!(group.map(|g| g.as_str()), Some("outline"));
    }

    #[test]
    fn ordering_group_styles() {
        assert_eq!(HeroiconsMetadata.group("heroicons-bell-solid-symbolic").map(|g| g.as_str()), Some("solid"));
        assert_eq!(HeroiconsMetadata.group("heroicons-bell-mini-symbolic").map(|g| g.as_str()), Some("mini"));
    }

    #[test]
    fn ordering_base_of_pairs_variant() {
        assert_eq!(
            HeroiconsMetadata.base_of("heroicons-bell-solid-symbolic"),
            Some("heroicons-bell-symbolic")
        );
    }

    #[test]
    fn ordering_alternates_contains_solid() {
        let alternates = HeroiconsMetadata.alternates("heroicons-bell-symbolic");
        assert!(alternates.contains(&"heroicons-bell-solid-symbolic"));
    }

    #[test]
    fn ordering_group_labels_cover_styles() {
        let labels = HeroiconsMetadata.group_labels();
        assert_eq!(labels.len(), 4);
        assert_eq!(labels[0].as_str(), "outline");
    }

    #[test]
    fn ordering_groups_partition_all_glyphs() {
        let total: usize = HeroiconsMetadata.groups().iter().map(|g| g.glyphs.len()).sum();
        let all = REVERSE_GLYPHS.entries().count();
        assert_eq!(total, all, "groups should partition every glyph");
    }
}
