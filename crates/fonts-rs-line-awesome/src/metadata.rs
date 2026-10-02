//! Icon metadata for Line Awesome.
//!
//! Keywords are filename-derived (the kebab stem splits into word
//! tokens), categories carry the Font Awesome 5 style (`solid`,
//! `regular`, `brands`), and aliases merge the vendored FA4-compat
//! `*-o` and `fa-*` rename table - all generated as `phf::Map`s by
//! `build.rs` when the `metadata` feature is enabled.
//!
//! [`GlyphOrdering`] groups glyphs by their style category (the cheat
//! sheet's group dropdown) and treats the solid face of a regular/
//! solid codepoint pair as a variant of the regular base glyph.
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

/// Metadata registry for the Line Awesome font family.
///
/// Implements [`GlyphMetadata`] using build-time generated static maps
/// sourced from filename conventions and the vendored `aliases.json`.
pub struct LineAwesomeMetadata;

impl GlyphMetadata for LineAwesomeMetadata {
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

/// Style groups in canonical order: regular, then solid, then brands.
static GROUP_ORDER: [GroupLabel; 3] = [GroupLabel::new("regular"), GroupLabel::new("solid"), GroupLabel::new("brands")];

/// Ordering tables derived once from the generated maps.
///
/// Line Awesome has no upstream ordering JSON, so the tables are
/// computed lazily from `REVERSE_GLYPHS` (all exported names) and
/// `CATEGORIES` (style per name) instead of being codegen'd.
struct OrderingTables {
    /// Alphabetical index per base glyph (paired solid variants are
    /// excluded - they sort through their base via `base_of`).
    order: HashMap<&'static str, u32>,
    /// `la-{stem}-solid-symbolic` -> `la-{stem}-symbolic` for the
    /// regular/solid pairs sharing one codepoint.
    base_of: HashMap<&'static str, &'static str>,
    /// `la-{stem}-symbolic` -> `[la-{stem}-solid-symbolic]` inverse.
    alternates: HashMap<&'static str, &'static [&'static str]>,
    /// Style groups with alphabetically sorted glyph names.
    groups: [GlyphGroup; 3],
}

/// Returns the process-cached [`OrderingTables`].
fn tables() -> &'static OrderingTables {
    static TABLES: OnceLock<OrderingTables> = OnceLock::new();
    TABLES.get_or_init(build_tables)
}

/// Builds the ordering tables from the generated glyph maps.
///
/// The small slices are intentionally leaked: [`GlyphOrdering`]
/// returns `&'static` references, so the tables live for the process
/// lifetime.
fn build_tables() -> OrderingTables {
    let names: Vec<&'static str> = REVERSE_GLYPHS.entries().map(|(name, _)| *name).collect();

    // Strip the `la-` prefix and `-symbolic` suffix:
    // `la-heart-solid-symbolic` -> `heart-solid`.
    let stem_of = |name: &'static str| -> &'static str { name.strip_prefix("la-").and_then(|s| s.strip_suffix("-symbolic")).unwrap_or(name) };

    // Regular/solid pairs: `la-{stem}-solid-symbolic` is a variant of
    // `la-{stem}-symbolic` when the plain spelling exists.
    let mut base_of: HashMap<&'static str, &'static str> = HashMap::new();
    for name in &names {
        if let Some(stem) = stem_of(name).strip_suffix("-solid") {
            let plain = format!("la-{stem}-symbolic");
            if let Some(&base) = names.iter().find(|candidate| **candidate == plain) {
                base_of.insert(*name, base);
            }
        }
    }

    let mut alternates: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    for (variant, base) in &base_of {
        alternates.entry(*base).or_default().push(*variant);
    }
    let alternates: HashMap<&'static str, &'static [&'static str]> = alternates
        .into_iter()
        .map(|(base, mut v)| {
            v.sort();
            (base, &*Box::leak(v.into_boxed_slice()))
        })
        .collect();

    // Alphabetical index over base glyphs only.
    let mut bases: Vec<&'static str> = names.iter().copied().filter(|name| !base_of.contains_key(*name)).collect();
    bases.sort();
    let order: HashMap<&'static str, u32> = bases.iter().enumerate().map(|(index, name)| (*name, index as u32)).collect();

    // Style groups: every glyph is filed under its style category,
    // sorted alphabetically within the group.
    let metadata = LineAwesomeMetadata;
    let groups = GROUP_ORDER.map(|label| {
        let mut glyphs: Vec<&'static str> = names
            .iter()
            .copied()
            .filter(|name| metadata.categories(*name).iter().any(|c| c.as_str() == label.as_str()))
            .collect();
        glyphs.sort();
        GlyphGroup::new(label, Box::leak(glyphs.into_boxed_slice()))
    });

    OrderingTables {
        order,
        base_of,
        alternates,
        groups,
    }
}

impl GlyphOrdering for LineAwesomeMetadata {
    fn order_index(&self, name: &str) -> Option<u32> {
        tables().order.get(name).copied()
    }

    /// Style group of a glyph: `regular`, `solid` or `brands`.
    fn group(&self, name: &str) -> Option<GroupLabel> {
        self.categories(name).first().map(|c| GroupLabel::new(c.as_str()))
    }

    fn group_labels(&self) -> &'static [GroupLabel] {
        &GROUP_ORDER
    }

    fn groups(&self) -> &'static [GlyphGroup] {
        &tables().groups
    }

    /// The solid face of a regular/solid codepoint pair.
    fn alternates(&self, name: &str) -> &'static [&'static str] {
        tables().alternates.get(name).copied().unwrap_or_default()
    }

    /// `la-{stem}-solid-symbolic` -> `la-{stem}-symbolic` for paired
    /// glyphs; solid-only and brand glyphs return `None`.
    fn base_of(&self, name: &str) -> Option<&'static str> {
        tables().base_of.get(name).copied()
    }
}
