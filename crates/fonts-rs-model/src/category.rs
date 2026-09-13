//! Glyph category label newtype for type-safe category handling.

use std::fmt;

/// A category label for a glyph.
///
/// Wraps a static string slice with type safety. Use [`GlyphCategory::as_str`]
/// or `AsRef<str>` to access the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlyphCategory(&'static str);

impl GlyphCategory {
    /// Creates a `GlyphCategory` from a static string slice.
    pub const fn new(category: &'static str) -> Self {
        Self(category)
    }

    /// Returns the category as a string slice.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for GlyphCategory {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for GlyphCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl PartialEq<str> for GlyphCategory {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
