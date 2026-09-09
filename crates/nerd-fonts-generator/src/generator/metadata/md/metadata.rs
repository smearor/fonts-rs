//! Parsed Material Design metadata: icon name to categories and tags.

use std::path::Path;

use super::super::mapping::MetadataMapping;
use super::super::source::IconMetadataSource;
use super::super::source::strip_xssi_prefix;
use super::file::MdMetadataFile;

/// Parsed Material Design metadata: icon name to categories and tags.
#[derive(Debug, Clone)]
pub struct MdMetadata {
    /// Underlying mapping holding categories, keywords, and aliases.
    pub(crate) mapping: MetadataMapping,
}

impl IconMetadataSource for MdMetadata {
    fn prefix(&self) -> &'static str {
        "nf-md-"
    }

    fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(path)?;
        let stripped = strip_xssi_prefix(&content);
        let metadata: MdMetadataFile = serde_json::from_str(stripped)?;

        let mut mapping = MetadataMapping::new();
        for entry in metadata.icons {
            if !entry.categories.is_empty() {
                mapping.categories.insert(entry.name.clone(), entry.categories);
            }
            if !entry.tags.is_empty() {
                mapping.keywords.insert(entry.name.clone(), entry.tags);
            }
        }

        Ok(Self { mapping })
    }

    fn mapping(&self) -> &MetadataMapping {
        &self.mapping
    }
}
