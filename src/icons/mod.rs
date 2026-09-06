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

use std::convert::TryFrom;

/// Convert an [`IconName`] to a [`CodePoint`] via the codepoint map.
impl TryFrom<&IconName> for CodePoint {
    type Error = CodePointParseError;

    fn try_from(icon_name: &IconName) -> Result<Self, Self::Error> {
        resolve_icon_codepoint(icon_name.as_ref()).map(Self).ok_or(CodePointParseError::IconNotFound)
    }
}

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

/// Resolve a Nerd Font icon name (e.g. `nf-weather-day_sunny`) to its
/// Unicode codepoint character by looking it up in the reverse codepoint map.
///
/// The name is normalized to the GTK symbolic icon name format
/// (kebab-case, `-symbolic` suffix) before lookup.
///
/// Lookup is O(1) via a compile-time `phf::Map`.
pub fn resolve_icon_codepoint(icon_name: &str) -> Option<char> {
    let normalized = icon_name.replace('_', "-").to_lowercase();
    let with_suffix = if normalized.ends_with("-symbolic") {
        normalized
    } else {
        format!("{}-symbolic", normalized)
    };
    codepoint_map::REVERSE_ICONS.get(with_suffix.as_str()).copied()
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
