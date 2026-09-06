//! Vendored Nerd Font icons: GResource registration, codepoint maps, and constants.
//!
//! This module replaces the external `nerd_gtk_icons` crate. It provides:
//! - `register_icons()`: registers the compiled GResource bundle
//! - `codepoint_map::ICONS`: `phf::Map<char, &'static str>` (codepoint -> name)
//! - `codepoint_map::REVERSE_ICONS`: `phf::Map<&'static str, char>` (name -> codepoint)
//! - `icon_constants`: auto-generated icon name constants
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

/// Shared constants for icon resource paths.
pub mod paths;

pub use codepoint::CodePoint;
pub use codepoint::CodePointParseError;
pub use name::IconName;
pub use paths::GRESOURCE_PREFIX;
pub use paths::ICONS_RESOURCE_PATH;
pub use resource_path::ResourcePath;
pub use set::IconSet;

/// Registers embedded Nerd Font icons as a GResource.
///
/// Must be called once before using any icons through GTK/GIO APIs.
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
pub fn register_icons() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")
}

/// Auto-generated Nerd Font icon name constants.
///
/// Each constant represents a GTK-compatible icon name (lowercase with
/// `-symbolic` suffix for symbolic rendering).
pub mod icon_constants {
    include!(concat!(env!("OUT_DIR"), "/icons.rs"));
}

/// Mapping between Unicode codepoints and icon names.
///
/// - `ICONS`: codepoint -> icon name
/// - `REVERSE_ICONS`: icon name -> codepoint
pub mod codepoint_map {
    include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
}

/// Unicode codepoint newtype for type-safe codepoint handling.
pub mod codepoint;

/// Icon collection enum for prefix-based grouping.
pub mod set;

/// Normalized GTK icon name newtype for type-safe icon name handling.
pub mod name;

/// GResource path newtype for type-safe resource path handling.
pub mod resource_path;

/// SVG path data builder for glyph outline extraction.
///
/// Available when the `export` feature is enabled.
#[cfg(feature = "export")]
pub mod svg_path_builder;

/// Metadata entry for a single exported Nerd Font icon.
///
/// Available when the `export` feature is enabled.
#[cfg(feature = "export")]
pub mod entry;

/// Glyph export from TTF/OTF fonts to SVG icons.
///
/// Available when the `export` feature is enabled. Provides
/// [`export::export_icons()`] for generating SVG icons, `metadata.json`,
/// and `icons.gresource.xml` from a font file.
#[cfg(feature = "export")]
pub mod export;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_name_codepoint_known_icon() {
        let name = IconName::from_glyph_name("fa-gamepad").unwrap();
        let codepoint = name.codepoint();
        assert!(codepoint.is_some());
        assert_eq!(codepoint.unwrap().as_char(), '\u{F11B}');
    }

    #[test]
    fn icon_name_codepoint_with_underscores() {
        let name = IconName::from_glyph_name("linux_tux").unwrap();
        let codepoint = name.codepoint();
        assert!(codepoint.is_some());
        assert_eq!(codepoint.unwrap().as_char(), '\u{F31A}');
    }

    #[test]
    fn icon_name_codepoint_unknown_icon() {
        let name = IconName::from_glyph_name("fa-nonexistent-xyz");
        if let Some(name) = name {
            assert_eq!(name.codepoint(), None);
        }
    }

    #[test]
    fn try_from_icon_name_to_codepoint() {
        let name = IconName::from_glyph_name("fa-gamepad").unwrap();
        let codepoint: Result<CodePoint, _> = CodePoint::try_from(&name);
        assert!(codepoint.is_ok());
        assert_eq!(codepoint.unwrap().as_char(), '\u{F11B}');
    }
}
