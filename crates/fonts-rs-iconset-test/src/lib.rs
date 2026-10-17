//! End-to-end fixture for custom icon sets.
//!
//! The build script runs `IconSetBuild` on `icons.toml`: two crate
//! sources (Lucide + Tabler/filled) resolved via `DEP_*_SVG_DIR`, a
//! local `dir` source, renames in every source, per-source
//! post-processors. `src/icons.rs` is the generated `iconset.rs`
//! module verbatim - exactly what an application would write.

pub mod icons {
    include!(concat!(env!("OUT_DIR"), "/iconset.rs"));
}

pub use icons::FixtureIconName;
pub use icons::FixtureIcons;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitelist_size_matches_glyph_count() {
        assert_eq!(FixtureIcons::all_glyphs().count(), 7);
    }

    #[test]
    fn renamed_icons_use_exported_names() {
        let names: Vec<String> = FixtureIcons::all_glyphs().map(|(_, name)| name.as_ref().to_string()).collect();
        assert!(names.contains(&"fixture-install-symbolic".to_string()));
        assert!(names.contains(&"fixture-check-filled-symbolic".to_string()));
        assert!(names.contains(&"fixture-brand-symbolic".to_string()));
        assert!(!names.contains(&"fixture-download-symbolic".to_string()));
        assert!(!names.contains(&"fixture-old-brand-symbolic".to_string()));
    }

    #[test]
    fn dir_source_icons_export() {
        let names: Vec<String> = FixtureIcons::all_glyphs().map(|(_, name)| name.as_ref().to_string()).collect();
        assert!(names.contains(&"fixture-custom-arrow-symbolic".to_string()));
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn glyphs_register_and_resolve_via_gresource() {
        use fonts_rs_model::GtkFont;

        FixtureIcons::register_glyphs().expect("register icons.gresource");

        for icon_name in [
            "fixture-check-symbolic",
            "fixture-install-symbolic",
            "fixture-trash-symbolic",
            "fixture-check-filled-symbolic",
            "fixture-heart-symbolic",
            "fixture-custom-arrow-symbolic",
            "fixture-brand-symbolic",
        ] {
            gio::resources_lookup_data(&format!("/com/example/iconset-test/icons/scalable/actions/{icon_name}.svg"), gio::ResourceLookupFlags::NONE)
                .unwrap_or_else(|_| panic!("{icon_name}.svg must be in the gresource"));
        }

        // Renamed-away and unwhitelisted icons must be absent.
        for absent in ["fixture-download-symbolic", "fixture-old-brand-symbolic", "fixture-house-symbolic"] {
            assert!(
                gio::resources_lookup_data(&format!("/com/example/iconset-test/icons/scalable/actions/{absent}.svg"), gio::ResourceLookupFlags::NONE,).is_err(),
                "{absent} must not be exported"
            );
        }
    }

    #[test]
    fn icon_name_constants_exist() {
        assert_eq!(icons::icon_names::FIXTURE_CHECK_SYMBOLIC, "fixture-check-symbolic");
        assert_eq!(icons::icon_names::FIXTURE_INSTALL_SYMBOLIC, "fixture-install-symbolic");
        assert_eq!(icons::icon_names::FIXTURE_BRAND_SYMBOLIC, "fixture-brand-symbolic");
    }
}
