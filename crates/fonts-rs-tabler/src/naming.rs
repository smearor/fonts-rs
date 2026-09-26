//! Font family marker type for Tabler Icons.
//!
//! The `Tabler` marker is used as the phantom type parameter in
//! generic name types, ensuring type safety at compile time.

use crate::definition::TablerConfig;
use fonts_rs_model::FontFamilyMarker;
use fonts_rs_model::IconName;
use fonts_rs_model::sealed;

/// Marker type identifying Tabler Icons in generic name types.
///
/// Zero-sized enum used as the phantom type parameter to ensure type
/// safety: `IconName<TablerConfig>` is distinct from other families'
/// icon names at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tabler {}

impl FontFamilyMarker for Tabler {}

impl sealed::Sealed for Tabler {}

/// Tabler icon name in canonical `tabler-{name}-symbolic` form.
///
/// Normalization (lowercase, kebab-case, `-symbolic` suffix) and
/// `tabler-` prefix validation come from the generic
/// [`IconName`](fonts_rs_model::IconName) via [`TablerConfig`].
pub type TablerIconName = IconName<TablerConfig>;
