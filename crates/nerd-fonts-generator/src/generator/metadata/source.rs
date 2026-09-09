//! Unified trait and types for upstream icon metadata sources.
//!
//! Each upstream icon set (Font Awesome, Material Design, Devicon, Octicons)
//! implements [`IconMetadataSource`] to provide keywords and categories
//! for its icons. The generator dispatches to the correct source based
//! on the Nerd Font icon name prefix.

#![allow(dead_code)]

use std::path::Path;

use super::mapping::MetadataMapping;
use super::mapping::RawCategory;
use super::mapping::RawKeyword;

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
    fn extract_name_strips_prefix_and_suffix() {
        let source = StubSource {
            prefix: "nf-fa-",
            mapping: MetadataMapping::new(),
        };
        assert_eq!(source.extract_name("nf-fa-gamepad-symbolic"), Some("gamepad".to_string()));
        assert_eq!(source.extract_name("nf-fa-star"), Some("star".to_string()));
        assert_eq!(source.extract_name("nf-md-home"), None);
    }

    /// Minimal stub source for testing.
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
