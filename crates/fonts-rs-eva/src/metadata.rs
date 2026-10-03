//! Icon metadata: keywords, style categories and ordering.
//!
//! Provides keyword lookup for Eva icons, enabling search beyond
//! simple name matching - e.g. "alarm" finds `eva-bell-symbolic` and
//! `eva-clock-symbolic`.
//!
//! Keywords are sourced from upstream `icons-tags.json` (converted
//! from `icons-tags.ts`, the search tags powering the official
//! eva-icons.com cheat sheet). Categories carry the style ("fill" /
//! "outline"); Eva ships no aliases.
//!
//! [`GlyphOrdering`] groups glyphs by style and pairs
//! `eva-{name}-outline-symbolic` onto its fill base
//! `eva-{name}-symbolic` - the link powers the detail sidebar's
//! Variants section and sorts each outline glyph next to its fill
//! base (styles never collapse).
//!
//! This module is only available when the `metadata` feature is enabled.

use std::collections::HashMap;
use std::sync::OnceLock;

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

/// Metadata registry for the Eva Icons font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from upstream `icons-tags.json`.
pub struct EvaMetadata;

impl GlyphMetadata for EvaMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }
}

/// Ordering tables derived once from the generated codemap.
///
/// Eva has no upstream ordering JSON, so the tables are computed
/// lazily from `REVERSE_GLYPHS` (all exported names) plus the
/// `-outline` style suffix instead of being codegen'd.
struct OrderingTables {
    /// Alphabetical index over the *base* glyphs (fill style) -
    /// variants sort next to their base via `base_of`.
    order: HashMap<&'static str, u32>,
    /// Style label per glyph name.
    group_of: HashMap<&'static str, GroupLabel>,
    /// `Some(fill_name)` for outline glyphs whose fill pair exists.
    base_of: HashMap<&'static str, &'static str>,
    /// `fill_name -> [outline_name]` for every outline pair.
    alternates_of: HashMap<&'static str, &'static [&'static str]>,
    /// One group per style with alphabetically sorted glyph names;
    /// `fill` first - it is the upstream default style.
    groups: &'static [GlyphGroup],
}

/// Returns the process-cached [`OrderingTables`].
fn tables() -> &'static OrderingTables {
    static TABLES: OnceLock<OrderingTables> = OnceLock::new();
    TABLES.get_or_init(build_tables)
}

/// Builds the ordering tables from the generated codemap.
fn build_tables() -> OrderingTables {
    let is_outline = |name: &str| name.strip_prefix("eva-").is_some_and(|n| n.ends_with("-outline-symbolic"));

    let mut fill: Vec<&'static str> = Vec::new();
    let mut outline: Vec<&'static str> = Vec::new();
    for (&name, _) in REVERSE_GLYPHS.entries() {
        if is_outline(name) { outline.push(name) } else { fill.push(name) }
    }
    fill.sort_unstable();

    let order: HashMap<&'static str, u32> = fill.iter().enumerate().map(|(i, &name)| (name, i as u32)).collect();
    let mut group_of = HashMap::with_capacity(fill.len() + outline.len());
    for &name in &fill {
        group_of.insert(name, GroupLabel::new("fill"));
    }
    for &name in &outline {
        group_of.insert(name, GroupLabel::new("outline"));
    }
    outline.sort_unstable();

    let mut base_of = HashMap::with_capacity(outline.len());
    let mut alternates_of = HashMap::with_capacity(outline.len());
    for &name in &outline {
        let stem = name
            .strip_suffix("-outline-symbolic")
            .unwrap_or_else(|| name.strip_suffix("-symbolic").unwrap_or(name));
        let base_name: &'static str = Box::leak(format!("{stem}-symbolic").into_boxed_str());
        if fill.contains(&base_name) {
            base_of.insert(name, base_name);
            alternates_of.insert(base_name, Box::leak(Box::new([name])) as &'static [&'static str]);
        }
    }

    let groups: &'static [GlyphGroup] = Box::leak(
        vec![
            GlyphGroup::new(GroupLabel::new("fill"), Box::leak(fill.into_boxed_slice())),
            GlyphGroup::new(GroupLabel::new("outline"), Box::leak(outline.into_boxed_slice())),
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

impl GlyphOrdering for EvaMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Style of a glyph: `eva-{name}-outline-symbolic` -> "outline",
    /// `eva-{name}-symbolic` -> "fill".
    fn group(&self, name: &str) -> Option<GroupLabel> {
        tables().group_of.get(name).copied()
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        static LABELS: &[GroupLabel] = &[GroupLabel::new("fill"), GroupLabel::new("outline")];
        LABELS
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        tables().groups
    }

    /// The fill glyph an outline glyph variants:
    /// `eva-{name}-outline-symbolic` -> `eva-{name}-symbolic`.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        tables().base_of.get(name).copied()
    }

    /// The outline variant of a fill glyph, if it exists.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        tables().alternates_of.get(name).copied().unwrap_or(&[])
    }

    /// Both styles are first-class browse content - a whole style is
    /// not a hideable variant (unlike emoji skin tones). The pair
    /// link still powers the detail sidebar's Variants section and
    /// keeps outline glyphs sorted next to their fill base.
    fn collapse_variants(&self) -> bool {
        false
    }
}
