//! Fluent UI System Icons integration for GTK 4.
//!
//! Hybrid family: pre-rendered upstream SVGs ship in the icon GResource
//! (symbolic-recolorable via `currentColor`) and the matching
//! `FluentSystemIcons-{Regular,Filled}.ttf` webfont ships for `pango`,
//! `html` (@font-face) and `render` (pixel-drawing). Codepoints are the
//! real upstream codepoints from the vendored `FluentSystemIcons-*.json`
//! maps - including the ~26% allocated in Supplementary PUA-A
//! (U+F0000–U+F0D9F).
//!
//! Two variants are available via Cargo features: `regular` (default)
//! and `filled`. Icon names are `fluent-{name}-symbolic` for the
//! regular variant and `fluent-filled-{name}-symbolic` for `filled`.
//!
//! Deprecated upstream icons (`hidden.json`: `flag-pride`,
//! `text-color-accent`) are excluded from the export. The `metadata`
//! feature exposes [`metadata::FluentMetadata`] with filename-derived
//! keywords and the iconify alias table.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_fluent::Fluent;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any icons
//! if let Err(e) = Fluent::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve icon names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_fluent::Fluent;
//!
//! let codepoint = Fluent::codepoint("fluent-add-symbolic");
//! ```

pub mod codepoint_map;
pub mod constants;
pub mod definition;
mod family;
pub mod filename_parser;
pub mod naming;
pub mod variant;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "render")]
pub mod fonts;

#[cfg(feature = "metadata")]
pub mod metadata;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::FluentConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
pub use naming::Fluent;
pub use naming::FluentIconName;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`FluentIconName`] from the [`naming`] module - this alias
/// is provided for consistency with the template structure.
pub type FamilyName = FluentIconName;

/// Returns all known Fluent icons as `(CodePoint, icon_name)` pairs,
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

    #[test]
    fn all_icons_is_not_empty() {
        let count = Fluent::glyph_count();
        assert!(count > 0, "Fluent should export at least one glyph");
    }

    /// The export must cover the vendored 24px slice minus the
    /// deprecated `hidden.json` icons - regular: 2,474 glyphs (both
    /// hidden icons are filled-only files), filled: 2,508 (2,510 - 2).
    #[test]
    fn glyph_count_matches_vendored_slice() {
        let expected = if cfg!(feature = "filled") { 2508 } else { 2474 };
        assert_eq!(Fluent::glyph_count(), expected);
    }

    /// Deprecated upstream icons (`hidden.json`: `flag-pride`,
    /// `text-color-accent`) are excluded from the export. Both are
    /// filled-only files, so this is the discriminating assertion for
    /// the `filled` variant.
    #[test]
    fn hidden_icons_are_excluded() {
        for hidden in ["flag-pride", "text-color-accent"] {
            let name = format!("{}-{hidden}-symbolic", variant::GLYPH_PREFIX);
            assert!(Fluent::codepoint(&name).is_none(), "hidden icon '{name}' should not be exported");
        }
    }

    /// All exported names carry the variant prefix (`fluent-` or
    /// `fluent-filled-`).
    #[test]
    fn all_icon_names_start_with_variant_prefix() {
        let prefix = format!("{}-", variant::GLYPH_PREFIX);
        for (_, name) in all_icons() {
            assert!(name.starts_with(&prefix), "icon name '{name}' should start with '{prefix}'");
        }
    }

    /// `codepoint`/`glyph_name` must round-trip for exported icons -
    /// the chain behind icon theme lookup and the cheat sheet.
    #[test]
    fn codepoint_glyph_name_round_trip() {
        let name = first_icon_name();
        let codepoint = Fluent::codepoint(&name).expect("icon codepoint should resolve");
        let resolved = Fluent::glyph_name(codepoint).expect("icon name should resolve");
        assert_eq!(resolved.as_ref(), name);
    }

    /// `fluent-add-symbolic` (`fluent-filled-add-symbolic` for the
    /// `filled` variant) resolves a real upstream codepoint.
    #[test]
    fn fluent_add_symbolic_resolves() {
        let name = format!("{}-add-symbolic", variant::GLYPH_PREFIX);
        assert!(Fluent::codepoint(&name).is_some(), "codepoint('{name}') should resolve");
    }

    /// Every codepoint falls inside a declared `CODEPOINT_RANGES`
    /// interval (BMP PUA or the Supplementary PUA-A tail).
    #[test]
    fn codepoints_within_declared_ranges() {
        for (codepoint, name) in all_icons() {
            let scalar = codepoint.as_char() as u32;
            assert!(
                (0xE000..=0xF8FF).contains(&scalar) || (0xF0000..=0xF0DFF).contains(&scalar),
                "icon '{name}' codepoint U+{scalar:04X} outside declared ranges"
            );
        }
    }

    /// The upstream codepoint map allocates ~26% of glyphs in
    /// Supplementary PUA-A - the `CodePoint` (char/u32) handling must
    /// preserve them, not truncate to the BMP.
    #[test]
    fn supplementary_pua_present() {
        let count = all_icons().iter().filter(|(cp, _)| cp.as_char() as u32 > 0xFFFF).count();
        assert!(count > 0, "expected Supplementary PUA-A codepoints in the exported map");
    }

    /// Keywords are filename-derived: `arrow-down` splits into `arrow`
    /// and `down`. Verified against the generated keyword map's own
    /// entries.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_keywords_filename_derived() {
        use fonts_rs_model::GlyphMetadata;
        let name = format!("{}-arrow-down-symbolic", variant::GLYPH_PREFIX);
        let keywords = metadata::FluentMetadata.keywords(&name);
        assert!(keywords.iter().any(|kw| kw.as_str() == "arrow"), "expected 'arrow' keyword, got: {keywords:?}");
        assert!(keywords.iter().any(|kw| kw.as_str() == "down"), "expected 'down' keyword, got: {keywords:?}");
    }

    /// The `CurrentColor` post-processor must have rewritten the
    /// upstream `fill="#212121"` to `fill="currentColor"` in the
    /// emitted GResource SVGs - verified at runtime on a bundled icon.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_uses_current_color() {
        use crate::FluentIconName;

        Fluent::ensure_registered().expect("gresource registration should succeed");
        let name = format!("{}-checkmark-symbolic", variant::GLYPH_PREFIX);
        let parsed = FluentIconName::parse(&name).expect("icon name should parse");
        let svg = Fluent::glyph_svg(&parsed).expect("glyph_svg should resolve");
        assert!(svg.contains("currentColor"), "{name}: expected fill=\"currentColor\" in GResource SVG");
        assert!(!svg.contains("#212121"), "{name}: upstream hardcoded fill leaked through the post-processor");
    }

    /// End-to-end `CurrentColor` proof: rendering a `-symbolic` icon
    /// through [`gtk4::SymbolicPaintable::snapshot_symbolic`] with a
    /// non-default foreground (magenta) must paint pixels of *that*
    /// color. If `fill="#212121"` had survived, the rendered pixels
    /// would be dark gray regardless of the symbolic color passed.
    #[cfg(feature = "gtk")]
    #[test]
    fn icon_theme_renders_current_color() {
        use gtk4::prelude::*;

        if gtk4::init().is_err() {
            eprintln!("SKIP: no display for GTK rendering test");
            return;
        }
        let display = gtk4::gdk::Display::default().expect("display after gtk4::init");
        Fluent::install_icon_theme(&display).expect("icon theme registration should succeed");
        let theme = gtk4::IconTheme::for_display(&display);
        let name = format!("{}-checkmark-symbolic", variant::GLYPH_PREFIX);
        assert!(theme.has_icon(&name), "{name} should resolve in the icon theme");
        let icon = theme.lookup_icon(&name, &[], 96, 1, gtk4::TextDirection::None, gtk4::IconLookupFlags::empty());
        assert!(icon.is_symbolic(), "{name}: expected symbolic paintable (currentColor follows theme color)");

        let magenta = gtk4::gdk::RGBA::new(1.0, 0.0, 1.0, 1.0);
        let snapshot = gtk4::Snapshot::new();
        icon.snapshot_symbolic(&snapshot, 96.0, 96.0, &[magenta]);
        let node = snapshot.to_node().expect("snapshot should produce a render node");

        let mut surface = gtk4::gdk::cairo::ImageSurface::create(gtk4::gdk::cairo::Format::ARgb32, 96, 96).expect("cairo surface");
        {
            let cr = gtk4::gdk::cairo::Context::new(&surface).expect("cairo context");
            node.draw(&cr);
        }
        surface.flush();
        let (width, height, stride) = (surface.width() as usize, surface.height() as usize, surface.stride() as usize);
        let data = surface.data().expect("surface data");

        // ARgb32 on little-endian = premultiplied B,G,R,A byte order.
        // Magenta: R==B, G==0 - dark #212121 would give R==G==B.
        let (mut opaque, mut magenta_pixels) = (0usize, 0usize);
        for y in 0..height {
            for x in 0..width {
                let offset = y * stride + x * 4;
                let (b, g, r, a) = (data[offset], data[offset + 1], data[offset + 2], data[offset + 3]);
                if a == 0 {
                    continue;
                }
                opaque += 1;
                if g as u16 <= r as u16 / 2 && r as u16 + 8 >= b as u16 {
                    magenta_pixels += 1;
                }
            }
        }
        assert!(opaque > 0, "{name}: icon rendered fully transparent");
        let magenta_ratio = magenta_pixels as f64 / opaque as f64;
        assert!(
            magenta_ratio > 0.5,
            "{name}: rendered pixels should follow the magenta symbolic color, got {magenta_ratio:.2} magenta ratio \
             (opaque={opaque}) - CurrentColor rewrite likely missing"
        );
    }

    /// Iconify renames resolve to canonical exported names:
    /// `arrow-down-square` -> `arrow-square-down`.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_resolves_alias() {
        use fonts_rs_model::GlyphMetadata;
        let prefix = variant::GLYPH_PREFIX;
        let alias = format!("{prefix}-arrow-down-square-symbolic");
        let canonical = format!("{prefix}-arrow-square-down-symbolic");
        let resolved = metadata::FluentMetadata.resolve_alias(&alias).expect("alias should resolve");
        assert_eq!(resolved, canonical.as_str());
    }

    /// Search finds icons via the alias table: `magic-wand` is an
    /// iconify alias for `wand`.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_search_finds_via_alias() {
        use fonts_rs_model::GlyphMetadata;
        let results = metadata::FluentMetadata.search("magic-wand");
        let wand = format!("{}-wand-symbolic", variant::GLYPH_PREFIX);
        assert!(results.contains(&wand.as_str()), "search for 'magic-wand' should find '{wand}', got: {results:?}");
    }
}
