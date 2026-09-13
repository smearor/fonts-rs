//! Font axis value types.
//!
//! Provides [`AxisValue`] (a single axis + value pair) and [`AxisValues`]
//! (a collection of axis values) for variable font instance specification.

use crate::axis::Axis;

/// A single axis value: an [`Axis`] paired with a user-space coordinate.
///
/// e.g. `AxisValue::new("wght", 700.0)` selects weight 700 for a variable font.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisValue {
    /// The axis this value applies to.
    pub axis: Axis,
    /// The user-space coordinate for this axis.
    pub value: f32,
}

impl AxisValue {
    /// Creates a new [`AxisValue`] from an axis name and a value.
    pub const fn new(axis: &'static str, value: f32) -> Self {
        Self { axis: Axis::new(axis), value }
    }
}

/// A collection of [`AxisValue`]s defining a variation location.
///
/// Wraps a `&'static [AxisValue]` so it can be constructed in `const` context
/// for use in `VARIANTS` arrays in build scripts.
///
/// # Example
///
/// ```
/// use fonts_rs_model::AxisValue;
/// use fonts_rs_model::AxisValues;
///
/// const LOCATION: AxisValues = AxisValues::new(&[
///     AxisValue::new("wght", 700.0),
///     AxisValue::new("ROND", 50.0),
/// ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisValues(&'static [AxisValue]);

impl AxisValues {
    /// An empty set of axis values (default location).
    pub const EMPTY: AxisValues = AxisValues(&[]);

    /// Creates a new [`AxisValues`] from a static slice of [`AxisValue`]s.
    pub const fn new(values: &'static [AxisValue]) -> Self {
        Self(values)
    }

    /// Returns `true` if no axis values are set.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the axis values as a slice.
    pub fn as_slice(&self) -> &[AxisValue] {
        self.0
    }

    /// Returns an iterator over the axis values.
    pub fn iter(&self) -> std::slice::Iter<'_, AxisValue> {
        self.0.iter()
    }
}

impl Default for AxisValues {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::ops::Deref for AxisValues {
    type Target = [AxisValue];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
