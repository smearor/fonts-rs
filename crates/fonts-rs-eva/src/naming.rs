//! Font family marker type for Eva Icons.
//!
//! The `Eva` marker is used as the phantom type parameter in
//! generic name types, ensuring type safety at compile time.

use crate::definition::EvaConfig;
use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::IconName;
use fonts_rs_model::sealed;

/// Marker type identifying Eva Icons in generic name types.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `IconName<EvaConfig>` is distinct from other families'
/// icon names at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Eva {}

impl FontFamilyMarker for Eva {}

impl sealed::Sealed for Eva {}

/// Eva icon name in canonical `eva-{name}-symbolic` form.
///
/// Normalization (lowercase, kebab-case, `-symbolic` suffix) and
/// `eva-` prefix validation come from the generic
/// [`IconName`](fonts_rs_model::IconName) via [`EvaConfig`].
pub type EvaIconName = IconName<EvaConfig>;
