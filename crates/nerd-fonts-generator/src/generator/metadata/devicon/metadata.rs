//! Parsed Devicon metadata: icon name to tags and altnames.

use std::path::Path;

use super::super::mapping::{MetadataMapping, RawAlias, RawKeyword};
use super::super::source::IconMetadataSource;
use super::icon_entry::DeviconEntry;

/// Parsed Devicon metadata: icon name to tags and altnames.
#[derive(Debug, Clone)]
pub struct DeviconMetadata {
    /// Underlying mapping holding categories, keywords, and aliases.
    pub(crate) mapping: MetadataMapping,
}

impl IconMetadataSource for DeviconMetadata {
    fn prefix(&self) -> &'static str {
        "nf-dev-"
    }

    fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let entries: Vec<DeviconEntry> = serde_json::from_str(&content)?;

        let mut mapping = MetadataMapping::new();
        for entry in entries {
            if !entry.tags.is_empty() {
                for tag in &entry.tags {
                    mapping.keywords.add(&entry.name, tag.clone());
                }
            }
            for altname in &entry.altnames {
                mapping.aliases.insert(altname.clone(), RawAlias::new(entry.name.clone()));
            }
            for style_alias in &entry.aliases {
                let alias_name = RawAlias::new(format!("{}-{}", entry.name, style_alias.alias));
                let canonical_name = RawAlias::new(format!("{}-{}", entry.name, style_alias.base));
                mapping.aliases.insert(alias_name, canonical_name);
            }
        }

        Ok(Self { mapping })
    }

    fn mapping(&self) -> &MetadataMapping {
        &self.mapping
    }

    fn keywords_for(&self, upstream_name: &str) -> Option<&[RawKeyword]> {
        // Try direct lookup first, then resolve via altname.
        if let Some(tags) = self.mapping.keywords.get(upstream_name) {
            return Some(tags);
        }
        if let Some(canonical) = self.mapping.aliases.get(upstream_name) {
            return self.mapping.keywords.get(canonical);
        }
        None
    }
}
