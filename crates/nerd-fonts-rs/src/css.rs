//! CSS string constants for GTK and web usage.
//!
//! The [`icon_css`] function generates CSS with `.nerd-icon` helper classes.
//! Font loading is handled separately by [`crate::load_fonts`] via Pango's
//! `add_font_file` API.

/// Generates CSS with `.nerd-icon` and `.nerd-icon-mono` helper classes.
///
/// These classes reference the `NerdFontsSymbolsOnly` and
/// `NerdFontsSymbolsOnlyMono` font families, which must be loaded via
/// [`crate::load_fonts`] before the CSS takes effect.
///
/// The `@font-face` rules are no longer included in the CSS because the
/// GTK4 CSS parser does not reliably load fonts from `@font-face` rules.
/// Instead, fonts are loaded directly into the Pango font map.
pub fn icon_css() -> String {
    let mut css = String::with_capacity(256);

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
    fn icon_css_contains_font_family() {
        let css = icon_css();
        assert!(css.contains("NerdFontsSymbolsOnly"));
        assert!(css.contains("NerdFontsSymbolsOnlyMono"));
    }

    #[test]
    fn icon_css_contains_nerd_icon_class() {
        let css = icon_css();
        assert!(css.contains(".nerd-icon"));
        assert!(css.contains(".nerd-icon-mono"));
    }

    #[test]
    fn icon_css_contains_font_size() {
        let css = icon_css();
        assert!(css.contains("font-size: 1.5em"));
    }

    #[test]
    fn icon_css_font_families_are_distinct() {
        let css = icon_css();
        assert!(css.contains("'NerdFontsSymbolsOnly'"));
        assert!(css.contains("'NerdFontsSymbolsOnlyMono'"));
        assert_ne!("NerdFontsSymbolsOnly", "NerdFontsSymbolsOnlyMono");
    }

    #[test]
    fn icon_css_has_no_font_face_rules() {
        let css = icon_css();
        let count = css.matches("@font-face").count();
        assert_eq!(count, 0, "icon_css should not contain @font-face rules");
    }

    #[test]
    fn icon_css_has_no_resource_urls() {
        let css = icon_css();
        assert!(!css.contains("resource://"));
        assert!(!css.contains("http"));
        assert!(!css.contains("file://"));
    }
}
