//! Heroicons integration for GTK 4.
//!
//! Provides icon name resolution, GResource registration, and codepoint
//! lookup for Heroicons - ~320 hand-crafted icons from the makers of
//! Tailwind CSS.
//!
//! Heroicons is a **fontless** icon set: upstream ships only
//! pre-rendered SVGs, no webfont. Glyphs are resolved via
//! `GtkIconTheme` from the GResource bundle; codepoints are stable
//! hash-of-name PUA allocations. There is no `pango`, `render`, or
//! `embed-fonts` capability - and no `html` (FontFace); typed
//! inline-SVG HTML export works via [`HtmlSvgExport`].
//!
//! Four variants are available via Cargo features: `outline` (default,
//! 24x24 stroke-based), `solid` (24x24 filled), `mini` (20x20 filled)
//! and `micro` (16x16 filled). Icon names are variant-independent
//! (`heroicons-{name}-symbolic`) - the variant selects which SVG set
//! ships in the GResource.
//!
//! Keyword metadata (`metadata` feature) is sourced from the
//! heroicons.com search index; aliases map Heroicons v1 names to their
//! v2 canonical names.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_heroicons::Heroicons;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any icons
//! if let Err(e) = Heroicons::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve icon names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_heroicons::Heroicons;
//!
//! let codepoint = Heroicons::codepoint("heroicons-star-symbolic");
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
pub use definition::HeroiconsConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Heroicons;
pub use naming::HeroiconsIconName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`HeroiconsIconName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = HeroiconsIconName;

/// Returns all known Heroicons icons as `(CodePoint, icon_name)` pairs,
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

    /// `codepoint`/`glyph_name` must round-trip for exported icons -
    /// the chain behind icon theme lookup and the cheat sheet.
    /// HashedPua codepoints are stable, but the test still derives
    /// them from the name to stay implementation-agnostic.
    #[test]
    fn codepoint_glyph_name_round_trip() {
        let name = first_icon_name();
        let codepoint = Heroicons::codepoint(&name).expect("icon codepoint should resolve");
        let resolved = Heroicons::glyph_name(codepoint).expect("icon name should resolve");
        assert_eq!(resolved.as_ref(), name);
    }

    /// All codepoints are hashed PUA allocations - outline in the BMP
    /// PUA (U+E000–U+F8FF), the other styles in PUA-B
    /// (U+100000–U+10FFFD). Heroicons has no upstream codepoint table.
    #[test]
    fn codepoints_are_pua() {
        for (codepoint, name) in all_icons() {
            let scalar = codepoint.as_char() as u32;
            assert!(
                (0xE000..=0xF8FF).contains(&scalar) || (0x100000..=0x10FFFD).contains(&scalar),
                "icon '{name}' codepoint U+{scalar:04X} outside PUA"
            );
        }
    }

    /// The `glyph_prefix` fallback must resolve suffix-only lookups:
    /// `codepoint("{name}-symbolic")` expands to the full
    /// `heroicons-{name}-symbolic` name.
    #[test]
    fn codepoint_resolves_suffixed_name() {
        let name = first_icon_name();
        let stripped = name.strip_prefix("heroicons-").expect("icon name should carry the heroicons- prefix");
        assert_eq!(Heroicons::codepoint(stripped), Heroicons::codepoint(&name));
    }

    /// `HeroiconsIconName` normalizes every accepted form to the
    /// canonical `heroicons-{name}-symbolic` representation.
    #[test]
    fn icon_name_normalizes() {
        for input in ["star", "heroicons-star", "heroicons-star-symbolic"] {
            let name = HeroiconsIconName::parse(input).unwrap_or_else(|| panic!("{input} should parse"));
            assert_eq!(name.as_ref(), "heroicons-star-symbolic", "input: {input}");
            assert_eq!(name.css_class_name(), "heroicons-star", "input: {input}");
        }
    }

    /// `glyph_svg` must return the vector artwork from the GResource -
    /// the chain behind `gtk_icon_theme` and the detail sidebar.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_returns_artwork() {
        let svg = Heroicons::glyph_svg(first_icon_name()).expect("glyph SVG should be in the GResource");
        assert!(svg.contains("<svg"), "expected an SVG document, got: {}", &svg[..svg.len().min(200)]);
        assert!(svg.contains("currentColor"), "Heroicons artwork should stay symbolic-recolorable");
    }

    /// Keywords come from the heroicons.com search index (vendored as
    /// `keywords.json`), keyed by the full glyph name. Aliases map
    /// Heroicons v1 names to v2 canonical names. Verified against the
    /// generated maps' own entries.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_keywords_aliases() {
        use fonts_rs_model::GlyphMetadata;
        let keyword_map = metadata::HeroiconsMetadata.keyword_map();
        let (name, expected) = keyword_map.0.entries().next().expect("keyword map should not be empty");
        assert_eq!(metadata::HeroiconsMetadata.keywords(name), *expected);
        let alias_map = metadata::HeroiconsMetadata.alias_map();
        let (alias, expected) = alias_map.0.entries().next().expect("alias map should not be empty");
        assert_eq!(metadata::HeroiconsMetadata.resolve_alias(alias), Some(*expected));
    }

    /// Rendering `heroicons-star-symbolic` through `GtkIconTheme` must
    /// produce a stroke outline, not a filled blob. GTK applies symbolic
    /// recoloring to `-symbolic` icon names; if it overrode `fill="none"`
    /// the star shape would fill solid (high pixel coverage instead of
    /// low). Outline style only - the filled styles are solid by design.
    #[cfg(feature = "gtk")]
    #[test]
    fn icon_theme_renders_outline_not_filled() {
        use gtk4::prelude::*;

        if gtk4::init().is_err() {
            eprintln!("SKIP: no display for GTK rendering test");
            return;
        }
        let display = gtk4::gdk::Display::default().expect("display after gtk4::init");
        Heroicons::install_icon_theme(&display).expect("Heroicons icon theme registration should succeed");
        let theme = gtk4::IconTheme::for_display(&display);
        assert!(theme.has_icon("heroicons-star-symbolic"), "heroicons-star-symbolic should resolve in the icon theme");
        let icon = theme.lookup_icon("heroicons-star-symbolic", &[], 96, 1, gtk4::TextDirection::None, gtk4::IconLookupFlags::empty());

        let snapshot = gtk4::Snapshot::new();
        icon.snapshot(&snapshot, 96.0, 96.0);
        let node = snapshot.to_node().expect("snapshot should produce a render node");

        let mut surface = gtk4::gdk::cairo::ImageSurface::create(gtk4::gdk::cairo::Format::ARgb32, 96, 96).expect("cairo surface");
        {
            let cr = gtk4::gdk::cairo::Context::new(&surface).expect("cairo context");
            node.draw(&cr);
        }
        surface.flush();
        let (width, height, stride) = (surface.width() as usize, surface.height() as usize, surface.stride() as usize);
        let data = surface.data().expect("surface data");
        let opaque = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|&(x, y)| data[y * stride + x * 4 + 3] > 0)
            .count();
        let coverage = opaque as f64 / (width * height) as f64;
        assert!(
            (0.05..0.42).contains(&coverage),
            "star should render as stroke outline (5-42% coverage), got {coverage:.2} - symbolic classes keep fill unpainted"
        );
    }

    /// Metadata search must find icons by keyword - e.g. the
    /// heroicons.com keyword map puts "stars" on `sparkles`.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_search_finds_icons() {
        use fonts_rs_model::GlyphMetadata;
        let results = metadata::HeroiconsMetadata.search("stars");
        assert!(results.contains(&"heroicons-sparkles-symbolic"), "search for 'stars' should find sparkles, got: {results:?}");
        assert!(metadata::HeroiconsMetadata.search("").is_empty(), "empty query should return no results");
    }

    /// Heroicons v1 aliases resolve to their v2 canonical names.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_resolves_v1_alias() {
        use fonts_rs_model::GlyphMetadata;
        assert_eq!(
            metadata::HeroiconsMetadata.resolve_alias("heroicons-exclaimation-circle-symbolic"),
            Some("heroicons-exclamation-circle-symbolic")
        );
    }
}
