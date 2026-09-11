//! Shared constants for GResource paths.
//!
//! Nerd Fonts-specific paths built on top of the generic base prefix
//! from `fonts-rs-model`.

use const_format::concatcp;

/// GResource prefix for Nerd Fonts resources.
///
/// Built from the shared base prefix: `{GRESOURCE_BASE_PREFIX}/nerd_fonts`.
pub const GRESOURCE_PREFIX: &str = "/io/smearor/fonts/nerd_fonts";

/// Root path inside the compiled GResource bundle where icons are stored.
///
/// `{GRESOURCE_PREFIX}/icons`
pub const ICONS_RESOURCE_PATH: &str = concatcp!(GRESOURCE_PREFIX, "/icons");
