use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::providers::required_string;
use miette::IntoDiagnostic;
use miette::miette;
use std::collections::BTreeMap;

/// `eva_metadata` provider: parses the upstream `icons-tags.ts`
/// TypeScript object literal (`'name': ['tag', ...],` lines) into a
/// `{name: [tags]}` JSON map with sorted keys, 2-space indentation
/// and a trailing newline — the vendored `icons-tags.json` format.
pub struct EvaMetadata;

impl Provider for EvaMetadata {
    fn name(&self) -> &'static str {
        "eva_metadata"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let source = required_string(context.with, "source", self.name())?;
        let path = context.checkout()?.join(&source);
        let text = std::fs::read_to_string(&path)
            .into_diagnostic()
            .map_err(|e| e.wrap_err(format!("eva_metadata: failed to read {}", path.display())))?;

        let mut tags: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for line in text.lines() {
            if let Some((name, values)) = parse_entry(line)
                && tags.insert(name.clone(), values).is_some()
            {
                return Err(miette!("eva_metadata: duplicate icon '{name}' in {source}"));
            }
        }
        if tags.is_empty() {
            return Err(miette!("eva_metadata: no icon entries found in {source}"));
        }

        let mut content = serde_json::to_string_pretty(&tags)
            .into_diagnostic()
            .map_err(|e| e.wrap_err("eva_metadata: failed to serialize"))?;
        content.push('\n');
        Ok(ProviderOutput::file(content.into_bytes()))
    }
}

/// Parses one `'name': ['tag', 'tag'],` line of the TypeScript object
/// literal. Returns `None` for lines that are not entries (header
/// comments, the `export const` wrapper, closing braces).
fn parse_entry(line: &str) -> Option<(String, Vec<String>)> {
    let line = line.trim().strip_prefix('\'')?;
    let (name, rest) = line.split_once('\'')?;
    let rest = rest.trim().strip_prefix(':')?.trim().strip_prefix('[')?.trim();
    let rest = rest.strip_suffix(',').unwrap_or(rest);
    let inner = rest.strip_suffix(']')?;
    let mut tags = Vec::new();
    let mut rest = inner.trim();
    while let Some(after_quote) = rest.strip_prefix('\'') {
        let Some((tag, tail)) = after_quote.split_once('\'') else {
            break;
        };
        tags.push(tag.to_string());
        rest = tail.trim().strip_prefix(',').unwrap_or(tail).trim();
    }
    Some((name.to_string(), tags))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Entries parse with and without tags; non-entry lines are skipped.
    #[test]
    fn parses_ts_entries() {
        assert_eq!(
            parse_entry("  'activity': ['pulse', 'health'],"),
            Some(("activity".to_string(), vec!["pulse".to_string(), "health".to_string()]))
        );
        assert_eq!(parse_entry("  'arrow-down': [],"), Some(("arrow-down".to_string(), vec![])));
        assert_eq!(
            parse_entry("  'attach': ['paperclip, attachment'],"),
            Some(("attach".to_string(), vec!["paperclip, attachment".to_string()]))
        );
        assert_eq!(parse_entry("export const iconsTags = {"), None);
        assert_eq!(parse_entry("};"), None);
        assert_eq!(parse_entry(" * @license"), None);
    }
}
