//! Unicode emoji-test.txt parsing for emoji categories.
//!
//! The file has `# group: Category Name` and `# subgroup: Sub Name` headers,
//! followed by lines like `1F600 ; fully-qualified # 😀 E1.0 grinning face`.

use std::collections::HashMap;

/// Parse emoji-test.txt to build a codepoint→category map.
///
/// The file has `# group: Category Name` and `# subgroup: Sub Name` headers,
/// followed by lines like `1F600 ; fully-qualified # 😀 E1.0 grinning face`.
pub fn parse_emoji_test(content: &str) -> HashMap<u32, String> {
    let mut categories = HashMap::new();
    let mut current_group = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("# group:") {
            current_group = rest.trim().to_string();
        } else if !trimmed.starts_with('#') && !trimmed.is_empty() {
            // Parse codepoint: "1F600 ; fully-qualified # ..."
            if let Some(codepoint_str) = trimmed.split_whitespace().next() {
                if let Ok(cp) = u32::from_str_radix(codepoint_str, 16) {
                    if !current_group.is_empty() {
                        categories.insert(cp, current_group.clone());
                    }
                }
            }
        }
    }

    categories
}
