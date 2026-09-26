//! Lucide Icons integration for GTK 4.
//!
//! Provides icon name resolution, GResource registration, and codepoint
//! lookup for the Lucide Icons font family — ~1,850 stroke-based
//! icons on a 24×24 grid.
//!
//! Lucide is a **fontless** icon set: upstream ships only pre-rendered
//! SVGs (`stroke="currentColor"`), no webfont. Glyphs are resolved via
//! `GtkIconTheme` from the GResource bundle; codepoints are stable
//! hash-of-name PUA allocations. There is no `pango`, `render`, or
//! `embed-fonts` capability — and no `html` (FontFace); typed
//! inline-SVG HTML export works via [`HtmlSvgExport`].
//!
//! Keyword and category metadata (`metadata` feature) is sourced from
//! upstream `icons/{name}.json` `tags`/`categories` fields.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_lucide::Lucide;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any icons
//! if let Err(e) = Lucide::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve icon names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_lucide::Lucide;
//!
//! let codepoint = Lucide::codepoint("lucide-a-arrow-down-symbolic");
//! ```
//!
//! [`HtmlSvgExport`]: fonts_rs_model::HtmlSvgExport

pub mod codepoint_map;
pub mod constants;
pub mod definition;
mod family;
pub mod naming;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "metadata")]
pub mod metadata;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::LucideConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Lucide;
pub use naming::LucideIconName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`LucideIconName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = LucideIconName;

/// Returns all known Lucide icons as `(CodePoint, icon_name)` pairs,
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

    /// Returns the first icon name in the glyph map.
    fn first_icon_name() -> String {
        all_icons().into_iter().next().map(|(_, name)| name).expect("at least one icon should exist")
    }

    /// `codepoint`/`glyph_name` must round-trip for exported icons —
    /// the chain behind icon theme lookup and the cheat sheet.
    /// HashedPua codepoints are stable, but the test still derives
    /// them from the name to stay implementation-agnostic.
    #[test]
    fn codepoint_glyph_name_round_trip() {
        let name = first_icon_name();
        let codepoint = Lucide::codepoint(&name).expect("icon codepoint should resolve");
        let resolved = Lucide::glyph_name(codepoint).expect("icon name should resolve");
        assert_eq!(resolved.as_ref(), name);
    }

    /// All codepoints are hashed PUA allocations (U+E000–U+F8FF) —
    /// Lucide has no upstream codepoint table.
    #[test]
    fn codepoints_are_pua() {
        for (codepoint, name) in all_icons() {
            assert!(('\u{E000}'..='\u{F8FF}').contains(&codepoint.as_char()), "icon '{name}' codepoint {codepoint:?} outside PUA");
        }
    }

    /// The `glyph_prefix` fallback must resolve suffix-only lookups:
    /// `codepoint("{name}-symbolic")` expands to the full
    /// `lucide-{name}-symbolic` name.
    #[test]
    fn codepoint_resolves_suffixed_name() {
        let name = first_icon_name();
        let stripped = name.strip_prefix("lucide-").expect("icon name should carry the lucide- prefix");
        assert_eq!(Lucide::codepoint(stripped), Lucide::codepoint(&name));
    }

    /// `LucideIconName` normalizes every accepted form to the canonical
    /// `lucide-{name}-symbolic` representation.
    #[test]
    fn icon_name_normalizes() {
        for input in ["accessibility", "lucide-accessibility", "lucide-accessibility-symbolic"] {
            let name = LucideIconName::parse(input).unwrap_or_else(|| panic!("{input} should parse"));
            assert_eq!(name.as_ref(), "lucide-accessibility-symbolic", "input: {input}");
            assert_eq!(name.css_class_name(), "lucide-accessibility", "input: {input}");
        }
    }

    /// `glyph_svg` must return the vector artwork from the GResource —
    /// the chain behind `gtk_icon_theme` and the detail sidebar.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_returns_artwork() {
        let svg = Lucide::glyph_svg(first_icon_name()).expect("glyph SVG should be in the GResource");
        assert!(svg.contains("<svg"), "expected an SVG document, got: {}", &svg[..svg.len().min(200)]);
        assert!(svg.contains("currentColor"), "Lucide artwork should stay symbolic-recolorable");
    }

    /// Keywords and categories come from upstream `icons.json`
    /// (`tags`/`categories` fields) keyed by the full glyph name.
    /// Verified against the generated maps' own entries.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_keywords_categories() {
        use fonts_rs_model::GlyphMetadata;
        let keyword_map = metadata::LucideMetadata.keyword_map();
        let (name, expected) = keyword_map.0.entries().next().expect("keyword map should not be empty");
        assert_eq!(metadata::LucideMetadata.keywords(name), *expected);
        let category_map = metadata::LucideMetadata.category_map();
        let (name, expected) = category_map.0.entries().next().expect("category map should not be empty");
        assert_eq!(metadata::LucideMetadata.categories(name), *expected);
    }

    /// Metadata search must find icons by keyword — e.g. upstream tags
    /// map "font size" onto `a-arrow-down`/`a-arrow-up`.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_search_finds_icons() {
        use fonts_rs_model::GlyphMetadata;
        let results = metadata::LucideMetadata.search("font size");
        assert!(
            results.contains(&"lucide-a-arrow-down-symbolic"),
            "search for 'font size' should find a-arrow-down, got: {results:?}"
        );
        assert!(metadata::LucideMetadata.search("").is_empty(), "empty query should return no results");
    }
}
