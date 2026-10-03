//! Glyph metadata: chess piece categories and style ordering.
//!
//! Provides category lookup for Cuernavaca glyphs: every piece glyph
//! belongs to `king`, `queen`, `rook`, `bishop`, `knight`, or `pawn`.
//! The upside-down pieces keep their piece category - `cuernavaca-a`
//! (white upside-down bishop) is a `bishop`, `cuernavaca-d`/`f`
//! (rook+knight fairy, used as upside-down rook) is a `rook`. Square
//! and diagram-border glyphs carry no category.
//!
//! [`GlyphOrdering`] groups the pieces into four styles and links the
//! non-base styles onto the `light normal` base:
//!
//! - `light normal` - white pieces, upright (`cuernavaca-k` .. `p`)
//! - `dark normal` - black pieces, upright (`cuernavaca-l` .. `o`)
//! - `light upside-down` - white pieces, rotated (`cuernavaca-a`/`d`/`g`)
//! - `dark upside-down` - black pieces, rotated (`cuernavaca-f`/`h`/`s`)
//!
//! Semantically the rotated glyphs are fairy pieces in chess-problem
//! convention - a bishop+knight (archbishop), a rook+knight
//! (chancellor), and a generic fairy knight (nightrider) - which is
//! why only bishop, knight, and rook exist upside-down: king, queen,
//! and pawn carry no canonical fairy role.
//!
//! The link powers the detail sidebar's Variants section, sorts each
//! variant next to its base, and collapses the variants behind the
//! base in the browse grid.
//!
//! Categories are generated at build time from the glyph names -
//! Cuernavaca ships no upstream keyword, category, or alias source,
//! so the keyword and alias maps stay empty.
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

/// Metadata registry for the Chess Cuernavaca font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// derived from the glyph names.
pub struct CuernavacaMetadata;

/// Piece table: glyph name stem -> style group, base stem, piece
/// rank. Light-normal pieces are the bases (no `Some` base); dark and
/// upside-down pieces link onto their light-normal counterpart.
///
/// Piece rank follows the canonical order king, queen, rook, bishop,
/// knight, pawn.
const PIECES: [(&str, &str, Option<&str>, u32); 18] = [
    // light normal (white, upright) - the bases
    ("k", "light normal", None, 0),
    ("q", "light normal", None, 1),
    ("r", "light normal", None, 2),
    ("b", "light normal", None, 3),
    ("n", "light normal", None, 4),
    ("p", "light normal", None, 5),
    // dark normal (black, upright)
    ("l", "dark normal", Some("k"), 0),
    ("w", "dark normal", Some("q"), 1),
    ("t", "dark normal", Some("r"), 2),
    ("v", "dark normal", Some("b"), 3),
    ("m", "dark normal", Some("n"), 4),
    ("o", "dark normal", Some("p"), 5),
    // light upside-down (white, rotated)
    ("a", "light upside-down", Some("b"), 3),
    ("d", "light upside-down", Some("r"), 2),
    ("g", "light upside-down", Some("n"), 4),
    // dark upside-down (black, rotated)
    ("f", "dark upside-down", Some("r"), 2),
    ("h", "dark upside-down", Some("n"), 4),
    ("s", "dark upside-down", Some("b"), 3),
];

/// Style group labels in canonical order.
const STYLE_LABELS: [&str; 4] = ["light normal", "dark normal", "light upside-down", "dark upside-down"];

/// Ordering tables derived once from the generated codemap.
///
/// Cuernavaca has no upstream ordering source, so the tables are
/// computed lazily from `REVERSE_GLYPHS` (all exported names) plus
/// the [`PIECES`] table instead of being codegen'd.
struct OrderingTables {
    /// Canonical index over base/standalone glyphs - variants sort
    /// next to their base via `base_of`.
    order: HashMap<&'static str, u32>,
    /// Style label per glyph name, variants included.
    group_of: HashMap<&'static str, GroupLabel>,
    /// `light normal` base name for dark and upside-down pieces.
    base_of: HashMap<&'static str, &'static str>,
    /// Base name -> the other styles of that piece, in canonical
    /// order (dark normal, light upside-down, dark upside-down).
    alternates_of: HashMap<&'static str, &'static [&'static str]>,
    /// One group per style with pieces in canonical order.
    groups: &'static [GlyphGroup],
}

/// Returns the [`PIECES`] entry for a glyph name, or `None` for
/// square and border glyphs.
fn piece_info(name: &str) -> Option<&'static (&'static str, &'static str, Option<&'static str>, u32)> {
    let stem = name.strip_prefix("cuernavaca-")?;
    PIECES.iter().find(|(piece_stem, ..)| *piece_stem == stem)
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

    // Style variants: every dark/upside-down piece links onto its
    // light-normal base (dropped if that base is not exported).
    let mut base_of = HashMap::new();
    let mut alternates_of: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let mut group_of = HashMap::with_capacity(names.len());
    let mut info_of: HashMap<&'static str, (usize, u32)> = HashMap::new();
    for &name in &names {
        let Some(&(_, group, base, piece_rank)) = piece_info(name) else { continue };
        let style_rank = STYLE_LABELS.iter().position(|label| *label == group).unwrap_or(usize::MAX);
        info_of.insert(name, (style_rank, piece_rank));
        group_of.insert(name, GroupLabel::new(group));
        let Some(base_stem) = base else { continue };
        let Some(&base_name) = names.iter().find(|n| n.strip_prefix("cuernavaca-") == Some(base_stem)) else {
            continue;
        };
        base_of.insert(name, base_name);
        alternates_of.entry(base_name).or_default().push(name);
    }
    // Canonical variant order follows STYLE_LABELS (dark normal,
    // light upside-down, dark upside-down), not name order.
    for alternates in alternates_of.values_mut() {
        alternates.sort_by_key(|name| info_of.get(name).map(|(style_rank, _)| *style_rank).unwrap_or(usize::MAX));
    }

    // Canonical order over bases and non-piece glyphs: pieces first
    // (king, queen, rook, bishop, knight, pawn), then the remaining
    // square and border glyphs by name.
    let mut bases: Vec<&'static str> = names.iter().copied().filter(|name| !base_of.contains_key(name)).collect();
    bases.sort_by(|a, b| {
        let rank = |name: &&'static str| info_of.get(name).map(|(_, piece_rank)| *piece_rank).unwrap_or(u32::MAX);
        rank(a).cmp(&rank(b)).then(a.cmp(b))
    });
    let order: HashMap<&'static str, u32> = bases.iter().enumerate().map(|(index, &name)| (name, index as u32)).collect();

    // Picker layout per style group, pieces in canonical order.
    let mut groups_vec = Vec::with_capacity(STYLE_LABELS.len());
    for label in STYLE_LABELS {
        let mut members: Vec<&'static str> = group_of.iter().filter(|(_, group)| group.as_str() == label).map(|(&name, _)| name).collect();
        members.sort_by_key(|name| info_of.get(name).map(|(_, piece_rank)| *piece_rank).unwrap_or(u32::MAX));
        if !members.is_empty() {
            groups_vec.push(GlyphGroup::new(GroupLabel::new(label), Box::leak(members.into_boxed_slice())));
        }
    }

    let alternates_of: HashMap<&'static str, &'static [&'static str]> = alternates_of
        .into_iter()
        .map(|(base, alts)| (base, Box::leak(alts.into_boxed_slice()) as &'static [&'static str]))
        .collect();

    OrderingTables {
        order,
        group_of,
        base_of,
        alternates_of,
        groups: Box::leak(groups_vec.into_boxed_slice()),
    }
}

impl GlyphMetadata for CuernavacaMetadata {
    fn keyword_map(&self) -> &GlyphKeywordMap {
        &KEYWORDS
    }

    fn category_map(&self) -> &GlyphCategoryMap {
        &CATEGORIES
    }
}

impl GlyphOrdering for CuernavacaMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Style of a piece glyph: `cuernavaca-l` -> "dark normal",
    /// `cuernavaca-a` -> "light upside-down".
    fn group(&self, name: &str) -> Option<GroupLabel> {
        tables().group_of.get(name).copied()
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        static LABELS: &[GroupLabel] = &[
            GroupLabel::new("light normal"),
            GroupLabel::new("dark normal"),
            GroupLabel::new("light upside-down"),
            GroupLabel::new("dark upside-down"),
        ];
        LABELS
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        tables().groups
    }

    /// The `light normal` piece a dark or upside-down style collapses
    /// onto: `cuernavaca-l` -> `cuernavaca-k`, `cuernavaca-a` ->
    /// `cuernavaca-b`.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        tables().base_of.get(name).copied()
    }

    /// The other styles of a `light normal` base piece: `cuernavaca-r`
    /// -> `cuernavaca-t`, `cuernavaca-d`, `cuernavaca-f`.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        tables().alternates_of.get(name).copied().unwrap_or(&[])
    }
}
