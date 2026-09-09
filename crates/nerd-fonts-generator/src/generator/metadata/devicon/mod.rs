//! Devicon metadata parser.
//!
//! Reads `devicon.json` from the devicons/devicon repository and builds
//! a mapping from Devicon icon names to tags. Devicon does not use
//! fixed categories; instead, the `tags` array groups icons by type
//! (e.g. `"language"`, `"framework"`, `"database"`, `"tool"`).
//! Implements [`IconMetadataSource`] for integration with the generator.

pub mod icon_entry;
pub mod metadata;
pub mod style_alias;

pub use metadata::DeviconMetadata;

#[cfg(test)]
mod tests {
    use super::super::mapping::MetadataMapping;
    use super::super::mapping::RawKeyword;
    use super::super::source::IconMetadataSource;
    use super::*;

    #[test]
    fn devicon_parses_json() {
        let json = r##"[
    {
        "name": "python",
        "altnames": ["py"],
        "tags": ["programming", "language"],
        "versions": {},
        "color": "#3776AB",
        "aliases": []
    },
    {
        "name": "react",
        "altnames": [],
        "tags": ["framework", "javascript"],
        "versions": {},
        "color": "#61DAFB",
        "aliases": []
    }
]"##;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_devicon.json");
        std::fs::write(&path, json).unwrap();

        let metadata = DeviconMetadata::from_file(&path).unwrap();

        let tags = metadata.keywords_for("python").unwrap();
        assert!(tags.contains(&RawKeyword::new("programming".to_string())));
        assert!(tags.contains(&RawKeyword::new("language".to_string())));

        let tags = metadata.keywords_for("react").unwrap();
        assert!(tags.contains(&RawKeyword::new("framework".to_string())));
    }

    #[test]
    fn devicon_resolves_altnames() {
        let json = r#"[{"name": "python", "altnames": ["py"], "tags": ["language"], "versions": {}, "color": "", "aliases": []}]"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_devicon_alt.json");
        std::fs::write(&path, json).unwrap();

        let metadata = DeviconMetadata::from_file(&path).unwrap();
        let tags = metadata.keywords_for("py").unwrap();
        assert!(tags.contains(&RawKeyword::new("language".to_string())));
    }

    #[test]
    fn devicon_no_categories() {
        let metadata = DeviconMetadata {
            mapping: MetadataMapping::new(),
        };
        assert!(metadata.categories_for("python").is_none());
    }

    #[test]
    fn devicon_prefix() {
        let metadata = DeviconMetadata {
            mapping: MetadataMapping::new(),
        };
        assert_eq!(metadata.prefix(), "nf-dev-");
    }

    #[test]
    fn devicon_empty_tags_not_stored() {
        let json = r#"[{"name": "foo", "altnames": [], "tags": [], "versions": {}, "color": "", "aliases": []}]"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_devicon_empty.json");
        std::fs::write(&path, json).unwrap();

        let metadata = DeviconMetadata::from_file(&path).unwrap();
        assert!(metadata.keywords_for("foo").is_none());
    }

    #[test]
    fn devicon_style_aliases() {
        let json = r#"[{"name": "python", "altnames": ["py"], "tags": ["language"], "versions": {}, "color": "", "aliases": [{"base": "plain", "alias": "plain-wordmark"}]}]"#;
        let tmp = std::env::temp_dir();
        let path = tmp.join("test_devicon_style.json");
        std::fs::write(&path, json).unwrap();

        let metadata = DeviconMetadata::from_file(&path).unwrap();
        assert_eq!(metadata.mapping().aliases.get("python-plain-wordmark"), Some("python-plain"));
    }
}
