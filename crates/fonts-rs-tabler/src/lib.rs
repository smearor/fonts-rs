//! Tabler Icons font integration for GTK 4 and pixel-drawing.
//!
//! Provides icon name resolution, GResource registration, and codepoint
//! lookup for the Tabler Icons font family - over 5,000 stroke-based
//! outline icons plus a filled variant, with semantic names, tags, and
//! categories from upstream `icons.json`.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_tabler::Tabler;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any icons
//! if let Err(e) = Tabler::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve icon names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_tabler::Tabler;
//!
//! let codepoint = Tabler::codepoint("tabler-a-b-2-symbolic");
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod definition;
mod family;
pub mod naming;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "render")]
pub mod fonts;

#[cfg(feature = "metadata")]
pub mod metadata;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::TablerConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Tabler;
pub use naming::TablerIconName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`TablerIconName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = TablerIconName;

/// Returns all known Tabler icons as `(CodePoint, icon_name)` pairs,
/// sorted alphabetically by icon name.
pub fn all_icons() -> Vec<(CodePoint, String)> {
    let mut icons: Vec<(CodePoint, String)> = codepoint_map::GLYPHS
        .entries()
        .map(|(cp, name)| (CodePoint::from(*cp), name.to_string()))
        .collect();
    icons.sort_by(|a, b| a.1.cmp(&b.1));
    icons
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns the first icon name in the active variant's glyph map.
    ///
    /// The filled variant ships a subset of the outline icons, so tests
    /// must derive names from the map instead of hardcoding them.
    fn first_icon_name() -> String {
        all_icons().into_iter().next().map(|(_, name)| name).expect("at least one icon should exist")
    }

    /// `codepoint`/`glyph_name` must round-trip for exported icons -
    /// the chain behind icon theme lookup and the cheat sheet.
    /// Codepoints differ between variants, so the test derives them
    /// from the name instead of hardcoding values.
    #[test]
    fn codepoint_glyph_name_round_trip() {
        let name = first_icon_name();
        let codepoint = Tabler::codepoint(&name).expect("icon codepoint should resolve");
        let resolved = Tabler::glyph_name(codepoint).expect("icon name should resolve");
        assert_eq!(resolved.as_ref(), name);
    }

    /// The `glyph_prefix` fallback must resolve suffix-only lookups:
    /// `codepoint("{name}-symbolic")` expands to the full
    /// `tabler-{name}-symbolic` name.
    #[test]
    fn codepoint_resolves_suffixed_name() {
        let name = first_icon_name();
        let stripped = name.strip_prefix("tabler-").expect("icon name should carry the tabler- prefix");
        assert_eq!(Tabler::codepoint(stripped), Tabler::codepoint(&name));
    }

    /// `TablerIconName` normalizes every accepted form to the canonical
    /// `tabler-{name}-symbolic` representation.
    #[test]
    fn icon_name_normalizes() {
        for input in ["abacus", "tabler-abacus", "tabler-abacus-symbolic"] {
            let name = TablerIconName::parse(input).unwrap_or_else(|| panic!("{input} should parse"));
            assert_eq!(name.as_ref(), "tabler-abacus-symbolic", "input: {input}");
            assert_eq!(name.css_class_name(), "tabler-abacus", "input: {input}");
        }
    }

    /// `glyph_svg` must return the vector artwork from the GResource -
    /// the chain behind `gtk_icon_theme` and the detail sidebar.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_returns_artwork() {
        let svg = Tabler::glyph_svg(first_icon_name()).expect("glyph SVG should be in the GResource");
        assert!(svg.contains("<svg"), "expected an SVG document, got: {}", &svg[..svg.len().min(200)]);
    }

    /// Keywords and categories come from upstream `icons.json`
    /// (`tags`/`category` fields) keyed by the full glyph name.
    /// Verified against the generated maps' own entries so the test
    /// works under either variant.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_keywords_categories() {
        use fonts_rs_model::GlyphMetadata;
        let keyword_map = metadata::TablerMetadata.keyword_map();
        let (name, expected) = keyword_map.0.entries().next().expect("keyword map should not be empty");
        assert_eq!(metadata::TablerMetadata.keywords(name), *expected);
        let category_map = metadata::TablerMetadata.category_map();
        let (name, expected) = category_map.0.entries().next().expect("category map should not be empty");
        assert_eq!(metadata::TablerMetadata.categories(name), *expected);
    }

    /// Every alias in the active style's table must resolve to its
    /// canonical glyph name - spot-checked on the first entries to
    /// stay variant-agnostic.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_aliases_resolve() {
        use fonts_rs_model::GlyphMetadata;
        let map = metadata::TablerMetadata.alias_map();
        assert!(!map.0.is_empty(), "alias map should not be empty");
        for (alias, canonical) in map.0.entries().take(5) {
            assert_eq!(metadata::TablerMetadata.resolve_alias(alias), Some(*canonical), "alias {alias} should resolve");
        }
    }

    /// Metadata search must find icons by keyword and category.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_search_finds_icons() {
        use fonts_rs_model::GlyphMetadata;
        assert!(!metadata::TablerMetadata.search("arrow").is_empty(), "search for 'arrow' should find icons");
        assert!(metadata::TablerMetadata.search("").is_empty(), "empty query should return no results");
    }
}
