//! Web CSS rule generation for icon fonts.
//!
//! Provides [`web_icon_css_rules`], the per-icon `content: "\XXXX"`
//! mapping shared by all icon font web CSS generators. Family-specific
//! generators add their own `@font-face` header and base class around
//! these rules.

use fonts_rs_model::GlyphEntry;

/// Generates per-icon CSS rules mapping `.{class_prefix}-{name}:before`
/// to the glyph's Unicode codepoint.
///
/// Icons without a codepoint are skipped. Rules are sorted by icon name
/// for deterministic output.
///
/// # Examples
///
/// ```
/// use fonts_rs_generator::generator::web_css::web_icon_css_rules;
/// use fonts_rs_model::CodePoint;
/// use fonts_rs_model::GlyphEntry;
///
/// let icons = vec![GlyphEntry::new(Some(CodePoint::from_char('\u{F11B}')), "nf-fa-gamepad-symbolic", "/io/smearor/fonts/nerd_fonts", "glyphs")];
/// let rules = web_icon_css_rules(&icons, "nerd");
/// assert!(rules.contains(".nerd-nf-fa-gamepad-symbolic:before"));
/// assert!(rules.contains("content: \"\\f11b\""));
/// ```
pub fn web_icon_css_rules<N: AsRef<str> + Ord>(icons: &[GlyphEntry<N>], class_prefix: &str) -> String {
    let mut sorted: Vec<&GlyphEntry<N>> = icons.iter().filter(|i| i.code.is_some()).collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    let mut output = String::new();
    for icon in sorted {
        let Some(code) = &icon.code else { continue };
        output.push_str(&format!(
            ".{class_prefix}-{}:before {{\n    content: \"\\{:x}\"\n}}\n\n",
            icon.name.as_ref(),
            code.as_char() as u32
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use fonts_rs_model::CodePoint;

    #[test]
    fn rules_skip_icons_without_codepoint() {
        let icons = vec![
            GlyphEntry::new(Some(CodePoint::from_char('\u{F11B}')), "nf-fa-gamepad-symbolic", "/p", "glyphs"),
            GlyphEntry::new(None, "nf-fa-unmapped-symbolic", "/p", "glyphs"),
        ];
        let rules = web_icon_css_rules(&icons, "nerd");
        assert!(rules.contains("nf-fa-gamepad-symbolic"));
        assert!(!rules.contains("nf-fa-unmapped-symbolic"));
    }

    #[test]
    fn rules_are_sorted_by_name() {
        let icons = vec![
            GlyphEntry::new(Some(CodePoint::from_char('b')), "b-icon", "/p", "glyphs"),
            GlyphEntry::new(Some(CodePoint::from_char('a')), "a-icon", "/p", "glyphs"),
        ];
        let rules = web_icon_css_rules(&icons, "x");
        assert!(rules.find("a-icon").unwrap() < rules.find("b-icon").unwrap());
    }
}
