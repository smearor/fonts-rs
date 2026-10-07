use crate::json::to_json;
use crate::lockfile::LockedInput;
use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::sha256::sha256_hex;
use miette::IntoDiagnostic;
use miette::miette;
use std::collections::BTreeMap;

/// `heroicons_keywords` provider: extracts the heroicons.com search
/// keyword map (`name -> [keywords]`) from the site's JavaScript
/// bundle and emits a sorted `{name: [keyword, ...]}` JSON object.
///
/// The keywords exist only inside the deployed site bundle (the
/// website source is not public): the provider fetches the index
/// page, downloads its script chunks and brace-matches the embedded
/// object literal starting at `{"academic-cap":[`.
///
/// `with.site` overrides the base URL (default `https://heroicons.com`).
pub struct HeroiconsKeywords;

impl Provider for HeroiconsKeywords {
    fn name(&self) -> &'static str {
        "heroicons_keywords"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let site = context
            .with
            .get("site")
            .and_then(|value| value.as_str())
            .unwrap_or("https://heroicons.com")
            .trim_end_matches('/');

        let mut inputs = Vec::new();
        let index_url = format!("{site}/");
        let html = context.http_get(&index_url)?;
        inputs.push(LockedInput {
            url: index_url,
            sha256: sha256_hex(&html),
            provider: self.name().to_string(),
        });
        let html = String::from_utf8(html).into_diagnostic()?;

        for chunk_url in script_urls(&html, site) {
            let body = context.http_get(&chunk_url)?;
            if !contains_keywords_map(&body) {
                continue;
            }
            inputs.push(LockedInput {
                url: chunk_url,
                sha256: sha256_hex(&body),
                provider: self.name().to_string(),
            });
            let keywords = extract_keywords(&body)?;
            let content = to_json(&keywords).map_err(|e| e.wrap_err("heroicons_keywords: failed to serialize keywords"))?;
            return Ok(ProviderOutput {
                content: content.into_bytes(),
                inputs,
            });
        }
        Err(miette!("heroicons_keywords: no script chunk on {site} contains the keywords map"))
    }
}

/// Collects `src` attributes of `<script>` tags, resolved against the
/// site origin.
fn script_urls(html: &str, site: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for segment in html.split("src=\"").skip(1) {
        if let Some(src) = segment.split('"').next()
            && src.ends_with(".js")
        {
            urls.push(if src.starts_with("http") { src.to_string() } else { format!("{site}{src}") });
        }
    }
    urls
}

/// Whether a chunk plausibly contains the keywords object.
fn contains_keywords_map(chunk: &[u8]) -> bool {
    chunk.windows(b"\"academic-cap\":[".len()).any(|w| w == b"\"academic-cap\":[")
}

/// Extracts the `{name: [keywords]}` object literal starting at
/// `{"academic-cap":[` by brace matching, then parses it as JSON
/// after sanitizing JavaScript-isms (unquoted keys, trailing
/// commas).
fn extract_keywords(chunk: &[u8]) -> miette::Result<BTreeMap<String, Vec<String>>> {
    let text = String::from_utf8_lossy(chunk);
    let start = text
        .find("{\"academic-cap\":[")
        .ok_or_else(|| miette!("heroicons_keywords: keywords object start not found"))?;
    let object = &text[start..start + matching_brace(&text[start..])?];
    let json = sanitize_js_object(object)?;
    serde_json::from_str(&json)
        .into_diagnostic()
        .map_err(|e| e.wrap_err("heroicons_keywords: failed to parse keywords object"))
}

/// Returns the length of the `{...}` literal starting at byte 0 of
/// `text` (which must begin with `{`), tracking string literals and
/// escapes. Fails when the object is unterminated.
fn matching_brace(text: &str) -> miette::Result<usize> {
    if !text.starts_with('{') {
        return Err(miette!("heroicons_keywords: internal error, object does not start with '{{'"));
    }
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (index, c) in text.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(index + c.len_utf8());
                }
            }
            _ => {}
        }
    }
    Err(miette!("heroicons_keywords: unterminated keywords object"))
}

/// Converts a JavaScript object literal to JSON: wraps bare
/// identifier keys in quotes and removes trailing commas. String
/// literals (with escapes) are preserved; a container stack tracks
/// whether `,` separates object entries or array elements.
fn sanitize_js_object(text: &str) -> miette::Result<String> {
    let mut out = String::with_capacity(text.len() + 64);
    let mut stack: Vec<u8> = Vec::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut expect_key = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            out.push(c);
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                expect_key = false;
                out.push(c);
            }
            '{' => {
                stack.push(b'{');
                expect_key = true;
                out.push(c);
            }
            '[' => {
                stack.push(b'[');
                expect_key = false;
                out.push(c);
            }
            '}' | ']' => {
                stack.pop();
                expect_key = false;
                out.push(c);
            }
            ',' => {
                if matches!(chars.peek(), Some('}') | Some(']')) {
                    continue;
                }
                expect_key = stack.last() == Some(&b'{');
                out.push(c);
            }
            c if c.is_whitespace() => out.push(c),
            c if expect_key && (c.is_ascii_alphanumeric() || c == '_' || c == '$') => {
                let mut key = String::from(c);
                while let Some(&k) = chars.peek() {
                    if k.is_ascii_alphanumeric() || matches!(k, '_' | '$' | '-') {
                        key.push(k);
                        chars.next();
                    } else {
                        break;
                    }
                }
                while matches!(chars.peek(), Some(' ')) {
                    chars.next();
                }
                if chars.next() != Some(':') {
                    return Err(miette!("heroicons_keywords: unexpected token after key '{key}'"));
                }
                out.push('"');
                out.push_str(&key);
                out.push('"');
                out.push(':');
                expect_key = false;
            }
            _ => {
                expect_key = false;
                out.push(c);
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Brace matching respects strings and escapes.
    #[test]
    fn matches_brace_around_strings() {
        assert_eq!(matching_brace("{\"a\":\"}\"}rest").unwrap(), 9);
        assert_eq!(matching_brace("{\"a\":[1,2]}tail").unwrap(), 11);
        assert!(matching_brace("{\"a\":1").is_err());
    }

    /// Sanitizes unquoted keys and trailing commas, preserving
    /// commas inside strings.
    #[test]
    fn sanitizes_js_object_literal() {
        assert_eq!(
            sanitize_js_object("{\"a\":[\"x\",\"y\",],b:[\"z\"],c:{d:1,}}").unwrap(),
            "{\"a\":[\"x\",\"y\"],\"b\":[\"z\"],\"c\":{\"d\":1}}"
        );
        assert_eq!(sanitize_js_object("{\"a\":\",]\"}").unwrap(), "{\"a\":\",]\"}");
    }

    /// Extracts a keywords map from bundle-shaped source.
    #[test]
    fn extracts_keywords_object() {
        let chunk = "var r={\"academic-cap\":[\"degree\",\"diploma\",],\"adjustments-horizontal\":[\"filter\"],backspace:[\"delete\"],};x()";
        let keywords = extract_keywords(chunk.as_bytes()).unwrap();
        assert_eq!(keywords["academic-cap"], vec!["degree".to_string(), "diploma".to_string()]);
        assert_eq!(keywords["adjustments-horizontal"], vec!["filter".to_string()]);
    }

    /// Resolves relative script srcs against the site origin.
    #[test]
    fn collects_script_urls() {
        let html = "<script src=\"/_next/a.js\"></script><script src=\"https://cdn/x.js\">";
        assert_eq!(
            script_urls(html, "https://heroicons.com"),
            ["https://heroicons.com/_next/a.js".to_string(), "https://cdn/x.js".to_string()]
        );
    }
}
