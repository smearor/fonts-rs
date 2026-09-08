//! Material Design metadata parser.
//!
//! Reads Google's Material Design icons metadata JSON (from
//! `https://fonts.google.com/metadata/icons`) and builds a mapping
//! from MD icon names to categories and tags.
//! Implements [`IconMetadataSource`] for integration with the generator.

pub mod file;
pub mod icon_entry;
pub mod metadata;

pub use metadata::MdMetadata;

#[cfg(test)]
mod tests {
    use super::super::mapping::{MetadataMapping, RawCategory, RawKeyword};
    use super::super::source::IconMetadataSource;
    use super::*;

    #[test]
    fn md_metadata_parses_json() {
        let json = r#"
{
  "icons": [
    {
      "name": "home",
      "version": 1,
      "categories": ["action"],
      "tags": ["house", "building", "living"]
    },
    {
      "name": "search",
      "version": 1,
      "categories": ["action"],
      "tags": ["find", "magnify"]
    }
  ]
}
"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_md.json");
        std::fs::write(&path, json).unwrap();

        let metadata = MdMetadata::from_file(&path).unwrap();

        assert_eq!(metadata.categories_for("home"), Some(&[RawCategory::new("action".to_string())] as &[RawCategory]));
        let tags = metadata.keywords_for("home").unwrap();
        assert!(tags.contains(&RawKeyword::new("house".to_string())));
        assert!(tags.contains(&RawKeyword::new("building".to_string())));
    }

    #[test]
    fn md_metadata_strips_xssi_prefix() {
        let json = ")]}'\n{\"icons\": [{\"name\": \"star\", \"categories\": [\"toggle\"], \"tags\": [\"favorite\"]}]}";
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_md_xssi.json");
        std::fs::write(&path, json).unwrap();

        let metadata = MdMetadata::from_file(&path).unwrap();
        assert_eq!(metadata.categories_for("star"), Some(&[RawCategory::new("toggle".to_string())] as &[RawCategory]));
    }

    #[test]
    fn md_metadata_prefix() {
        let metadata = MdMetadata {
            mapping: MetadataMapping::new(),
        };
        assert_eq!(metadata.prefix(), "nf-md-");
    }

    #[test]
    fn md_metadata_empty_tags_not_stored() {
        let json = r#"{"icons": [{"name": "foo", "categories": [], "tags": []}]}"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_md_empty.json");
        std::fs::write(&path, json).unwrap();

        let metadata = MdMetadata::from_file(&path).unwrap();
        assert!(metadata.keywords_for("foo").is_none());
        assert!(metadata.categories_for("foo").is_none());
    }
}
