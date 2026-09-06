//! GTK4 icon name resolution and color application.
//!
//! Provides functions to resolve Nerd Font icon names into GTK icon names
//! and to apply custom colors to `gtk4::Image` and `gtk4::Label` widgets.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

use gtk4::prelude::WidgetExt;
use tracing::trace;

use crate::color::Color;
use crate::icons::IconName;

/// Resolves a CSS class name (e.g. `"nf-fa-gamepad"` or `"fa-gamepad"`)
/// into an icon name string that `gtk4::Image::from_icon_name` understands.
///
/// The vendored icon GResource registers SVG icons under
/// the path `/io/smearor/nerd_fonts/icons/`. Each icon is named following
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

/// Applies a configured icon color to a GTK `Image` via a display-scoped `CssProvider`.
///
/// A unique CSS class is added to the icon widget, and a CSS rule targeting only that class
/// is loaded on the display. This follows the GTK 4.10 recommendation to avoid widget-scoped
/// `StyleContext::add_provider` (deprecated since 4.10).
///
/// On each call, all previously applied `icon-color-*` CSS classes are removed from the
/// icon before the new class is added. This prevents CSS class accumulation across
/// repeated calls (e.g. theme color changes).
pub fn apply_icon_color<C: Into<Color>>(icon: &gtk4::Image, color: C) {
    let color = color.into();

    let existing_classes: Vec<String> = icon
        .css_classes()
        .iter()
        .filter(|c| c.starts_with("icon-color-"))
        .map(|c| c.to_string())
        .collect();

    for class_name in existing_classes {
        icon.remove_css_class(&class_name);
    }

    let class_name = format!(
        "icon-color-{:02x}{:02x}{:02x}{:02x}",
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8,
        (color.a * 255.0).round() as u8
    );
    icon.add_css_class(&class_name);
    let css = format!(
        ".{} {{ color: rgba({}, {}, {}, {}); }}",
        class_name,
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8,
        color.a
    );
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(&css);
    let display = icon.display();
    gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
}

/// Applies a configured text color to a GTK `Label` via a display-scoped `CssProvider`.
///
/// Accepts `Option<Color>` so callers can pass `None` to reset to the default CSS class.
/// On each call, all previously applied `text-color-*` CSS classes are removed from the
/// label before the new class (if any) is added. This prevents CSS class accumulation
/// across repeated `update_ui()` calls (e.g. semantic color changes Normal → Warning → Critical).
/// The dynamic CSS rule includes `opacity: 1;` to neutralise the `opacity: 0.8` from
/// `.widget-info-text`, ensuring the user's configured color is applied exactly as specified.
pub fn apply_text_color<C: Into<Color>>(label: &gtk4::Label, color: Option<C>) {
    let existing_classes: Vec<String> = label
        .css_classes()
        .iter()
        .filter(|c| c.starts_with("text-color-"))
        .map(|c| c.to_string())
        .collect();

    for class_name in existing_classes {
        label.remove_css_class(&class_name);
    }

    if let Some(color) = color.map(Into::into) {
        let class_name = format!(
            "text-color-{:02x}{:02x}{:02x}{:02x}",
            (color.r * 255.0).round() as u8,
            (color.g * 255.0).round() as u8,
            (color.b * 255.0).round() as u8,
            (color.a * 255.0).round() as u8
        );
        label.add_css_class(&class_name);
        let css = format!(
            ".{} {{ color: rgba({}, {}, {}, {}); opacity: 1; }}",
            class_name,
            (color.r * 255.0).round() as u8,
            (color.g * 255.0).round() as u8,
            (color.b * 255.0).round() as u8,
            color.a
        );
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&css);
        gtk4::style_context_add_provider_for_display(&label.display(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }
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
}
