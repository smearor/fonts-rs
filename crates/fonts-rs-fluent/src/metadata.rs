//! Icon metadata: keywords and aliases for search functionality.
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
//! Fluent ships no categories upstream, so the category map is empty.
//!
//! This module is only available when the `metadata` feature is
//! enabled.

use fonts_rs_model::GlyphAliasMap;
use fonts_rs_model::GlyphCategoryMap;
use fonts_rs_model::GlyphKeyword;
use fonts_rs_model::GlyphKeywordMap;
use fonts_rs_model::GlyphMetadata;

// Include the build-time generated phf::Map constants.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));
include!(concat!(env!("OUT_DIR"), "/categories.rs"));
include!(concat!(env!("OUT_DIR"), "/aliases.rs"));

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
