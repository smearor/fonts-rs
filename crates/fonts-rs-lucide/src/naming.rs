//! Font family marker type for Lucide Icons.
//!
//! The `Lucide` marker is used as the phantom type parameter in
//! generic name types, ensuring type safety at compile time.

use crate::definition::LucideConfig;
use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::IconName;
use fonts_rs_model::sealed;

/// Marker type identifying Lucide Icons in generic name types.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `IconName<LucideConfig>` is distinct from other families'
/// icon names at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Lucide {}

impl FontFamilyMarker for Lucide {}

impl sealed::Sealed for Lucide {}

/// Lucide icon name in canonical `lucide-{name}-symbolic` form.
///
/// Normalization (lowercase, kebab-case, `-symbolic` suffix) and
/// `lucide-` prefix validation come from the generic
/// [`IconName`](fonts_rs_model::IconName) via [`LucideConfig`].
pub type LucideIconName = IconName<LucideConfig>;
