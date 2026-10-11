//! Newtype wrapping a `HashMap<String, CodePoint>` for type-safe glyph name lookups.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Deserializer;

use crate::CodePoint;

/// A mapping from glyph names to Unicode codepoints.
///
/// Used by build scripts to drive glyph export via
/// `ExportConfig::export_glyphs_by_name_map` (in `fonts_rs_generator`) when font-internal
/// glyph names are not semantically meaningful (e.g. `uniXXXX` PostScript
/// names) and must be replaced with names from external metadata
/// (CLDR annotations, SMuFL glyphnames.json).
#[derive(Debug, Clone, Default)]
pub struct GlyphNameMap(HashMap<String, CodePoint>);

impl GlyphNameMap {
    /// Creates an empty `GlyphNameMap`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns a reference to the codepoint for the given glyph name.
    pub fn get(&self, name: &str) -> Option<&CodePoint> {
        self.0.get(name)
    }

    /// Inserts a glyph name → codepoint mapping.
    ///
    /// If the name already exists, the old value is returned.
    pub fn insert(&mut self, name: String, codepoint: CodePoint) -> Option<CodePoint> {
        self.0.insert(name, codepoint)
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the map entries.
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, CodePoint> {
        self.0.iter()
    }
}

impl IntoIterator for GlyphNameMap {
    type Item = (String, CodePoint);
    type IntoIter = std::collections::hash_map::IntoIter<String, CodePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a GlyphNameMap {
    type Item = (&'a String, &'a CodePoint);
    type IntoIter = std::collections::hash_map::Iter<'a, String, CodePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<(String, CodePoint)> for GlyphNameMap {
    fn from_iter<I: IntoIterator<Item = (String, CodePoint)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'de> Deserialize<'de> for GlyphNameMap {
    /// Deserializes a vendor glyph-name → codepoint map into a [`GlyphNameMap`].
    ///
    /// Accepts the common upstream JSON shapes transparently:
    ///
    /// - SMuFL object: `{ "glyphName": { "codepoint": "U+E050", ... } }` -
    ///   only `codepoint` is extracted, other fields are ignored.
    /// - Flat integer: `{ "glyphName": 57994 }` (e.g. Fluent System Icons).
    /// - String: `{ "glyphName": "U+E050" }` or bare hex `"E050"`.
    /// - Object with raw int: `{ "glyphName": { "codepoint": 57400 } }`.
    ///
    /// Key normalization (prefix/suffix stripping, name aliasing) is not
    /// performed here - that is inherently vendor-specific and belongs in
    /// the consumer.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// A codepoint scalar in either numeric or string form.
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Scalar {
            /// Raw scalar value (e.g. `57994`).
            Int(u32),
            /// Hex or `U+` prefixed string, parsed by `CodePoint`.
            String(CodePoint),
        }

        /// One `glyphName` entry: bare scalar or `{ "codepoint": ... }` object.
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Entry {
            /// The value itself is the codepoint.
            Scalar(Scalar),
            /// SMuFL-style object; only `codepoint` is extracted.
            Object { codepoint: Scalar },
        }

        let map: HashMap<String, Entry> = HashMap::deserialize(deserializer)?;
        map.into_iter()
            .map(|(name, entry)| {
                let scalar = match entry {
                    Entry::Scalar(scalar) => scalar,
                    Entry::Object { codepoint } => codepoint,
                };
                let codepoint = match scalar {
                    Scalar::Int(value) => CodePoint::try_from(value).map_err(serde::de::Error::custom)?,
                    Scalar::String(codepoint) => codepoint,
                };
                Ok((name, codepoint))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::GlyphNameMap;

    /// SMuFL shape: `{ "name": { "codepoint": "U+E050", ...ignored } }`.
    #[test]
    fn deserialize_smufl_object() {
        let json = r#"{ "accidentalFlat": { "codepoint": "U+E260", "description": "Flat" } }"#;
        let map: GlyphNameMap = serde_json::from_str(json).unwrap();
        assert_eq!(map.get("accidentalFlat").unwrap().as_char() as u32, 0xE260);
    }

    /// Flat integer map: `{ "name": 57994 }` (Fluent System Icons).
    #[test]
    fn deserialize_flat_integer() {
        let json = r#"{ "ic_fluent_add_24_regular": 57994 }"#;
        let map: GlyphNameMap = serde_json::from_str(json).unwrap();
        assert_eq!(map.get("ic_fluent_add_24_regular").unwrap().as_char() as u32, 57994);
    }

    /// Integer values above the Basic Multilingual Plane (PUA-A).
    #[test]
    fn deserialize_supplementary_plane() {
        let json = r#"{ "icon": 984566 }"#; // U+F05F6
        let map: GlyphNameMap = serde_json::from_str(json).unwrap();
        assert_eq!(map.get("icon").unwrap().as_char() as u32, 0xF05F6);
    }

    /// String codepoints with and without the `U+` prefix.
    #[test]
    fn deserialize_string_forms() {
        let json = r#"{ "a": "U+E050", "b": "E051" }"#;
        let map: GlyphNameMap = serde_json::from_str(json).unwrap();
        assert_eq!(map.get("a").unwrap().as_char() as u32, 0xE050);
        assert_eq!(map.get("b").unwrap().as_char() as u32, 0xE051);
    }

    /// SMuFL variant with a raw integer inside the object.
    #[test]
    fn deserialize_object_int_codepoint() {
        let json = r#"{ "x": { "codepoint": 57400 } }"#;
        let map: GlyphNameMap = serde_json::from_str(json).unwrap();
        assert_eq!(map.get("x").unwrap().as_char() as u32, 57400);
    }

    /// Surrogate-range scalar values are rejected.
    #[test]
    fn deserialize_rejects_surrogate() {
        let json = r#"{ "bad": 55296 }"#; // U+D800
        assert!(serde_json::from_str::<GlyphNameMap>(json).is_err());
    }
}
