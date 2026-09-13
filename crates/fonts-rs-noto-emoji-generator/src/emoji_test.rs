//! Unicode emoji-test.txt parsing for emoji categories.
//!
//! The file has `# group: Category Name` and `# subgroup: Sub Name` headers,
//! followed by lines like `1F600 ; fully-qualified # 😀 E1.0 grinning face`.

use fonts_rs_model::CodePoint;
use fonts_rs_model::CodePointCategoryMap;

/// Parse emoji-test.txt to build a codepoint→category map.
///
/// The file has `# group: Category Name` and `# subgroup: Sub Name` headers,
/// followed by lines like `1F600 ; fully-qualified # 😀 E1.0 grinning face`.
pub fn parse_emoji_test(content: &str) -> CodePointCategoryMap {
    let mut categories = CodePointCategoryMap::new();
    let mut current_group = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("# group:") {
            current_group = rest.trim().to_string();
        } else if !trimmed.starts_with('#') && !trimmed.is_empty() {
            // Parse codepoint: "1F600 ; fully-qualified # ..."
            if let Some(codepoint_str) = trimmed.split_whitespace().next() {
                if let Ok(cp) = u32::from_str_radix(codepoint_str, 16) {
                    if let Some(ch) = char::from_u32(cp) {
                        if !current_group.is_empty() {
                            categories.insert(CodePoint::from(ch), current_group.clone());
                        }
                    }
                }
            }
        }
    }

    categories
}
