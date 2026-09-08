//! Font Awesome metadata parser.
//!
//! Reads Font Awesome `categories.yml` and `icons.yml` files and builds
//! a mapping from FA icon names to categories and search terms.
//! Implements [`IconMetadataSource`] for integration with the generator.

pub mod categories;
pub mod icon_entry;
pub mod metadata;
pub mod search_term;
pub mod shim;
pub mod shim_replacement;

pub use metadata::FaMetadata;

#[cfg(test)]
mod tests {
    use super::super::mapping::{MetadataMapping, RawCategory, RawKeyword};
    use super::super::source::IconMetadataSource;
    use super::*;

    #[test]
    fn fa_metadata_parses_categories() {
        let categories_yaml = "
accessibility:
  icons:
    - wheelchair
    - blind
  label: Accessibility
arrows:
  icons:
    - arrow-left
    - arrow-right
  label: Arrows
";
        let icons_yaml = "
wheelchair:
  search:
    terms:
      - handicap
      - accessibility
arrow-left:
  search:
    terms: []
";
        let tmp = std::env::temp_dir();
        let dir = tmp.join("test_fa");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("categories.yml"), categories_yaml).unwrap();
        std::fs::write(dir.join("icons.yml"), icons_yaml).unwrap();

        let metadata = FaMetadata::from_file(&dir).unwrap();

        assert_eq!(
            metadata.categories_for("wheelchair"),
            Some(&[RawCategory::new("Accessibility".to_string())] as &[RawCategory])
        );
        assert_eq!(metadata.categories_for("arrow-left"), Some(&[RawCategory::new("Arrows".to_string())] as &[RawCategory]));
        assert!(metadata.categories_for("nonexistent").is_none());
    }

    #[test]
    fn fa_metadata_parses_search_terms() {
        let categories_yaml = "test:\n  icons: [gamepad]\n  label: Test\n";
        let icons_yaml = "gamepad:\n  search:\n    terms:\n      - controller\n      - video game\n";
        let tmp = std::env::temp_dir();
        let dir = tmp.join("test_fa2");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("categories.yml"), categories_yaml).unwrap();
        std::fs::write(dir.join("icons.yml"), icons_yaml).unwrap();

        let metadata = FaMetadata::from_file(&dir).unwrap();

        let terms = metadata.keywords_for("gamepad").unwrap();
        assert!(terms.contains(&RawKeyword::new("controller".to_string())));
        assert!(terms.contains(&RawKeyword::new("video game".to_string())));
    }

    #[test]
    fn fa_metadata_empty_search_terms_not_stored() {
        let categories_yaml = "test:\n  icons: [foo]\n  label: Test\n";
        let icons_yaml = "foo:\n  search:\n    terms: []\n";
        let tmp = std::env::temp_dir();
        let dir = tmp.join("test_fa3");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("categories.yml"), categories_yaml).unwrap();
        std::fs::write(dir.join("icons.yml"), icons_yaml).unwrap();

        let metadata = FaMetadata::from_file(&dir).unwrap();
        assert!(metadata.keywords_for("foo").is_none());
    }

    #[test]
    fn fa_metadata_multi_category_icon() {
        let categories_yaml = "
cat_a:
  icons: [shared-icon]
  label: Category A
cat_b:
  icons: [shared-icon, other-icon]
  label: Category B
";
        let icons_yaml = "shared-icon:\n  search:\n    terms: []\n";
        let tmp = std::env::temp_dir();
        let dir = tmp.join("test_fa4");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("categories.yml"), categories_yaml).unwrap();
        std::fs::write(dir.join("icons.yml"), icons_yaml).unwrap();

        let metadata = FaMetadata::from_file(&dir).unwrap();
        let cats = metadata.categories_for("shared-icon").unwrap();
        assert!(cats.contains(&RawCategory::new("Category A".to_string())));
        assert!(cats.contains(&RawCategory::new("Category B".to_string())));
    }

    #[test]
    fn fa_metadata_prefix() {
        let metadata = FaMetadata {
            mapping: MetadataMapping::new(),
        };
        assert_eq!(metadata.prefix(), "nf-fa-");
    }

    #[test]
    fn fa_metadata_extract_name() {
        let metadata = FaMetadata {
            mapping: MetadataMapping::new(),
        };
        assert_eq!(metadata.extract_name("nf-fa-gamepad-symbolic"), Some("gamepad".to_string()));
        assert_eq!(metadata.extract_name("nf-md-home-symbolic"), None);
    }

    #[test]
    fn fa_metadata_parses_aliases() {
        let categories_yaml = "test:\n  icons: [gear]\n  label: Test\n";
        let icons_yaml = "
gear:
  search:
    terms: []
  aliases:
    - cog
";
        let tmp = std::env::temp_dir();
        let dir = tmp.join("test_fa_aliases");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("categories.yml"), categories_yaml).unwrap();
        std::fs::write(dir.join("icons.yml"), icons_yaml).unwrap();

        let metadata = FaMetadata::from_file(&dir).unwrap();
        assert_eq!(metadata.mapping().aliases.get("cog"), Some("gear"));
    }

    #[test]
    fn fa_metadata_parses_shims() {
        let categories_yaml = "test:\n  icons: [gear]\n  label: Test\n";
        let icons_yaml = "gear:\n  search:\n    terms: []\n";
        let shims_json = r#"[{"name": "cog", "prefix": "fas", "replacement": {"name": "gear", "prefix": "fas"}}]"#;
        let tmp = std::env::temp_dir();
        let dir = tmp.join("test_fa_shims");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("categories.yml"), categories_yaml).unwrap();
        std::fs::write(dir.join("icons.yml"), icons_yaml).unwrap();
        std::fs::write(dir.join("shims.json"), shims_json).unwrap();

        let metadata = FaMetadata::from_file(&dir).unwrap();
        assert_eq!(metadata.mapping().aliases.get("cog"), Some("gear"));
    }
}
