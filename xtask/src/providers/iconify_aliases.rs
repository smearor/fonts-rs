use crate::json::to_json;
use crate::lockfile::LockedInput;
use crate::providers::Provider;
use crate::providers::ProviderContext;
use crate::providers::ProviderOutput;
use crate::providers::required_string;
use crate::providers::required_string_list;
use crate::sha256::sha256_hex;
use miette::IntoDiagnostic;
use miette::miette;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// `iconify_aliases` provider: derives a kebab-case `alias ->
/// canonical` map from an `@iconify-json` set's alias table.
///
/// `with.iconify_set` names the set (e.g. `heroicons`), `with.version`
/// pins the package range (default `1`) and `with.suffixes` lists
/// style suffixes stripped from both alias and parent names
/// (longest first). Aliases for sizes/styles outside `with.suffixes`
/// (e.g. `-16-filled` when only `-24-*` is vendored) are dropped, as
/// are aliases resolving to a hidden or missing icon.
pub struct IconifyAliases;

/// Top-level shape of an iconify `icons.json`.
#[derive(Deserialize)]
struct IconifySet {
    /// Icon table: icon name -> icon record (only `hidden` used).
    #[serde(default)]
    icons: HashMap<String, IconifyIcon>,
    /// Alias table: alias name -> alias record.
    #[serde(default)]
    aliases: HashMap<String, IconifyAlias>,
}

/// Minimal shape of an iconify icon record.
#[derive(Deserialize)]
struct IconifyIcon {
    /// Iconify hides deprecated/duplicate icons from pickers.
    #[serde(default)]
    hidden: bool,
}

/// One iconify alias record.
#[derive(Deserialize)]
struct IconifyAlias {
    /// Canonical icon name the alias resolves to.
    parent: String,
}

impl Provider for IconifyAliases {
    fn name(&self) -> &'static str {
        "iconify_aliases"
    }

    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput> {
        let set = required_string(context.with, "iconify_set", self.name())?;
        let version = match context.with.get("version") {
            Some(toml::Value::String(value)) => value.clone(),
            Some(toml::Value::Integer(value)) => value.to_string(),
            Some(_) => return Err(miette!("iconify_aliases: 'with.version' must be a string or integer")),
            None => "1".to_string(),
        };
        let mut suffixes = required_string_list(context.with, "suffixes", self.name())?;
        suffixes.sort_by_key(|suffix| std::cmp::Reverse(suffix.len()));

        let url = format!("https://unpkg.com/@iconify-json/{set}@{version}/icons.json");
        let body = context.http_get(&url)?;
        let input = LockedInput {
            url,
            sha256: sha256_hex(&body),
            provider: self.name().to_string(),
        };

        let iconify: IconifySet = serde_json::from_slice(&body)
            .into_diagnostic()
            .map_err(|e| e.wrap_err("iconify_aliases: failed to parse icons.json"))?;

        let mut aliases = BTreeMap::new();
        for (alias, record) in &iconify.aliases {
            if !suffixes.iter().any(|suffix| alias.ends_with(suffix.as_str())) && has_other_size_suffix(alias) {
                continue;
            }
            match iconify.icons.get(&record.parent) {
                Some(icon) if icon.hidden => continue,
                None => continue,
                _ => {}
            }
            let alias = strip_suffix(alias, &suffixes);
            let parent = strip_suffix(&record.parent, &suffixes);
            if alias == parent {
                continue;
            }
            match aliases.get(&alias) {
                Some(existing) if existing != &parent => {
                    return Err(miette!("iconify_aliases: alias '{alias}' maps to both '{existing}' and '{parent}'"));
                }
                Some(_) => {}
                None => {
                    aliases.insert(alias, parent);
                }
            }
        }

        let content = to_json(&aliases).map_err(|e| e.wrap_err("iconify_aliases: failed to serialize aliases"))?;
        Ok(ProviderOutput {
            content: content.into_bytes(),
            inputs: vec![input],
        })
    }
}

/// Whether `name` ends with `-<digits>-<word>` — i.e. it belongs to
/// a size variant that is not vendored (e.g. `-16-filled` or
/// `-32-light` when only `-24-*` is selected). Aliases carrying a
/// declared suffix are kept by the caller before this check.
fn has_other_size_suffix(name: &str) -> bool {
    let Some((head, _style)) = name.rsplit_once('-') else {
        return false;
    };
    let Some((_, size)) = head.rsplit_once('-') else {
        return false;
    };
    !size.is_empty() && size.chars().all(|c| c.is_ascii_digit())
}

/// Removes the first matching style suffix from `name`.
fn strip_suffix(name: &str, suffixes: &[String]) -> String {
    for suffix in suffixes {
        if let Some(stripped) = name.strip_suffix(suffix.as_str()) {
            return stripped.to_string();
        }
    }
    name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Detects unvendored `-<size>-<style>` suffixes only.
    #[test]
    fn detects_other_size_suffixes() {
        assert!(has_other_size_suffix("app-span-16-filled"));
        assert!(has_other_size_suffix("arrow-growth-20-regular"));
        assert!(has_other_size_suffix("text-add-32-light"));
        assert!(!has_other_size_suffix("arrow-down-square"));
        assert!(!has_other_size_suffix("fill-24"));
    }

    /// Longest suffix wins; no suffix leaves the name untouched.
    #[test]
    fn strips_longest_suffix() {
        let suffixes = vec!["-16-solid".to_string(), "-20-solid".to_string(), "-solid".to_string()];
        assert_eq!(strip_suffix("code-solid", &suffixes), "code");
        assert_eq!(strip_suffix("viewfinder-dot-20-solid", &suffixes), "viewfinder-dot");
        assert_eq!(strip_suffix("viewfinder-dot-16-solid", &suffixes), "viewfinder-dot");
        assert_eq!(strip_suffix("academic-cap", &suffixes), "academic-cap");
    }
}
