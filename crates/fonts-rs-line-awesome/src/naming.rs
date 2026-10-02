//! Font family marker type for Line Awesome.
//!
//! Defines the [`LineAwesome`] marker type used in generic name types,
//! and the [`LineAwesomeIconName`] alias for the family's typed icon
//! names.

use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::IconName;
use fonts_rs_model::sealed;

use crate::definition::LineAwesomeConfig;

/// Marker type identifying Line Awesome in generic name types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LineAwesome {}

impl FontFamilyMarker for LineAwesome {}

impl sealed::Sealed for LineAwesome {}

/// Line Awesome icon name in canonical form.
///
/// `la-{name}-symbolic` for regular/brand icons,
/// `la-{name}-solid-symbolic` for the solid style.
pub type LineAwesomeIconName = IconName<LineAwesomeConfig>;
