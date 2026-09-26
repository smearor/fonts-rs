//! Font family marker type for Heroicons.
//!
//! The `Heroicons` marker is used as the phantom type parameter in
//! generic name types, ensuring type safety at compile time.

use crate::definition::HeroiconsConfig;
use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::IconName;
use fonts_rs_model::sealed;

/// Marker type identifying Heroicons in generic name types.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `IconName<HeroiconsConfig>` is distinct from other families'
/// icon names at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Heroicons {}

impl FontFamilyMarker for Heroicons {}

impl sealed::Sealed for Heroicons {}

/// Heroicons icon name in canonical `heroicons-{name}-symbolic` form.
///
/// Normalization (lowercase, kebab-case, `-symbolic` suffix) and
/// `heroicons-` prefix validation come from the generic
/// [`IconName`](fonts_rs_model::IconName) via [`HeroiconsConfig`].
pub type HeroiconsIconName = IconName<HeroiconsConfig>;
