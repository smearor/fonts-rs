//! Eva Icons font integration for GTK 4 and pixel-drawing.
//!
//! Provides icon name resolution, GResource registration, and codepoint
//! lookup for the Eva Icons font family — ~490 icons in a `fill`
//! (default) and an `outline` variant sharing a single webfont TTF.
//!
//! Keyword metadata (`metadata` feature) is sourced from upstream
//! `icons-tags.json` — the search tags powering the official
//! eva-icons.com cheat sheet.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_eva::Eva;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any icons
//! if let Err(e) = Eva::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve icon names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_eva::Eva;
//!
//! let codepoint = Eva::codepoint("eva-home-symbolic");
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
pub use definition::EvaConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Eva;
pub use naming::EvaIconName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`EvaIconName`] from the [`naming`] module — this alias
/// is provided for consistency with the template structure.
pub type FamilyName = EvaIconName;

/// Returns all known Eva icons as `(CodePoint, icon_name)` pairs,
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
    /// The fill and outline variants ship slightly different icon sets,
    /// so tests must derive names from the map instead of hardcoding
    /// them.
    fn first_icon_name() -> String {
        all_icons().into_iter().next().map(|(_, name)| name).expect("at least one icon should exist")
    }

    /// `codepoint`/`glyph_name` must round-trip for exported icons —
    /// the chain behind icon theme lookup and the cheat sheet.
    /// Codepoints differ between variants, so the test derives them
    /// from the name instead of hardcoding values.
    #[test]
    fn codepoint_glyph_name_round_trip() {
        let name = first_icon_name();
        let codepoint = Eva::codepoint(&name).expect("icon codepoint should resolve");
        let resolved = Eva::glyph_name(codepoint).expect("icon name should resolve");
        assert_eq!(resolved.as_ref(), name);
    }

    /// The `glyph_prefix` fallback must resolve suffix-only lookups:
    /// `codepoint("{name}-symbolic")` expands to the full
    /// `eva-{name}-symbolic` name.
    #[test]
    fn codepoint_resolves_suffixed_name() {
        let name = first_icon_name();
        let stripped = name.strip_prefix("eva-").expect("icon name should carry the eva- prefix");
        assert_eq!(Eva::codepoint(stripped), Eva::codepoint(&name));
    }

    /// `EvaIconName` normalizes every accepted form to the canonical
    /// `eva-{name}-symbolic` representation.
    #[test]
    fn icon_name_normalizes() {
        for input in ["activity", "eva-activity", "eva-activity-symbolic"] {
            let name = EvaIconName::parse(input).unwrap_or_else(|| panic!("{input} should parse"));
            assert_eq!(name.as_ref(), "eva-activity-symbolic", "input: {input}");
            assert_eq!(name.css_class_name(), "eva-activity", "input: {input}");
        }
    }

    /// The outline variant strips the upstream `-outline` suffix, so
    /// names stay variant-independent — no exported name may contain
    /// `-outline-symbolic`.
    #[test]
    fn icon_names_are_variant_independent() {
        for (_, name) in all_icons() {
            assert!(!name.ends_with("-outline-symbolic"), "glyph name '{name}' should be variant-independent");
        }
    }

    /// Metadata search must find icons by keyword — e.g. upstream tags
    /// map "alarm" to `bell`, `bell-off`, and `clock`.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_keywords_and_search() {
        use fonts_rs_model::GlyphMetadata;
        let results = metadata::EvaMetadata.search("alarm");
        assert!(results.contains(&"eva-bell-symbolic"), "search for 'alarm' should find bell, got: {results:?}");
        assert!(results.contains(&"eva-clock-symbolic"), "search for 'alarm' should find clock, got: {results:?}");
        assert!(metadata::EvaMetadata.search("").is_empty(), "empty query should return no results");
    }

    /// `glyph_svg` must return the vector artwork from the GResource —
    /// the chain behind `gtk_icon_theme` and the detail sidebar.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_returns_artwork() {
        let svg = Eva::glyph_svg(first_icon_name()).expect("glyph SVG should be in the GResource");
        assert!(svg.contains("<svg"), "expected an SVG document, got: {}", &svg[..svg.len().min(200)]);
    }
}
