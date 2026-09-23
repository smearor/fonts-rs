//! Vendored Nerd Font icons: GResource registration, codepoint maps, and constants.
//!
//! This module replaces the external `nerd_gtk_icons` crate. It provides:
//! - `register_icons()`: registers the compiled GResource bundle
//! - `codepoint_map::ICONS`: `phf::Map<char, &'static str>` (codepoint -> name)
//! - `codepoint_map::REVERSE_ICONS`: `phf::Map<&'static str, char>` (name -> codepoint)
//! - `constants`: auto-generated icon name constants

use nerd_fonts_model::CodePoint;
use nerd_fonts_model::IconName;

/// Auto-generated Nerd Font icon name constants.
pub mod constants;

/// Mapping between Unicode codepoints and icon names.
pub mod codepoint_map;

/// Registers embedded Nerd Font icons as a GResource.
///
/// Must be called once before using any icons through GTK/GIO APIs.
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
#[cfg(feature = "gtk")]
pub fn register_icons() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")
}

/// Returns an iterator over all known Nerd Font icons, yielding
/// `(char, &'static str)` pairs of codepoint and icon name.
///
/// # Examples
///
/// ```
/// use nerd_fonts_rs::all_icons;
///
/// let count = all_icons().count();
/// assert!(count > 1000, "Nerd Fonts contain thousands of icons");
///
/// let gamepad = all_icons().find(|(_, name)| *name == "nf-fa-gamepad-symbolic");
/// assert!(gamepad.is_some());
/// ```
pub fn all_icons() -> impl Iterator<Item = (char, &'static str)> {
    codepoint_map::ICONS.entries().map(|(c, name)| (*c, *name))
}

/// Returns all known Nerd Font icons as `(CodePoint, IconName)` pairs,
/// sorted alphabetically by icon name.
///
/// # Examples
///
/// ```
/// use nerd_fonts_rs::all_icons_typed;
///
/// let icons = all_icons_typed();
/// assert!(icons.len() > 1000, "Nerd Fonts contain thousands of icons");
///
/// let gamepad = icons.iter().find(|(_, name)| name.as_ref() == "nf-fa-gamepad-symbolic");
/// assert!(gamepad.is_some());
/// ```
pub fn all_icons_typed() -> Vec<(CodePoint, IconName)> {
    let mut icons: Vec<(CodePoint, IconName)> = codepoint_map::ICONS
        .entries()
        .filter_map(|(c, name)| {
            let cp = CodePoint::from(*c);
            let icon_name = IconName::parse(name)?;
            Some((cp, icon_name))
        })
        .collect();
    icons.sort_by(|a, b| a.1.cmp(&b.1));
    icons
}

#[cfg(test)]
mod tests {
    use super::*;
    use fonts_rs_model::CodePoint;

    #[test]
    fn icon_name_codepoint_known_icon() {
        let codepoint = crate::NerdFonts::codepoint("fa-gamepad");
        assert_eq!(codepoint, Some(CodePoint::from_char('\u{F11B}')));
    }

    #[test]
    fn icon_name_codepoint_with_underscores() {
        let codepoint = crate::NerdFonts::codepoint("linux_tux");
        assert_eq!(codepoint, Some(CodePoint::from_char('\u{F31A}')));
    }

    #[test]
    fn icon_name_codepoint_unknown_icon() {
        assert_eq!(crate::NerdFonts::codepoint("fa-nonexistent-xyz"), None);
    }

    #[test]
    fn all_icons_returns_thousands_of_icons() {
        let count = all_icons().count();
        assert!(count > 1000, "Nerd Fonts should contain thousands of icons, got {}", count);
    }

    #[test]
    fn all_icons_contains_known_icon() {
        let gamepad = all_icons().find(|(_, name)| *name == "nf-fa-gamepad-symbolic");
        assert!(gamepad.is_some(), "nf-fa-gamepad-symbolic should be in all_icons");
        assert_eq!(gamepad.unwrap().0, '\u{F11B}');
    }

    #[test]
    fn all_icons_yields_unique_codepoints() {
        let codepoints: Vec<char> = all_icons().map(|(c, _)| c).collect();
        let unique: std::collections::HashSet<char> = codepoints.iter().copied().collect();
        assert_eq!(codepoints.len(), unique.len(), "all codepoints should be unique");
    }

    #[test]
    fn all_icons_names_are_well_formed() {
        for (_, name) in all_icons() {
            assert!(name.starts_with("nf-"), "icon name '{}' should start with 'nf-'", name);
            assert!(name.ends_with("-symbolic"), "icon name '{}' should end with '-symbolic'", name);
            assert!(!name.contains('_'), "icon name '{}' should not contain underscores", name);
            assert_eq!(*name, name.to_lowercase(), "icon name '{}' should be lowercase", name);
        }
    }
}
