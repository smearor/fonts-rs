//! Phantom-typed glyph name newtype for compile-time font family safety.

use crate::font_family_marker::FontFamilyMarker;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

/// A normalized glyph name for a specific font family.
///
/// Wraps a `String` with standard trait implementations shared across all
/// font families. The phantom type parameter `F` ensures type safety:
/// a `GlyphName<SevenSegment>` is a distinct type from `GlyphName<Barcode>`.
///
/// Construction and normalization logic is provided by each font family
/// crate's `FontDefinition::normalize_name` implementation, not by this
/// type directly. Use [`GlyphName::new`] to wrap an already-normalized
/// string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlyphName<F: FontFamilyMarker> {
    /// The normalized glyph name string.
    name: String,
    /// Phantom marker for the font family type.
    _marker: PhantomData<F>,
}

impl<F: FontFamilyMarker> GlyphName<F> {
    /// Creates a `GlyphName` from an already-normalized string.
    ///
    /// This constructor is called by `FontDefinition::normalize_name`
    /// implementations after applying family-specific normalization rules.
    pub fn new(name: String) -> Self {
        Self { name, _marker: PhantomData }
    }
}

impl<F: FontFamilyMarker> AsRef<str> for GlyphName<F> {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl<F: FontFamilyMarker> fmt::Display for GlyphName<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl<F: FontFamilyMarker> Serialize for GlyphName<F> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl<'de, F: FontFamilyMarker> Deserialize<'de> for GlyphName<F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Ok(Self { name, _marker: PhantomData })
    }
}

impl<F: FontFamilyMarker> FromStr for GlyphName<F> {
    type Err = Infallible;

    /// Wraps `s` without validation — normalization is the job of
    /// `FontDefinition::normalize_name`, not of this newtype.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl<F: FontFamilyMarker> From<&str> for GlyphName<F> {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}

impl<F: FontFamilyMarker> From<String> for GlyphName<F> {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}
