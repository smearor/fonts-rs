//! CLDR annotation parsing for emoji keywords and semantic names.
//!
//! The CLDR JSON uses the actual emoji character as the key.
//! The `tts` field contains the canonical name (e.g. "grinning face").
//! The `default` field contains search keywords (e.g. ["face", "grin"]).

use std::collections::HashMap;

use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointKeywordMap;
use fonts_rs_model::CodePointNameMap;
use fonts_rs_model::GlyphNameMap;
use serde::Deserialize;

/// CLDR annotation entry: `tts` (name) and `default` (keywords).
///
/// Fields are optional because the JSON also contains an `identity` entry
/// with `language` instead of annotation data.
#[derive(Deserialize)]
pub struct CldrAnnotation {
    #[serde(rename = "tts", default)]
    tts: Vec<String>,
    #[serde(rename = "default", default)]
    default: Vec<String>,
}

/// CLDR annotations inner structure: contains `identity` and the actual annotations map.
#[derive(Deserialize)]
struct CldrAnnotationsInner {
    #[serde(default)]
    _identity: serde_json::Value,
    annotations: HashMap<String, CldrAnnotation>,
}

/// CLDR annotations root structure.
#[derive(Deserialize)]
struct CldrAnnotations {
    annotations: CldrAnnotationsInner,
}

/// Convert a CLDR tts name to a kebab-case glyph name.
///
/// e.g. "grinning face" → "grinning-face"
///      "face with tears of joy" → "face-with-tears-of-joy"
fn name_to_kebab(name: &str) -> String {
    let kebab: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect();
    let kebab: String = kebab.split_whitespace().collect::<Vec<_>>().join("-");
    kebab
}

/// Parse CLDR annotations JSON into (name→codepoint map, keyword map, name map).
///
/// The CLDR JSON uses the actual emoji character as the key.
/// The `tts` field contains the canonical name (e.g. "grinning face").
/// The `default` field contains search keywords (e.g. ["face", "grin"]).
pub fn parse_cldr_annotations(json: &str) -> miette::Result<(GlyphNameMap, CodePointKeywordMap, CodePointNameMap)> {
    let data: CldrAnnotations = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse CLDR annotations: {e}"))?;

    let mut name_map = GlyphNameMap::new();
    let mut keyword_map = CodePointKeywordMap::new();
    let mut name_by_codepoint = CodePointNameMap::new();

    for (char_str, annotation) in &data.annotations.annotations {
        // The key is the actual emoji character(s). For single-codepoint emoji,
        // take the first char. For sequences (e.g. skin tone modifiers), skip
        // multi-codepoint entries that aren't in the font.
        let Some(ch) = char_str.chars().next() else { continue };
        if char_str.chars().count() > 1 {
            continue;
        }
        let codepoint = CodePoint::from(ch);

        // Skip entries without a tts name (e.g. the identity entry)
        if annotation.tts.is_empty() {
            continue;
        }

        // Use the first tts entry as the canonical name
        if let Some(name) = annotation.tts.first() {
            let kebab = name_to_kebab(name);
            if !kebab.is_empty() {
                name_map.insert(kebab.clone(), codepoint);
                name_by_codepoint.insert(codepoint, kebab);
            }
        }

        if !annotation.default.is_empty() {
            keyword_map.insert(codepoint, annotation.default.clone());
        }
    }

    Ok((name_map, keyword_map, name_by_codepoint))
}
