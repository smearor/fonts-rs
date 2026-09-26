//! Glyph group label newtype for type-safe group handling.

use std::fmt;

/// A single group label for a glyph (e.g. an emoji picker group).
///
/// Wraps a static string slice with type safety. Use
/// [`GroupLabel::as_str`] or `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupLabel(&'static str);

impl GroupLabel {
    /// Creates a `GroupLabel` from a static string slice.
    pub const fn new(label: &'static str) -> Self {
        Self(label)
    }

    /// Returns the group label as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for GroupLabel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for GroupLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl PartialEq<str> for GroupLabel {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
