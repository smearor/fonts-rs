//! Font axis identifier type.
//!
//! Provides an [`Axis`] newtype wrapping a `&'static str` axis name
//! (e.g. `"wght"`, `"ROND"`) for type-safe axis handling in variable fonts.

/// A variable font axis name, e.g. `"wght"`, `"ROND"`, `"wdth"`.
///
/// Wraps a `&'static str` to provide type-safe axis identification.
/// The axis name corresponds to the OpenType axis tag as a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Axis(&'static str);

impl Axis {
    /// Creates a new [`Axis`] from a static string.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the axis name as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl From<&'static str> for Axis {
    fn from(name: &'static str) -> Self {
        Self(name)
    }
}

impl AsRef<str> for Axis {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
