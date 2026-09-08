//! CSS string constants for GTK and web usage.
//!
//! The [`font_face_css`] function generates a version-adapted `@font-face` CSS
//! string at runtime, adjusting the `src` descriptor syntax for the GTK4 CSS
//! parser version in use. The [`FONT_FACE_CSS`] constant retains the static
//! baseline CSS for backward compatibility and testing.

pub mod version;

pub use nerd_fonts_model::GRESOURCE_PREFIX;
pub use version::GtkVersion;

/// GTK CSS: `@font-face` rules for Nerd Font symbol fonts and `.nerd-icon` helper classes.
///
/// This is the static baseline CSS included from `resources/font-face.css`.
/// For version-adapted CSS, use [`font_face_css`] instead.
///
/// Load this into a `CssProvider` at startup (done automatically by `init()` with the `gtk` feature).
pub const FONT_FACE_CSS: &str = include_str!("../resources/font-face.css");

/// GResource path for the proportional Nerd Font symbol font.
const FONT_REGULAR_PATH: &str =
    "resource:///io/smearor/nerd_fonts/SymbolsNerdFont-Regular.ttf";

/// GResource path for the monospace Nerd Font symbol font.
const FONT_MONO_PATH: &str =
    "resource:///io/smearor/nerd_fonts/SymbolsNerdFontMono-Regular.ttf";

/// Generates a version-adapted `@font-face` CSS string for the runtime GTK4
/// CSS parser.
///
/// The GTK4 CSS parser was rewritten in 4.10. The new parser (GtkCssParser)
/// properly supports the `format()` hint in `@font-face` `src` descriptors,
/// while the legacy parser (GTK < 4.10) may reject unknown `format()` values.
///
/// This function detects the runtime GTK4 version and:
///
/// - **GTK >= 4.10**: emits `src: url("...") format("truetype")` for better
///   spec compliance with the new parser.
/// - **GTK < 4.10**: emits `src: url("...")` without a `format()` hint to
///   avoid parse errors with the legacy parser.
///
/// The `.nerd-icon` and `.nerd-icon-mono` helper classes are identical across
/// all versions.
///
/// When the `gtk` feature is disabled, the baseline CSS (without `format()`
/// hints) is returned.
#[cfg(feature = "gtk")]
pub fn font_face_css() -> String {
    build_font_face_css(GtkVersion::runtime())
}

/// Generates the baseline `@font-face` CSS without version detection.
///
/// Equivalent to [`font_face_css`] when running against GTK < 4.10.
#[cfg(not(feature = "gtk"))]
pub fn font_face_css() -> String {
    build_font_face_css(GtkVersion::new(4, 0, 0))
}

/// Builds the `@font-face` CSS string for a given GTK4 version.
fn build_font_face_css(version: GtkVersion) -> String {
    let format_hint = if version.at_least(4, 10) {
        " format(\"truetype\")"
    } else {
        ""
    };

    let mut css = String::with_capacity(512);

    css.push_str("\n@font-face {\n");
    css.push_str("    font-family: 'NerdFontsSymbolsOnly';\n");
    css.push_str("    src: url(\"");
    css.push_str(FONT_REGULAR_PATH);
    css.push_str("\")");
    css.push_str(format_hint);
    css.push_str(";\n");
    css.push_str("}\n");

    css.push_str("@font-face {\n");
    css.push_str("    font-family: 'NerdFontsSymbolsOnlyMono';\n");
    css.push_str("    src: url(\"");
    css.push_str(FONT_MONO_PATH);
    css.push_str("\")");
    css.push_str(format_hint);
    css.push_str(";\n");
    css.push_str("}\n");

    css.push_str(".nerd-icon {\n");
    css.push_str("    font-family: 'NerdFontsSymbolsOnly';\n");
    css.push_str("    font-size: 1.5em;\n");
    css.push_str("}\n");
    css.push_str(".nerd-icon-mono {\n");
    css.push_str("    font-family: 'NerdFontsSymbolsOnlyMono';\n");
    css.push_str("    font-size: 1.5em;\n");
    css.push_str("}\n");

    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gresource_prefix_is_correct() {
        assert_eq!(GRESOURCE_PREFIX, "/io/smearor/nerd_fonts");
    }

    #[test]
    fn font_face_css_contains_font_family() {
        assert!(FONT_FACE_CSS.contains("NerdFontsSymbolsOnly"));
        assert!(FONT_FACE_CSS.contains("NerdFontsSymbolsOnlyMono"));
    }

    #[test]
    fn font_face_css_contains_gresource_url() {
        assert!(FONT_FACE_CSS.contains("resource:///io/smearor/nerd_fonts/SymbolsNerdFont-Regular.ttf"));
        assert!(FONT_FACE_CSS.contains("resource:///io/smearor/nerd_fonts/SymbolsNerdFontMono-Regular.ttf"));
    }

    #[test]
    fn font_face_css_contains_nerd_icon_class() {
        assert!(FONT_FACE_CSS.contains(".nerd-icon"));
        assert!(FONT_FACE_CSS.contains(".nerd-icon-mono"));
    }

    #[test]
    fn font_face_css_contains_font_size() {
        assert!(FONT_FACE_CSS.contains("font-size: 1.5em"));
    }

    #[test]
    fn font_face_css_contains_two_font_faces() {
        let count = FONT_FACE_CSS.matches("@font-face").count();
        assert_eq!(count, 2, "should have exactly two @font-face rules");
    }

    #[test]
    fn font_face_css_font_families_are_distinct() {
        assert!(FONT_FACE_CSS.contains("'NerdFontsSymbolsOnly'"));
        assert!(FONT_FACE_CSS.contains("'NerdFontsSymbolsOnlyMono'"));
        assert_ne!("NerdFontsSymbolsOnly", "NerdFontsSymbolsOnlyMono");
    }

    #[test]
    fn font_face_css_uses_gresource_urls() {
        assert!(FONT_FACE_CSS.contains("resource:///"));
        assert!(!FONT_FACE_CSS.contains("http"));
        assert!(!FONT_FACE_CSS.contains("file://"));
    }

    // --- Version-adapted CSS tests ---

    #[test]
    fn build_font_face_css_legacy_has_no_format_hint() {
        let css = build_font_face_css(GtkVersion::new(4, 0, 0));
        assert!(!css.contains("format("), "legacy CSS should not contain format() hint");
        assert!(css.contains("NerdFontsSymbolsOnly"));
        assert!(css.contains("NerdFontsSymbolsOnlyMono"));
    }

    #[test]
    fn build_font_face_css_modern_has_format_hint() {
        let css = build_font_face_css(GtkVersion::new(4, 10, 0));
        assert!(css.contains("format(\"truetype\")"), "modern CSS should contain format(\"truetype\") hint");
        assert!(css.contains("NerdFontsSymbolsOnly"));
        assert!(css.contains("NerdFontsSymbolsOnlyMono"));
    }

    #[test]
    fn build_font_face_css_4_8_is_legacy() {
        let css = build_font_face_css(GtkVersion::new(4, 8, 5));
        assert!(!css.contains("format("));
    }

    #[test]
    fn build_font_face_css_4_10_is_modern() {
        let css = build_font_face_css(GtkVersion::new(4, 10, 0));
        assert!(css.contains("format(\"truetype\")"));
    }

    #[test]
    fn build_font_face_css_4_14_is_modern() {
        let css = build_font_face_css(GtkVersion::new(4, 14, 2));
        assert!(css.contains("format(\"truetype\")"));
    }

    #[test]
    fn build_font_face_css_has_two_font_faces() {
        let css = build_font_face_css(GtkVersion::new(4, 14, 0));
        let count = css.matches("@font-face").count();
        assert_eq!(count, 2, "should have exactly two @font-face rules");
    }

    #[test]
    fn build_font_face_css_contains_nerd_icon_classes() {
        let css = build_font_face_css(GtkVersion::new(4, 14, 0));
        assert!(css.contains(".nerd-icon"));
        assert!(css.contains(".nerd-icon-mono"));
        assert!(css.contains("font-size: 1.5em"));
    }

    #[test]
    fn build_font_face_css_contains_gresource_urls() {
        let css = build_font_face_css(GtkVersion::new(4, 14, 0));
        assert!(css.contains(FONT_REGULAR_PATH));
        assert!(css.contains(FONT_MONO_PATH));
    }

    #[test]
    fn font_face_css_runtime_matches_expected_structure() {
        let css = font_face_css();
        assert!(css.contains("@font-face"));
        assert!(css.contains("NerdFontsSymbolsOnly"));
        assert!(css.contains(".nerd-icon"));
    }

    #[test]
    fn font_face_css_runtime_has_two_font_faces() {
        let css = font_face_css();
        let count = css.matches("@font-face").count();
        assert_eq!(count, 2, "font_face_css() should produce exactly two @font-face rules");
    }
}
