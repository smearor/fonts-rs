//! Font family marker type for Fluent UI System Icons.
//!
//! Defines the [`Fluent`] marker type used in generic name types, and
//! the [`FluentIconName`] alias for the family's typed icon names.

use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::IconName;
use fonts_rs_model::sealed;

use crate::definition::FluentConfig;

/// Marker type identifying Fluent UI System Icons in generic name types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Fluent {}

impl FontFamilyMarker for Fluent {}

impl sealed::Sealed for Fluent {}

/// Fluent icon name in canonical form.
///
/// `fluent-{name}-symbolic` for the regular variant,
/// `fluent-filled-{name}-symbolic` for the `filled` variant.
pub type FluentIconName = IconName<FluentConfig>;
