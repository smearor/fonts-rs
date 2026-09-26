//! Nerd Fonts icon name type alias.
//!
//! [`IconName`] is the generic [`fonts_rs_model::IconName`] instantiated
//! with [`NerdFontsConfig`]: normalization (lowercase, kebab-case,
//! `-symbolic` suffix) lives in `fonts-rs-model`; the `nf-` prefix and
//! [`IconSet`]-based validation come from the config.

use crate::config::NerdFontsConfig;

/// A normalized Nerd Fonts GTK icon name (`nf-{set}-{name}-symbolic`).
///
/// Alias for `fonts_rs_model::IconName<NerdFontsConfig>` - see the
/// generic type for the normalization pipeline and method surface.
/// `icon_set()` is available because [`NerdFontsConfig`] implements
/// [`IconSetDetect`](fonts_rs_model::IconSetDetect).
///
/// # Examples
///
/// ```
/// use nerd_fonts_model::IconName;
///
/// let a = IconName::parse("fa-gamepad").unwrap();
/// let b = IconName::parse("nf-fa-gamepad").unwrap();
/// let c = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
/// assert_eq!(a, b);
/// assert_eq!(b, c);
/// assert_eq!(a.as_ref(), "nf-fa-gamepad-symbolic");
/// ```
pub type IconName = fonts_rs_model::IconName<NerdFontsConfig>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IconSet;

    #[test]
    fn parse_full_name() {
        let name = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
        assert_eq!(name.as_ref(), "nf-fa-gamepad-symbolic");
    }

    #[test]
    fn parse_short_name() {
        let name = IconName::parse("fa-gamepad").unwrap();
        assert_eq!(name.as_ref(), "nf-fa-gamepad-symbolic");
    }

    #[test]
    fn parse_with_nf_prefix_no_symbolic() {
        let name = IconName::parse("nf-fa-gamepad").unwrap();
        assert_eq!(name.as_ref(), "nf-fa-gamepad-symbolic");
    }

    #[test]
    fn parse_all_forms_equivalent() {
        let a = IconName::parse("fa-gamepad").unwrap();
        let b = IconName::parse("nf-fa-gamepad").unwrap();
        let c = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn parse_unknown_prefix_returns_none() {
        assert!(IconName::parse("nf-xx-nonexistent").is_none());
    }

    #[test]
    fn parse_empty_returns_none() {
        assert!(IconName::parse("").is_none());
    }

    #[test]
    fn css_class_name_strips_symbolic() {
        let name = IconName::parse("nf-fa-gamepad-symbolic").unwrap();
        assert_eq!(name.css_class_name(), "nf-fa-gamepad");
    }

    #[test]
    fn icon_set_detects_font_awesome() {
        let name = IconName::from_glyph_name("fa-gamepad").unwrap();
        assert_eq!(name.icon_set(), IconSet::FontAwesome);
    }

    #[test]
    fn icon_name_resource_path() {
        let name = IconName::parse("nf-fa-gamepad").unwrap();
        let path = crate::ResourcePath::from(&name);
        assert_eq!(path.as_ref(), "/io/smearor/fonts/nerd_fonts/scalable/glyphs/nf-fa-gamepad-symbolic");
    }
}
