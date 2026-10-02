//! Line Awesome integration for GTK 4.
//!
//! Hybrid family: the 1,544 vendored upstream SVGs ship in the icon
//! GResource (symbolic-recolorable via `currentColor` injected at
//! build time) and the three upstream `la-{regular-400,solid-900,
//! brands-400}.ttf` webfonts ship for `pango`, `html` (@font-face) and
//! `render` (pixel-drawing).
//!
//! Line Awesome follows the Font Awesome 5 model: regular and solid
//! style variants of an icon share one codepoint - the file stem
//! decides the style (`la-{name}-symbolic` for regular/brand icons,
//! `la-{name}-solid-symbolic` for solid). All codepoints are FA5 BMP
//! PUA allocations (`U+F000`–`U+F8FF`).
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[cfg(feature = "gtk")]
//! use fonts_rs_line_awesome::LineAwesome;
//!
//! # #[cfg(feature = "gtk")]
//! // Call once at startup before using any icons
//! if let Err(e) = LineAwesome::install() {
//!     eprintln!("Failed to install font: {e}");
//! }
//! ```
//!
//! Then resolve icon names to Unicode codepoints:
//!
//! ```no_run
//! use fonts_rs_line_awesome::LineAwesome;
//!
//! let codepoint = LineAwesome::codepoint("la-adjust-symbolic");
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
#[cfg(feature = "render")]
pub mod style;

#[cfg(feature = "metadata")]
pub mod metadata;

use fonts_rs_model::CodePoint;

// Re-export key types
pub use definition::GLYPH_PREFIX;
pub use definition::LineAwesomeConfig;
#[cfg(feature = "gtk")]
pub use fonts_rs_model::GtkFont;
pub use fonts_rs_model::prelude;
#[cfg(feature = "metadata")]
pub use metadata::LineAwesomeMetadata;
pub use naming::LineAwesome;
pub use naming::LineAwesomeIconName;
#[cfg(feature = "render")]
pub use style::LineAwesomeStyle;

/// Type alias for this font family's glyph name type.
///
/// Prefer [`LineAwesomeIconName`] from the [`naming`] module - this
/// alias is provided for consistency with the template structure.
pub type FamilyName = LineAwesomeIconName;

/// Returns all known Line Awesome icons as `(CodePoint, icon_name)`
/// pairs, sorted alphabetically by icon name.
///
/// One entry per codepoint: for regular/solid style pairs sharing a
/// codepoint (Font Awesome 5 model) the canonical regular-style name
/// is listed. Style siblings are still resolvable via
/// `LineAwesome::codepoint` and the icon theme.
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
        let count = LineAwesome::glyph_count();
        assert!(count > 0, "LineAwesome should export at least one glyph");
    }

    /// The export must cover the whole vendored flat SVG set:
    /// 1,544 files = 584 regular/brand `{name}.svg` plus 960 solid
    /// `{name}-solid.svg` variants, resolving to 1,393 unique upstream
    /// codepoints (151 regular/solid pairs share one).
    ///
    /// `glyph_count` counts the deduplicated forward map; the reverse
    /// map must still resolve every exported name.
    #[test]
    fn vendored_set_fully_exported() {
        assert_eq!(LineAwesome::glyph_count(), 1393, "unique codepoints");
        assert_eq!(codepoint_map::REVERSE_GLYPHS.len(), 1544, "every exported name must resolve");
    }

    /// All exported names carry the `la-` prefix.
    #[test]
    fn all_icon_names_start_with_la_prefix() {
        for (_, name) in all_icons() {
            assert!(name.starts_with("la-"), "icon name '{name}' should start with 'la-'");
        }
    }

    /// `la-address-book-symbolic` (regular style) resolves a real
    /// upstream codepoint.
    #[test]
    fn la_address_book_symbolic_resolves() {
        assert!(
            LineAwesome::codepoint("la-address-book-symbolic").is_some(),
            "codepoint('la-address-book-symbolic') should resolve"
        );
    }

    /// `la-address-book-solid-symbolic` resolves to the *same*
    /// codepoint as the regular variant - the FA5 model shares
    /// codepoints across style variants per icon name.
    #[test]
    fn la_address_book_solid_symbolic_resolves() {
        let regular = LineAwesome::codepoint("la-address-book-symbolic");
        let solid = LineAwesome::codepoint("la-address-book-solid-symbolic");
        assert_eq!(solid, regular, "regular/solid variants of 'address-book' should share codepoint U+F2B9");
        assert_eq!(regular.map(|cp| cp.as_char() as u32), Some(0xF2B9));
    }

    /// `la-500px-symbolic` (brands face) resolves a real upstream
    /// codepoint.
    #[test]
    fn la_500px_symbolic_resolves() {
        let codepoint = LineAwesome::codepoint("la-500px-symbolic");
        assert_eq!(codepoint.map(|cp| cp.as_char() as u32), Some(0xF26E));
    }

    /// Every codepoint falls inside the declared `CODEPOINT_RANGES`
    /// interval (BMP PUA U+F000-U+F8FF - Line Awesome has no
    /// Supplementary PUA-A allocation).
    #[test]
    fn codepoints_within_declared_ranges() {
        for (codepoint, name) in all_icons() {
            let scalar = codepoint.as_char() as u32;
            assert!((0xF000..=0xF8FF).contains(&scalar), "icon '{name}' codepoint U+{scalar:04X} outside the BMP PUA range");
        }
    }

    /// `codepoint`/`glyph_name` must round-trip: for shared codepoints
    /// the forward map resolves to *a* name of the same codepoint (the
    /// canonical regular-style one), so compare codepoints, not names.
    #[test]
    fn codepoint_glyph_name_round_trip() {
        let name = first_icon_name();
        let codepoint = LineAwesome::codepoint(&name).expect("icon codepoint should resolve");
        let resolved = LineAwesome::glyph_name(codepoint).expect("icon name should resolve");
        assert_eq!(LineAwesome::codepoint(resolved.as_ref()), Some(codepoint));
    }

    /// The `CurrentColor` post-processor must have injected
    /// `fill="currentColor"` into the emitted GResource SVGs - upstream
    /// Line Awesome paths carry no paint attribute at all, so rewriting
    /// alone would leave them permanently black.
    #[cfg(feature = "gtk")]
    #[test]
    fn glyph_svg_uses_current_color() {
        use crate::LineAwesomeIconName;

        LineAwesome::ensure_registered().expect("gresource registration should succeed");
        let parsed = LineAwesomeIconName::parse("la-address-book-symbolic").expect("icon name should parse");
        let svg = LineAwesome::glyph_svg(&parsed).expect("glyph_svg should resolve");
        assert!(svg.contains("fill=\"currentColor\""), "la-address-book-symbolic: expected injected fill=\"currentColor\"");
        assert!(!svg.contains("fill=\"#"), "la-address-book-symbolic: hardcoded fill leaked through the post-processor");
    }

    /// End-to-end icon theme resolution: all three style spellings must
    /// resolve in the GTK icon theme (regular `address-book`, solid
    /// `address-book-solid`, brand `500px`).
    #[cfg(feature = "gtk")]
    #[test]
    fn icon_theme_resolves_all_styles() {
        if gtk4::init().is_err() {
            eprintln!("SKIP: no display for GTK icon theme test");
            return;
        }
        let display = gtk4::gdk::Display::default().expect("display after gtk4::init");
        LineAwesome::install_icon_theme(&display).expect("icon theme registration should succeed");
        let theme = gtk4::IconTheme::for_display(&display);
        for name in ["la-address-book-symbolic", "la-address-book-solid-symbolic", "la-500px-symbolic"] {
            assert!(theme.has_icon(name), "{name} should resolve in the icon theme");
        }
    }

    /// `install_pango` registers all three faces in the font map -
    /// each upstream TTF declares its own family name
    /// (`la-regular-400`, `la-solid-900`, `la-brands-400`).
    #[cfg(feature = "gtk")]
    #[test]
    fn install_pango_registers_all_faces() {
        use fonts_rs_model::pango;
        use fonts_rs_model::pango::prelude::*;

        let font_map: pango::FontMap = pangocairo::FontMap::default().upcast();
        LineAwesome::install_pango(&font_map).expect("install_pango should succeed");
        for family_name in ["la-regular-400", "la-solid-900", "la-brands-400"] {
            assert!(
                font_map.list_families().iter().any(|family| family.name() == family_name),
                "font family '{family_name}' should be registered in the font map"
            );
        }
    }

    /// Software rendering resolves a parsed `ab_glyph` font per style.
    #[cfg(feature = "render")]
    #[test]
    fn ab_glyph_loads_every_style() {
        for style in [LineAwesomeStyle::Regular, LineAwesomeStyle::Solid, LineAwesomeStyle::Brands] {
            assert!(LineAwesome::ab_glyph(style).is_some(), "ab_glyph font for {style:?} should load");
        }
        assert!(LineAwesome::font().is_some(), "default (solid) font should load");
    }

    /// Filename-derived keywords and style categories: solid suffix
    /// marks solid, a plain stem with solid sibling marks regular, a
    /// plain stem without sibling marks brands.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_keywords_categories() {
        use fonts_rs_model::GlyphCategory;
        use fonts_rs_model::GlyphKeyword;
        use fonts_rs_model::GlyphMetadata;
        let metadata = metadata::LineAwesomeMetadata;
        assert!(
            metadata.keywords("la-address-book-solid-symbolic").contains(&GlyphKeyword::new("address")),
            "keywords split the kebab stem"
        );
        assert_eq!(metadata.categories("la-address-book-solid-symbolic"), [GlyphCategory::new("solid")].as_slice());
        assert_eq!(metadata.categories("la-address-book-symbolic"), [GlyphCategory::new("regular")].as_slice());
        assert_eq!(metadata.categories("la-500px-symbolic"), [GlyphCategory::new("brands")].as_slice());
    }

    /// Vendored aliases resolve: FA4-compat `*-o` spellings and `fa-*`
    /// class names map to the exported canonical glyph name.
    #[cfg(feature = "metadata")]
    #[test]
    fn metadata_aliases_resolve() {
        use fonts_rs_model::GlyphMetadata;
        let metadata = metadata::LineAwesomeMetadata;
        assert_eq!(metadata.resolve_alias("la-address-book-o-symbolic"), Some("la-address-book-symbolic"));
        assert_eq!(metadata.resolve_alias("la-fa-adjust-symbolic"), Some("la-adjust-solid-symbolic"));
        assert!(metadata.search("fa-address-book").contains(&"la-address-book-symbolic"), "search via alias");
    }
}
