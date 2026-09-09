//! Parsed Font Awesome metadata: icon name to categories, search terms, and aliases.

use std::collections::HashMap;
use std::path::Path;

use super::super::mapping::MetadataMapping;
use super::super::mapping::RawAlias;
use super::super::source::IconMetadataSource;
use super::categories::FaCategories;
use super::icon_entry::FaIconEntry;
use super::shim::FaShim;

/// Parsed Font Awesome metadata: icon name to categories, search terms, and aliases.
#[derive(Debug, Clone)]
pub struct FaMetadata {
    /// Underlying mapping holding categories, keywords, and aliases.
    pub(crate) mapping: MetadataMapping,
}

impl IconMetadataSource for FaMetadata {
    fn prefix(&self) -> &'static str {
        "nf-fa-"
    }

    fn from_file(dir: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let categories_path = dir.join("categories.yml");
        let icons_path = dir.join("icons.yml");

        let categories_yaml = std::fs::read_to_string(&categories_path)?;
        let icons_yaml = std::fs::read_to_string(&icons_path)?;

        let categories: HashMap<String, FaCategories> = serde_yaml::from_str(&categories_yaml)?;
        let icons: HashMap<String, FaIconEntry> = serde_yaml::from_str(&icons_yaml)?;

        let mut mapping = MetadataMapping::new();
        for cat in categories.values() {
            for icon_name in &cat.icons {
                mapping.categories.add(icon_name, cat.label.clone());
            }
        }
        for (icon_name, entry) in &icons {
            if !entry.search.terms.is_empty() {
                for term in &entry.search.terms {
                    mapping.keywords.add(icon_name, term.clone());
                }
            }
            for alias in &entry.aliases {
                mapping.aliases.insert(alias.clone(), RawAlias::new(icon_name.clone()));
            }
        }

        // Optionally parse shims.json for v4/v5 compatibility aliases.
        let shims_path = dir.join("shims.json");
        if shims_path.exists() {
            let shims_json = std::fs::read_to_string(&shims_path)?;
            let shims: Vec<FaShim> = serde_json::from_str(&shims_json)?;
            for shim in shims {
                if let Some(replacement) = shim.replacement {
                    mapping.aliases.insert(RawAlias::new(shim.name), RawAlias::new(replacement.name));
                }
            }
        }

        Ok(Self { mapping })
    }

    fn mapping(&self) -> &MetadataMapping {
        &self.mapping
    }
}
