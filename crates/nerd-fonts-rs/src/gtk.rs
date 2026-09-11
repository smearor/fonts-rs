//! GTK4 icon name resolution.
//!
//! Provides functions to resolve Nerd Font icon names into GTK icon names.

use tracing::trace;

use nerd_fonts_model::IconName;

use crate::icons::IconNameExt;

/// Resolves a CSS class name (e.g. `"nf-fa-gamepad"` or `"fa-gamepad"`)
/// into an icon name string that `gtk4::Image::from_icon_name` understands.
///
/// The vendored icon GResource registers SVG icons under
/// the path `/io/smearor/fonts/nerd_fonts/icons/`. Each icon is named following
/// the pattern `nf-{prefix}-{name}-symbolic` (kebab-case, lower-case).
///
/// Returns `None` if the resolved icon name does not exist in the
/// codepoint map.
pub fn resolve_gtk_nerd_icon(css_class: &str) -> Option<String> {
    let clean_name = css_class.strip_prefix("nf-").unwrap_or(css_class);
    let normalized = clean_name.replace('-', "_").to_uppercase();

    let mut icon_name = if normalized.starts_with("NF_") {
        normalized
    } else {
        format!("NF_{}", normalized)
    };

    if !icon_name.ends_with("_SYMBOLIC") {
        icon_name.push_str("_SYMBOLIC");
    }

    let gtk_friendly_name = icon_name.to_lowercase().replace('_', "-");

    trace!("resolve_gtk_nerd_icon: input='{}' -> output='{}'", css_class, gtk_friendly_name);

    // Validate that the icon actually exists in the codepoint map.
    // Strip the `nf-` prefix and `-symbolic` suffix since `from_glyph_name`
    // re-adds them during normalization.
    let glyph_name = gtk_friendly_name
        .strip_prefix("nf-")
        .unwrap_or(&gtk_friendly_name)
        .strip_suffix("-symbolic")
        .unwrap_or(&gtk_friendly_name);
    IconName::from_glyph_name(glyph_name)?.codepoint()?;

    Some(gtk_friendly_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_icon_with_nf_prefix() {
        let result = resolve_gtk_nerd_icon("nf-fa-gamepad");
        assert_eq!(result, Some("nf-fa-gamepad-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_without_nf_prefix() {
        let result = resolve_gtk_nerd_icon("fa-gamepad");
        assert_eq!(result, Some("nf-fa-gamepad-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_already_has_symbolic_suffix() {
        let result = resolve_gtk_nerd_icon("nf-fa-gamepad-symbolic");
        assert_eq!(result, Some("nf-fa-gamepad-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_with_underscores() {
        let result = resolve_gtk_nerd_icon("nf_weather_day_sunny");
        assert_eq!(result, Some("nf-weather-day-sunny-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_uppercase_input() {
        let result = resolve_gtk_nerd_icon("NF-FA-GAMEPAD");
        assert_eq!(result, Some("nf-fa-gamepad-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_single_segment() {
        let result = resolve_gtk_nerd_icon("nf-linux-tux");
        assert_eq!(result, Some("nf-linux-tux-symbolic".to_string()));
    }

    #[test]
    fn resolve_unknown_icon_returns_none() {
        let result = resolve_gtk_nerd_icon("nf-nonexistent-icon-xyz");
        assert_eq!(result, None);
    }

    #[test]
    fn resolve_icon_preserves_multi_segment_names() {
        let result = resolve_gtk_nerd_icon("nf-md-alert_circle_outline");
        assert_eq!(result, Some("nf-md-alert-circle-outline-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_empty_string_returns_none() {
        let result = resolve_gtk_nerd_icon("");
        assert_eq!(result, None);
    }

    #[test]
    fn resolve_icon_double_nf_prefix() {
        let result = resolve_gtk_nerd_icon("nf-nf-fa-gamepad");
        assert_eq!(result, Some("nf-fa-gamepad-symbolic".to_string()));
    }

    #[test]
    fn resolve_icon_css_class_format() {
        let result = resolve_gtk_nerd_icon("nf-fa-star");
        assert!(result.is_some());
        let name = result.unwrap();
        assert!(name.starts_with("nf-"), "CSS class should start with 'nf-'");
        assert!(name.ends_with("-symbolic"), "CSS class should end with '-symbolic'");
        assert!(!name.contains('_'), "CSS class should not contain underscores");
        assert_eq!(name, name.to_lowercase(), "CSS class should be lowercase");
    }
}
