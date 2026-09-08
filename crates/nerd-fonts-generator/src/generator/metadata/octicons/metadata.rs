//! Parsed Octicons metadata: icon name to keywords.

use std::path::Path;

use super::super::mapping::{MetadataMapping, RawKeyword};
use super::super::source::IconMetadataSource;

/// Parsed Octicons metadata: icon name to keywords.
#[derive(Debug, Clone)]
pub struct OcticonsMetadata {
    /// Underlying mapping holding categories, keywords, and aliases.
    pub(crate) mapping: MetadataMapping,
}

impl IconMetadataSource for OcticonsMetadata {
    fn prefix(&self) -> &'static str {
        "nf-oct-"
    }

    fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let keywords: std::collections::HashMap<String, Vec<RawKeyword>> = serde_json::from_str(&content)?;

        let mut mapping = MetadataMapping::new();
        for (icon_name, terms) in keywords {
            if !terms.is_empty() {
                mapping.keywords.insert(icon_name, terms);
            }
        }

        Ok(Self { mapping })
    }

    fn mapping(&self) -> &MetadataMapping {
        &self.mapping
    }
}
