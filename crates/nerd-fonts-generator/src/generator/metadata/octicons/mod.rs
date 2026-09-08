//! Primer Octicons metadata parser.
//!
//! Reads `keywords.json` from the primer/octicons repository and builds
//! a mapping from Octicon icon names to keywords. Octicons does not use
//! fixed categories; the `keywords` array is used for filtering and search.
//! Implements [`IconMetadataSource`] for integration with the generator.

pub mod metadata;

pub use metadata::OcticonsMetadata;

#[cfg(test)]
mod tests {
    use super::super::mapping::{MetadataMapping, RawKeyword};
    use super::super::source::IconMetadataSource;
    use super::*;

    #[test]
    fn octicons_parses_json() {
        let json = r#"{
    "alert": ["warning", "triangle", "exclamation"],
    "bell": ["notification"],
    "bug": ["insect", "issue"]
}"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_octicons.json");
        std::fs::write(&path, json).unwrap();

        let metadata = OcticonsMetadata::from_file(&path).unwrap();

        let kws = metadata.keywords_for("alert").unwrap();
        assert!(kws.contains(&RawKeyword::new("warning".to_string())));
        assert!(kws.contains(&RawKeyword::new("triangle".to_string())));

        let kws = metadata.keywords_for("bell").unwrap();
        assert!(kws.contains(&RawKeyword::new("notification".to_string())));
    }

    #[test]
    fn octicons_empty_keywords_not_stored() {
        let json = r#"{"foo": []}"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_octicons_empty.json");
        std::fs::write(&path, json).unwrap();

        let metadata = OcticonsMetadata::from_file(&path).unwrap();
        assert!(metadata.keywords_for("foo").is_none());
    }

    #[test]
    fn octicons_no_categories() {
        let metadata = OcticonsMetadata {
            mapping: MetadataMapping::new(),
        };
        assert!(metadata.categories_for("alert").is_none());
    }

    #[test]
    fn octicons_prefix() {
        let metadata = OcticonsMetadata {
            mapping: MetadataMapping::new(),
        };
        assert_eq!(metadata.prefix(), "nf-oct-");
    }
}
