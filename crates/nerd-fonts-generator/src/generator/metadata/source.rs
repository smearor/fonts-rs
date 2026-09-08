//! Unified trait and types for upstream icon metadata sources.
//!
//! Each upstream icon set (Font Awesome, Material Design, Devicon, Octicons)
//! implements [`IconMetadataSource`] to provide keywords and categories
//! for its icons. The generator dispatches to the correct source based
//! on the Nerd Font icon name prefix.

#![allow(dead_code)]

use std::path::Path;

use super::mapping::{MetadataMapping, RawCategory, RawKeyword};
use nerd_fonts_model::IconName;

/// Trait for upstream icon metadata providers.
///
/// Each implementation parses upstream metadata files and provides
/// keyword and category lookups for individual icons by their upstream
/// name (without Nerd Font prefix or `-symbolic` suffix).
pub trait IconMetadataSource {
    /// Returns the Nerd Font prefix this source handles, e.g. `"nf-fa-"`.
    fn prefix(&self) -> &'static str;

    /// Parses metadata from the upstream source file(s).
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the data is malformed.
    fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// Returns the underlying [`MetadataMapping`] holding all mapping data.
    fn mapping(&self) -> &MetadataMapping;

    /// Extracts the upstream icon name from a Nerd Font icon name.
    ///
    /// Strips the Nerd Font prefix and `-symbolic` suffix.
    /// Returns `None` if the name does not match this source's prefix
    /// or is empty after stripping.
    fn extract_name(&self, nf_icon_name: &str) -> Option<String> {
        let name = nf_icon_name.strip_prefix(self.prefix())?;
        let name = name.strip_suffix("-symbolic").unwrap_or(name);
        if name.is_empty() { None } else { Some(name.to_string()) }
    }

    /// Returns keywords for the upstream icon name, if available.
    fn keywords_for(&self, upstream_name: &str) -> Option<&[RawKeyword]> {
        self.mapping().keywords.get(upstream_name)
    }

    /// Returns categories for the upstream icon name, if available.
    fn categories_for(&self, upstream_name: &str) -> Option<&[RawCategory]> {
        self.mapping().categories.get(upstream_name)
    }

    /// Returns the number of icons with keyword data.
    fn keyword_count(&self) -> usize {
        self.mapping().keywords.len()
    }

    /// Returns the number of icons with category data.
    fn category_count(&self) -> usize {
        self.mapping().categories.len()
    }

    /// Returns the number of aliases.
    fn alias_count(&self) -> usize {
        self.mapping().aliases.len()
    }
}

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
    /// # Panics
    ///
    /// Panics if the metadata file cannot be parsed.
    pub fn register<S: IconMetadataSource + 'static>(mut self, path: &Path, label: &str) -> Self {
        let source = S::from_file(path).unwrap_or_else(|e| panic!("Failed to parse {label} metadata: {e}"));
        eprintln!(
            "build.rs: {label}: {} categories, {} keywords, {} aliases",
            source.category_count(),
            source.keyword_count(),
            source.alias_count()
        );
        self.sources.push(Box::new(source));
        self
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
    pub fn all_aliases(&self) -> Vec<(String, String)> {
        let mut entries = Vec::new();
        for source in &self.sources {
            let prefix = source.prefix();
            for (alias, canonical) in source.mapping().aliases.iter() {
                let alias_nf = format!("{}{}-symbolic", prefix, alias.as_str());
                let canonical_nf = format!("{}{}-symbolic", prefix, canonical.as_str());
                entries.push((alias_nf, canonical_nf));
            }
        }
        entries
    }
}

impl Default for IconMetadataRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Strips the XSSI protection prefix `)]}'` from a JSON response.
///
/// Google's Material Design metadata endpoint prepends this prefix
/// to prevent JSON hijacking via cross-site script inclusion.
pub fn strip_xssi_prefix(content: &str) -> &str {
    content.strip_prefix(")]}'").map(|s| s.trim_start()).unwrap_or(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_xssi_prefix_with_prefix() {
        let content = ")]}'\n{\"icons\": []}";
        assert_eq!(strip_xssi_prefix(content), "{\"icons\": []}");
    }

    #[test]
    fn strip_xssi_prefix_without_prefix() {
        let content = "{\"icons\": []}";
        assert_eq!(strip_xssi_prefix(content), "{\"icons\": []}");
    }

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

    #[test]
    fn extract_name_strips_prefix_and_suffix() {
        let source = StubSource {
            prefix: "nf-fa-",
            mapping: MetadataMapping::new(),
        };
        assert_eq!(source.extract_name("nf-fa-gamepad-symbolic"), Some("gamepad".to_string()));
        assert_eq!(source.extract_name("nf-fa-star"), Some("star".to_string()));
        assert_eq!(source.extract_name("nf-md-home"), None);
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

        fn from_file(_path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
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
