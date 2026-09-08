//! Composite mapping type holding categories, keywords, and aliases.

use super::alias_mapping::AliasMapping;
use super::category_mapping::CategoryMapping;
use super::keyword_mapping::KeywordMapping;

/// Holds all three mapping types for a single icon metadata source.
///
/// Each upstream metadata source (Font Awesome, Material Design, Devicon,
/// Octicons) populates only the fields it has data for; the rest remain empty.
#[derive(Debug, Clone, Default)]
pub struct MetadataMapping {
    /// Maps icon name to category labels.
    pub categories: CategoryMapping,
    /// Maps icon name to keyword tags.
    pub keywords: KeywordMapping,
    /// Maps alias name to canonical icon name.
    pub aliases: AliasMapping,
}

impl MetadataMapping {
    /// Creates an empty `MetadataMapping` with all three mappings initialized.
    pub fn new() -> Self {
        Self {
            categories: CategoryMapping::new(),
            keywords: KeywordMapping::new(),
            aliases: AliasMapping::new(),
        }
    }
}
