//! Nerd Font icon collection enumeration.
//!
//! Provides [`IconSet`] for prefix-based grouping of Nerd Font icons
//! (Font Awesome, Material Design, Octicons, etc.).
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

/// Nerd Font icon collections.
///
/// Each variant corresponds to a prefix convention used in Nerd Font
/// icon names (e.g. `nf-fa-*` for Font Awesome, `nf-md-*` for Material
/// Design). Use [`crate::icons::IconName::icon_set`] to determine the
/// collection from a normalized icon name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconSet {
    /// Material Design icons (`nf-md-*`).
    MaterialDesign,
    /// Font Awesome icons (`nf-fa-*`).
    FontAwesome,
    /// Devicons (`nf-dev-*`).
    Devicons,
    /// Codicons (`nf-cod-*`).
    Codicons,
    /// GitHub Octicons (`nf-oct-*`).
    Octicons,
    /// Weather icons (`nf-weather-*`).
    Weather,
    /// Font Awesome Extended (`nf-fae-*`).
    FontAwesomeExtended,
    /// Seti-UI icons (`nf-seti-*`).
    Seti,
    /// Linux icons (`nf-linux-*`).
    Linux,
    /// Powerline Extra symbols (`nf-ple-*`).
    PowerlineExtra,
    /// Powerline symbols (`nf-pl-*`).
    Powerline,
    /// Custom icons (`nf-custom-*`).
    Custom,
    /// Extra icons (`nf-extra-*`).
    Extra,
    /// Pomicons (`nf-pom-*`).
    Pomicons,
    /// Alpha icons (`nf-alpha-*`).
    Alpha,
    /// IEC symbols (`nf-iec-*`).
    Iec,
    /// Checkbox icons (`nf-checkbox-*`).
    Checkbox,
    /// File icons (`nf-file-*`).
    File,
    /// Heart icons (`nf-heart-*`).
    Heart,
    /// Image icons (`nf-image-*`).
    Image,
    /// Indentation icons (`nf-indentation-*`).
    Indentation,
    /// Music icons (`nf-music-*`).
    Music,
    /// Near icons (`nf-near-*`).
    Near,
    /// Non-marking return symbols (`nf-nonmarkingreturn-*`).
    NonMarkingReturn,
    /// Exit icons (`nf-exit-*`).
    Exit,
    /// Icons that do not match any known prefix.
    Other,
}

impl IconSet {
    /// Detects the icon set from a Nerd Font icon name prefix.
    ///
    /// Performs a simple prefix match — zero allocation, O(1).
    /// The compiler can optimize the match into a jump table.
    ///
    /// This is `pub(crate)` because the public API for obtaining an
    /// `IconSet` is [`crate::icons::IconName::icon_set`].
    pub(crate) fn detect(icon_name: &str) -> Self {
        match icon_name {
            n if n.starts_with("nf-md-") => IconSet::MaterialDesign,
            n if n.starts_with("nf-fa-") => IconSet::FontAwesome,
            n if n.starts_with("nf-dev-") => IconSet::Devicons,
            n if n.starts_with("nf-cod-") => IconSet::Codicons,
            n if n.starts_with("nf-oct-") => IconSet::Octicons,
            n if n.starts_with("nf-weather-") => IconSet::Weather,
            n if n.starts_with("nf-fae-") => IconSet::FontAwesomeExtended,
            n if n.starts_with("nf-seti-") => IconSet::Seti,
            n if n.starts_with("nf-linux-") => IconSet::Linux,
            n if n.starts_with("nf-ple-") => IconSet::PowerlineExtra,
            n if n.starts_with("nf-pl-") => IconSet::Powerline,
            n if n.starts_with("nf-custom-") => IconSet::Custom,
            n if n.starts_with("nf-extra-") => IconSet::Extra,
            n if n.starts_with("nf-pom-") => IconSet::Pomicons,
            n if n.starts_with("nf-alpha-") => IconSet::Alpha,
            n if n.starts_with("nf-iec-") => IconSet::Iec,
            n if n.starts_with("nf-checkbox-") => IconSet::Checkbox,
            n if n.starts_with("nf-file-") => IconSet::File,
            n if n.starts_with("nf-heart-") => IconSet::Heart,
            n if n.starts_with("nf-image-") => IconSet::Image,
            n if n.starts_with("nf-indentation-") => IconSet::Indentation,
            n if n.starts_with("nf-music-") => IconSet::Music,
            n if n.starts_with("nf-near-") => IconSet::Near,
            n if n.starts_with("nf-nonmarkingreturn-") => IconSet::NonMarkingReturn,
            n if n.starts_with("nf-exit-") => IconSet::Exit,
            _ => IconSet::Other,
        }
    }

    /// Returns the Nerd Font name prefix for this icon set.
    ///
    /// # Examples
    ///
    /// ```
    /// use nerd_fonts_gtk::icons::IconSet;
    ///
    /// assert_eq!(IconSet::FontAwesome.prefix(), "nf-fa-");
    /// assert_eq!(IconSet::MaterialDesign.prefix(), "nf-md-");
    /// ```
    pub const fn prefix(self) -> &'static str {
        match self {
            IconSet::MaterialDesign => "nf-md-",
            IconSet::FontAwesome => "nf-fa-",
            IconSet::Devicons => "nf-dev-",
            IconSet::Codicons => "nf-cod-",
            IconSet::Octicons => "nf-oct-",
            IconSet::Weather => "nf-weather-",
            IconSet::FontAwesomeExtended => "nf-fae-",
            IconSet::Seti => "nf-seti-",
            IconSet::Linux => "nf-linux-",
            IconSet::PowerlineExtra => "nf-ple-",
            IconSet::Powerline => "nf-pl-",
            IconSet::Custom => "nf-custom-",
            IconSet::Extra => "nf-extra-",
            IconSet::Pomicons => "nf-pom-",
            IconSet::Alpha => "nf-alpha-",
            IconSet::Iec => "nf-iec-",
            IconSet::Checkbox => "nf-checkbox-",
            IconSet::File => "nf-file-",
            IconSet::Heart => "nf-heart-",
            IconSet::Image => "nf-image-",
            IconSet::Indentation => "nf-indentation-",
            IconSet::Music => "nf-music-",
            IconSet::Near => "nf-near-",
            IconSet::NonMarkingReturn => "nf-nonmarkingreturn-",
            IconSet::Exit => "nf-exit-",
            IconSet::Other => "",
        }
    }

    /// Returns all known icon sets (excluding `Other`).
    pub const fn all() -> &'static [IconSet] {
        &[
            IconSet::MaterialDesign,
            IconSet::FontAwesome,
            IconSet::Devicons,
            IconSet::Codicons,
            IconSet::Octicons,
            IconSet::Weather,
            IconSet::FontAwesomeExtended,
            IconSet::Seti,
            IconSet::Linux,
            IconSet::PowerlineExtra,
            IconSet::Powerline,
            IconSet::Custom,
            IconSet::Extra,
            IconSet::Pomicons,
            IconSet::Alpha,
            IconSet::Iec,
            IconSet::Checkbox,
            IconSet::File,
            IconSet::Heart,
            IconSet::Image,
            IconSet::Indentation,
            IconSet::Music,
            IconSet::Near,
            IconSet::NonMarkingReturn,
            IconSet::Exit,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icons::IconName;

    #[test]
    fn detect_font_awesome() {
        let name = IconName::from_glyph_name("fa-gamepad").unwrap();
        assert_eq!(name.icon_set(), IconSet::FontAwesome);
    }

    #[test]
    fn detect_material_design() {
        let name = IconName::from_glyph_name("md-home").unwrap();
        assert_eq!(name.icon_set(), IconSet::MaterialDesign);
    }

    #[test]
    fn detect_octicons() {
        let name = IconName::from_glyph_name("oct-mark-github").unwrap();
        assert_eq!(name.icon_set(), IconSet::Octicons);
    }

    #[test]
    fn detect_weather() {
        let name = IconName::from_glyph_name("weather-day-sunny").unwrap();
        assert_eq!(name.icon_set(), IconSet::Weather);
    }

    #[test]
    fn detect_powerline_extra() {
        let name = IconName::from_glyph_name("ple-hud").unwrap();
        assert_eq!(name.icon_set(), IconSet::PowerlineExtra);
    }

    #[test]
    fn detect_powerline() {
        let name = IconName::from_glyph_name("pl-branch").unwrap();
        assert_eq!(name.icon_set(), IconSet::Powerline);
    }

    #[test]
    fn detect_linux() {
        let name = IconName::from_glyph_name("linux-tux").unwrap();
        assert_eq!(name.icon_set(), IconSet::Linux);
    }

    #[test]
    fn from_glyph_name_rejects_unknown_prefix() {
        assert!(IconName::from_glyph_name("unknown-icon").is_none());
    }

    #[test]
    fn from_glyph_name_rejects_empty() {
        assert!(IconName::from_glyph_name("").is_none());
    }

    #[test]
    fn prefix_round_trip() {
        for set in IconSet::all() {
            let name =
                IconName::from_glyph_name(&format!("{}test", &set.prefix()[3..])).unwrap_or_else(|| panic!("failed to construct IconName for {:?}", set));
            assert_eq!(name.icon_set(), *set);
        }
    }

    #[test]
    fn all_excludes_other() {
        assert!(!IconSet::all().contains(&IconSet::Other));
    }

    #[test]
    fn all_contains_all_known_sets() {
        assert_eq!(IconSet::all().len(), 25);
    }
}
