//! Optional icon set detection capability for multi-set icon fonts.
//!
//! Provides the [`IconSetDetect`] trait for icon font families whose
//! namespace is split into named sets distinguished by name prefix
//! (e.g. Nerd Fonts: `nf-fa-*` for Font Awesome, `nf-md-*` for Material
//! Design). Single-set icon fonts simply don't implement this trait -
//! [`IconName::icon_set`](crate::IconName::icon_set) is then absent at
//! compile time.

use crate::FontFamilyConfig;

/// Prefix-based icon set detection for a font family.
///
/// Implemented by families whose icon namespace aggregates multiple
/// upstream sets. The associated [`Set`](Self::Set) type is the
/// family-specific set enumeration (e.g. `IconSet` for Nerd Fonts).
///
/// The detection result is also the natural implementation of
/// [`FontFamilyConfig::is_valid_icon_name`]: a name is valid when it
/// resolves to a known set.
pub trait IconSetDetect: FontFamilyConfig {
    /// The icon set type produced by detection.
    ///
    /// Typically a family-local enum with a fallback variant for
    /// unknown prefixes (e.g. `IconSet::Other`).
    type Set;

    /// Detects the icon set from a fully-formed icon name
    /// (`{ICON_PREFIX}{name}-symbolic`).
    fn detect_set(icon_name: &str) -> Self::Set;
}
