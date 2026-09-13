//! Newtype wrapper for SVG content.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

/// SVG content as a string newtype.
///
/// Wraps the SVG markup returned by [`Font::glyph_to_svg`](crate::font::Font::glyph_to_svg)
/// and related methods. Provides a type-safe alternative to bare `String` and
/// a [`Default`] implementation that yields an empty placeholder SVG.
#[derive(Debug, Clone, PartialEq)]
pub struct Svg(String);

impl Svg {
    /// Create an `Svg` from a string.
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }

    /// Returns the SVG content as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the inner `String`.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Write the SVG content to a file at the given path.
    pub fn write_to(&self, path: &Path) -> io::Result<()> {
        fs::write(path, &self.0)
    }
}

impl Default for Svg {
    /// Returns an empty SVG placeholder.
    fn default() -> Self {
        Self(crate::svg::EMPTY_SVG.to_string())
    }
}

impl From<String> for Svg {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Svg {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for Svg {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
