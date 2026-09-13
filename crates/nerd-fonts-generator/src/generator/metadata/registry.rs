//! Aggregation of multiple [`IconMetadataSource`] implementations.
//!
//! The [`IconMetadataRegistry`] dispatches keyword and category lookups
//! to the matching source by Nerd Font icon name prefix.

use std::path::Path;

use super::mapping::RawCategory;
use super::mapping::RawKeyword;
use super::source::IconMetadataSource;
use fonts_rs_generator::GenerateError;
use nerd_fonts_model::IconName;

/// Aggregates multiple [`IconMetadataSource`] implementations.
///
/// The generator queries this registry to resolve keywords and categories
/// for any Nerd Font icon name. Sources are tried in order; the first
/// matching source (by prefix) wins.
pub struct IconMetadataRegistry {
    sources: Vec<Box<dyn IconMetadataSource>>,
}

impl IconMetadataRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }

    /// Adds a metadata source to the registry.
    pub fn with_source(mut self, source: Box<dyn IconMetadataSource>) -> Self {
        self.sources.push(source);
        self
    }

    /// Parses and registers a metadata source from file.
    ///
    /// # Errors
    ///
    /// Returns [`GenerateError::Metadata`] if the metadata file cannot be parsed.
    pub fn register<S: IconMetadataSource + 'static>(mut self, path: &Path, label: &str) -> Result<Self, GenerateError> {
        let source = S::from_file(path).map_err(|e| GenerateError::Metadata {
            label: label.to_string(),
            source: e,
        })?;
        eprintln!(
            "build.rs: {label}: {} categories, {} keywords, {} aliases",
            source.category_count(),
            source.keyword_count(),
            source.alias_count()
        );
        self.sources.push(Box::new(source));
        Ok(self)
    }

    /// Finds the source matching the given Nerd Font icon name prefix.
    fn find_source(&self, nf_icon_name: &IconName) -> Option<&dyn IconMetadataSource> {
        self.sources.iter().find(|s| nf_icon_name.as_ref().starts_with(s.prefix())).map(|s| s.as_ref())
    }

    /// Resolves keywords for a Nerd Font icon name.
    ///
    /// Delegates to the matching source's [`IconMetadataSource::keywords_for`].
    /// Returns `None` if no source matches or the source has no data.
    pub fn keywords_for(&self, nf_icon_name: &IconName) -> Option<&[RawKeyword]> {
        let source = self.find_source(nf_icon_name)?;
        let upstream = source.extract_name(nf_icon_name.as_ref())?;
        source.keywords_for(&upstream)
    }

    /// Resolves categories for a Nerd Font icon name.
    ///
    /// Delegates to the matching source's [`IconMetadataSource::categories_for`].
    /// Returns `None` if no source matches or the source has no data.
    pub fn categories_for(&self, nf_icon_name: &IconName) -> Option<&[RawCategory]> {
        let source = self.find_source(nf_icon_name)?;
        let upstream = source.extract_name(nf_icon_name.as_ref())?;
        source.categories_for(&upstream)
    }

    /// Resolves the canonical NF icon name for an alias.
    ///
    /// Returns the full Nerd Font icon name (e.g. `"nf-fa-gear-symbolic"`)
    /// if the given name is an alias, or `None` if no source matches or
    /// the name is not a known alias.
    pub fn alias_for(&self, nf_icon_name: &IconName) -> Option<IconName> {
        let source = self.find_source(nf_icon_name)?;
        let upstream = source.extract_name(nf_icon_name.as_ref())?;
        let canonical = source.mapping().aliases.get(&upstream)?;
        let suffix = if nf_icon_name.as_ref().ends_with("-symbolic") { "-symbolic" } else { "" };
        IconName::parse(&format!("{}{}{}", source.prefix(), canonical, suffix))
    }

    /// Returns all aliases from all registered sources as (alias_nf_name, canonical_nf_name) pairs.
    ///
    /// Each pair uses full Nerd Font icon names with `-symbolic` suffix.
    /// Names are validated and normalized via [`IconName::parse`].
    /// Duplicate alias keys (after normalization) are deduplicated.
    pub fn all_aliases(&self) -> Vec<(IconName, IconName)> {
        use std::collections::HashMap;

        let mut entries: HashMap<IconName, IconName> = HashMap::new();
        for source in &self.sources {
            let prefix = source.prefix();
            for (alias, canonical) in source.mapping().aliases.iter() {
                let alias_nf = format!("{}{}-symbolic", prefix, alias.as_str());
                let canonical_nf = format!("{}{}-symbolic", prefix, canonical.as_str());
                if let (Some(a), Some(c)) = (IconName::parse(&alias_nf), IconName::parse(&canonical_nf)) {
                    entries.entry(a).or_insert(c);
                }
            }
        }
        let mut result: Vec<(IconName, IconName)> = entries.into_iter().collect();
        result.sort_by(|(a, _), (b, _)| a.cmp(b));
        result
    }
}

impl Default for IconMetadataRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::mapping::MetadataMapping;
    use super::*;

    #[test]
    fn registry_dispatches_by_prefix() {
        let registry = IconMetadataRegistry::new()
            .with_source(Box::new(StubSource {
                prefix: "nf-fa-",
                mapping: MetadataMapping::new(),
            }))
            .with_source(Box::new(StubSource {
                prefix: "nf-md-",
                mapping: MetadataMapping::new(),
            }));

        assert!(registry.keywords_for(&IconName::parse("nf-fa-gamepad-symbolic").unwrap()).is_none());
        assert!(registry.keywords_for(&IconName::parse("nf-md-home-symbolic").unwrap()).is_none());
        assert!(IconName::parse("nf-unknown-icon").is_none());
    }

    /// Minimal stub source for testing registry dispatch.
    struct StubSource {
        prefix: &'static str,
        mapping: MetadataMapping,
    }

    impl IconMetadataSource for StubSource {
        fn prefix(&self) -> &'static str {
            self.prefix
        }

        fn from_file(_path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Self {
                prefix: "",
                mapping: MetadataMapping::new(),
            })
        }

        fn mapping(&self) -> &MetadataMapping {
            &self.mapping
        }
    }
}
