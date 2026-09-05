//! Nerd Font icon name to Unicode codepoint resolution.
//!
//! Maps human-readable icon names like `nf-fa-gamepad` to their Unicode
//! codepoints using the glyph list compiled into the `nerd_gtk_icons` crate.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

/// Resolve a Nerd Font icon name (e.g. `nf-weather-day_sunny`) to its
/// Unicode codepoint character by looking it up in the `nerd_gtk_icons`
/// codepoint map.
///
/// The name is normalized to the GTK symbolic icon name format
/// (kebab-case, `-symbolic` suffix) before lookup.
pub fn resolve_icon_codepoint(icon_name: &str) -> Option<char> {
    let normalized = icon_name.replace('_', "-").to_lowercase();
    let with_suffix = if normalized.ends_with("-symbolic") {
        normalized
    } else {
        format!("{}-symbolic", normalized)
    };
    nerd_gtk_icons::codepoint_map::ICONS
        .entries()
        .find(|(_, name)| **name == with_suffix)
        .map(|(c, _)| *c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_known_icon() {
        let codepoint = resolve_icon_codepoint("nf-fa-gamepad");
        assert_eq!(codepoint, Some('\u{F11B}'));
    }

    #[test]
    fn resolve_icon_with_underscores() {
        let codepoint = resolve_icon_codepoint("nf_linux_tux");
        assert_eq!(codepoint, Some('\u{F31A}'));
    }

    #[test]
    fn resolve_icon_already_has_symbolic_suffix() {
        let codepoint = resolve_icon_codepoint("nf-fa-gamepad-symbolic");
        assert_eq!(codepoint, Some('\u{F11B}'));
    }

    #[test]
    fn resolve_icon_case_insensitive() {
        let codepoint = resolve_icon_codepoint("NF-FA-GAMEPAD");
        assert_eq!(codepoint, Some('\u{F11B}'));
    }

    #[test]
    fn resolve_unknown_icon_returns_none() {
        let codepoint = resolve_icon_codepoint("nf-nonexistent-icon-xyz");
        assert_eq!(codepoint, None);
    }

    #[test]
    fn resolve_empty_string_returns_none() {
        let codepoint = resolve_icon_codepoint("");
        assert_eq!(codepoint, None);
    }
}
