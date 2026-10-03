//! Glyph metadata: die-type categories and d6 style ordering.
//!
//! Provides category lookup for Dicefont glyphs, grouping each die
//! face by its die type: `dicefont-d20-4` -> "d20",
//! `dicefont-dot-d6-2` -> "d6". The dotted d6 variants keep their
//! "d6" category - they are six-sided dice. The Fudge dice
//! (`dicefont-f`, `dicefont-f-*`) land in "fudge".
//!
//! Categories are generated at build time from the glyph names -
//! Dicefont ships no upstream keyword, category, or alias source, so
//! the keyword and alias maps stay empty.
//!
//! [`GlyphOrdering`] treats `dicefont-solid-small-dot-d6-{face}` as
//! the base d6 style and links the other three d6 styles
//! (`d6-{face}`, `dot-d6-{face}`, `small-dot-d6-{face}`) onto it -
//! the link powers the detail sidebar's Variants section, sorts each
//! variant next to its base, and collapses the variants behind the
//! base in the browse grid.
//!
//! This module is only available when the `metadata` feature is enabled.

use std::collections::HashMap;
use std::sync::OnceLock;

use fonts_rs_model::GlyphCategory;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphGroup;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;
use fonts_rs_model::GlyphOrdering;
use fonts_rs_model::GroupLabel;

use crate::codepoint_map::REVERSE_GLYPHS;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));

/// Metadata registry for the Dicefont font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// derived from the glyph names.
pub struct DicefontMetadata;

impl GlyphMetadata for DicefontMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }
}

/// Name-stem prefixes of the three non-base d6 styles, in canonical
/// variant order for the sidebar's Variants section.
const D6_VARIANT_STEMS: [&str; 3] = ["d6-", "dot-d6-", "small-dot-d6-"];

/// Name-stem prefix of the base d6 style.
const D6_BASE_STEM: &str = "solid-small-dot-d6-";

/// Die-type name segments (mirrors `DICE_CATEGORIES` in `build.rs`;
/// `f` renders as the "fudge" label).
const DIE_SEGMENTS: [&str; 8] = ["d2", "d4", "d6", "d8", "d10", "d12", "d20", "f"];

/// Group labels in canonical dice order.
const DIE_LABELS: [&str; 8] = ["d2", "d4", "d6", "d8", "d10", "d12", "d20", "fudge"];

/// Ordering tables derived once from the generated codemap.
///
/// Dicefont has no upstream ordering source, so the tables are
/// computed lazily from `REVERSE_GLYPHS` (all exported names) plus
/// the d6 style stems instead of being codegen'd.
struct OrderingTables {
    /// Canonical index over base/standalone glyphs - variants sort
    /// next to their base via `base_of`.
    order: HashMap<&'static str, u32>,
    /// Die-type label per glyph name, variants included.
    group_of: HashMap<&'static str, GroupLabel>,
    /// `solid-small-dot` base name for the other d6 styles.
    base_of: HashMap<&'static str, &'static str>,
    /// Base name -> the other three d6 styles, in canonical order.
    alternates_of: HashMap<&'static str, &'static [&'static str]>,
    /// One group per die type with variants inlined after their base.
    groups: &'static [GlyphGroup],
}

/// Returns the die-type label for a glyph name, or `None`.
fn die_label(name: &'static str) -> Option<&'static str> {
    let stem = name.strip_prefix("dicefont-").unwrap_or(name);
    let segment = stem.split('-').find(|s| DIE_SEGMENTS.contains(s))?;
    Some(if segment == "f" { "fudge" } else { segment })
}

/// Returns the canonical rank of a glyph's die type, `usize::MAX`
/// for unlabeled names.
fn die_rank(name: &'static str) -> usize {
    die_label(name).and_then(|label| DIE_LABELS.iter().position(|l| *l == label)).unwrap_or(usize::MAX)
}

/// Returns the face number suffix of a glyph name, `0` for names
/// without one (bare die glyphs like `dicefont-d6`).
fn face_number(name: &str) -> u32 {
    name.rsplit('-').next().and_then(|s| s.parse().ok()).unwrap_or(0)
}

/// Returns the process-cached [`OrderingTables`].
fn tables() -> &'static OrderingTables {
    static TABLES: OnceLock<OrderingTables> = OnceLock::new();
    TABLES.get_or_init(build_tables)
}

/// Builds the ordering tables from the generated codemap.
fn build_tables() -> OrderingTables {
    let mut names: Vec<&'static str> = REVERSE_GLYPHS.entries().map(|(&name, _)| name).collect();
    names.sort_unstable();

    // d6 style variants: `dicefont-{style}-d6-{face}` links onto the
    // `dicefont-solid-small-dot-d6-{face}` base. Bare names without a
    // face suffix (dicefont-d6, dicefont-dot-d6) have no base glyph -
    // they stay standalone.
    let mut base_of = HashMap::new();
    let mut alternates_of: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    for &name in &names {
        let Some(stem) = name.strip_prefix("dicefont-") else { continue };
        let Some(face) = D6_VARIANT_STEMS.iter().find_map(|prefix| stem.strip_prefix(prefix)) else {
            continue;
        };
        let base: &'static str = Box::leak(format!("dicefont-{D6_BASE_STEM}{face}").into_boxed_str());
        if REVERSE_GLYPHS.get(base).is_some() {
            base_of.insert(name, base);
            alternates_of.entry(base).or_default().push(name);
        }
    }
    // Canonical variant order follows D6_VARIANT_STEMS, not the
    // alphabetical name order produced above.
    for alternates in alternates_of.values_mut() {
        alternates.sort_by_key(|name| {
            D6_VARIANT_STEMS
                .iter()
                .position(|prefix| name.strip_prefix("dicefont-").is_some_and(|stem| stem.starts_with(prefix)))
                .unwrap_or(D6_VARIANT_STEMS.len())
        });
    }

    // Die-type label per glyph name.
    let mut group_of = HashMap::with_capacity(names.len());
    for &name in &names {
        if let Some(label) = die_label(name) {
            group_of.insert(name, GroupLabel::new(label));
        }
    }

    // Canonical order over bases and standalone glyphs: dice in
    // pip-count order (d2..d20, fudge last), bare die names first,
    // then faces ascending.
    let mut bases: Vec<&'static str> = names.iter().copied().filter(|name| !base_of.contains_key(name)).collect();
    bases.sort_by(|a, b| die_rank(a).cmp(&die_rank(b)).then(face_number(a).cmp(&face_number(b))).then(a.cmp(b)));
    let order: HashMap<&'static str, u32> = bases.iter().enumerate().map(|(index, &name)| (name, index as u32)).collect();

    // Picker layout per die group, variants inlined after their base.
    let mut groups_vec = Vec::with_capacity(DIE_LABELS.len());
    for label in DIE_LABELS {
        let mut group_names: Vec<&'static str> = Vec::new();
        for &base in &bases {
            if group_of.get(&base) != Some(&GroupLabel::new(label)) {
                continue;
            }
            group_names.push(base);
            if let Some(alternates) = alternates_of.get(&base) {
                group_names.extend(alternates.iter().copied());
            }
        }
        if !group_names.is_empty() {
            groups_vec.push(GlyphGroup::new(GroupLabel::new(label), Box::leak(group_names.into_boxed_slice())));
        }
    }

    let alternates_of: HashMap<&'static str, &'static [&'static str]> =
        alternates_of.into_iter().map(|(base, alts)| (base, Box::leak(alts.into_boxed_slice()) as &'static [&'static str])).collect();

    OrderingTables { order, group_of, base_of, alternates_of, groups: Box::leak(groups_vec.into_boxed_slice()) }
}

impl GlyphOrdering for DicefontMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Die type of a glyph: `dicefont-dot-d6-3` -> "d6",
    /// `dicefont-f-minus` -> "fudge".
    fn group(&self, name: &str) -> Option<GroupLabel> {
        tables().group_of.get(name).copied()
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        static LABELS: &[GroupLabel] = &[
            GroupLabel::new("d2"),
            GroupLabel::new("d4"),
            GroupLabel::new("d6"),
            GroupLabel::new("d8"),
            GroupLabel::new("d10"),
            GroupLabel::new("d12"),
            GroupLabel::new("d20"),
            GroupLabel::new("fudge"),
        ];
        LABELS
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        tables().groups
    }

    /// The `solid-small-dot` face a d6 style variant collapses onto:
    /// `dicefont-d6-1` -> `dicefont-solid-small-dot-d6-1`.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        tables().base_of.get(name).copied()
    }

    /// The three other d6 styles of a `solid-small-dot` base face.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        tables().alternates_of.get(name).copied().unwrap_or(&[])
    }
}
