//! GResource path newtype for type-safe resource path handling.

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;

/// A GResource path for a font glyph.
///
/// Constructed from a resource prefix and glyph name, producing a path
/// like `/io/smearor/fonts/seven_segment/scalable/glyphs/dseg7-0` or
/// `/io/smearor/fonts/nerd_fonts/scalable/glyphs/nf-fa-gamepad-symbolic`.
///
/// The `scalable/{context}` directory structure follows the GTK 4
/// `GtkIconTheme` convention, enabling automatic icon resolution via
/// `GtkIconTheme::add_resource_path`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourcePath(String);

impl ResourcePath {
    /// Construct a `ResourcePath` from a resource prefix and glyph name.
    ///
    /// # Examples
    ///
    /// ```
    /// use fonts_rs_model::ResourcePath;
    ///
    /// let path = ResourcePath::from_name(
    ///     "/io/smearor/fonts/seven_segment/scalable/glyphs",
    ///     "dseg7-0",
    /// );
    /// assert_eq!(path.as_ref(), "/io/smearor/fonts/seven_segment/scalable/glyphs/dseg7-0");
    /// ```
    pub fn from_name(prefix: &str, name: &str) -> Self {
        Self(format!("{}/{}", prefix, name))
    }
}

impl AsRef<str> for ResourcePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for ResourcePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ResourcePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
    }
}
