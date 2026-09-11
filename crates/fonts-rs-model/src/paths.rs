//! Shared constants for GResource paths.
//!
//! All font families in the `fonts-rs` ecosystem share a common base
//! GResource prefix. Each family appends its own sub-path to this base.

/// Base GResource prefix for all fonts-rs resources.
///
/// Individual font families build their full prefix by appending their
/// family-specific sub-path (e.g. `nerd_fonts`, `seven_segment`).
///
/// # Examples
///
/// - Nerd Fonts: `{GRESOURCE_BASE_PREFIX}/nerd_fonts` -> `/io/smearor/fonts/nerd_fonts`
/// - Seven Segment: `{GRESOURCE_BASE_PREFIX}/seven_segment` -> `/io/smearor/fonts/seven_segment`
pub const GRESOURCE_BASE_PREFIX: &str = "/io/smearor/fonts";
