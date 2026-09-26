//! CSS helper class generation for icon fonts.
//!
//! Provides [`icon_css`] for generating GTK CSS classes that map a
//! class name to a `font-family`. The font itself must be loaded into
//! the Pango font map separately - GTK4's CSS parser does not reliably
//! load fonts from `@font-face` rules.

/// Generates GTK CSS helper classes for icon fonts.
///
/// Each `(class, font_family)` pair emits a rule
/// `.{class} { font-family: '{font_family}'; font-size: 1.5em; }`.
///
/// The referenced font families must be loaded via the family's
/// [`PangoFont::install_pango`](crate::PangoFont::install_pango) before
/// the CSS takes effect.
///
/// # Examples
///
/// ```
/// use fonts_rs_model::icon_css;
///
/// let css = icon_css(&[("tabler-icon", "tabler-icons")]);
/// assert!(css.contains(".tabler-icon"));
/// assert!(css.contains("font-family: 'tabler-icons'"));
/// ```
pub fn icon_css(classes: &[(&str, &str)]) -> String {
    let mut css = String::with_capacity(128 * classes.len());
    for (class, font_family) in classes {
        css.push_str(&format!(".{class} {{\n    font-family: '{font_family}';\n    font-size: 1.5em;\n}}\n"));
    }
    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_css_contains_class_and_family() {
        let css = icon_css(&[("nerd-icon", "NerdFontsSymbolsOnly"), ("nerd-icon-mono", "NerdFontsSymbolsOnlyMono")]);
        assert!(css.contains(".nerd-icon {"));
        assert!(css.contains(".nerd-icon-mono {"));
        assert!(css.contains("font-family: 'NerdFontsSymbolsOnly'"));
        assert!(css.contains("font-family: 'NerdFontsSymbolsOnlyMono'"));
        assert!(css.contains("font-size: 1.5em"));
    }

    #[test]
    fn icon_css_empty_produces_empty() {
        assert!(icon_css(&[]).is_empty());
    }

    #[test]
    fn icon_css_has_no_font_face_rules() {
        let css = icon_css(&[("x", "Y")]);
        assert_eq!(css.matches("@font-face").count(), 0);
    }
}
