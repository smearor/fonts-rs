//! Shared build-time helpers for font crates with variants.
//!
//! Provides common utilities used by `build.rs` scripts across font family
//! crates that support multiple font variants via Cargo features.

use std::path::Path;

use fonts_rs_model::AxisValues;
use fonts_rs_model::CodePointRange;
use fonts_rs_model::FontVariant;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;

use crate::export_config::ExportConfig;

/// Set `cargo:rustc-env` with the absolute font path for `include_bytes!` in lib.rs.
///
/// Constructs the absolute path from `CARGO_MANIFEST_DIR` and the given relative
/// font path, then emits a `cargo:rustc-env` directive so that `include_bytes!`
/// can locate the font file at compile time.
pub fn set_font_path_env(env_var: &str, font_path: &str) {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string());
    let absolute_font_path = Path::new(&crate_dir).join(font_path);
    println!("cargo:rustc-env={env_var}={}", absolute_font_path.display());
}

/// Detect the active variant from Cargo features.
///
/// Scans `CARGO_FEATURE_{NAME}` environment variables for each variant and
/// returns the index of the active one. If no variant is active, returns
/// `default_index`. If more than one is active, returns an error.
pub fn detect_active_variant_index(
    variants: &[FontVariant],
    default_index: usize,
    family_display_name: &str,
) -> miette::Result<usize> {
    let active: Vec<usize> = variants
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_active())
        .map(|(i, _)| i)
        .collect();

    if active.is_empty() {
        eprintln!("build.rs: no {family_display_name} variant feature active, defaulting to {}", variants[default_index].as_str());
        Ok(default_index)
    } else if active.len() > 1 {
        eprintln!("build.rs: expected at most one {family_display_name} variant feature, found {}", active.len());
        for &i in &active {
            eprintln!("  active: {}", variants[i].as_str());
        }
        eprintln!("build.rs: available variants:");
        for v in variants {
            eprintln!("  {}", v.as_str());
        }
        Err(miette::miette!("expected at most one {family_display_name} variant feature, found {}", active.len()))
    } else {
        Ok(active[0])
    }
}

/// Build an [`ExportConfig`] from family info and optional variant.
///
/// Constructs `glyph_name_prefix` and `gresource_prefix` from the family
/// name, GResource subdirectory, and optional variant:
///
/// - With variant: `glyph_name_prefix = "{family}-{variant}"`,
///   `gresource_prefix = "{base}/{subdir}/{variant_slug}"`
/// - Without variant: `glyph_name_prefix = "{family}"`,
///   `gresource_prefix = "{base}/{subdir}"`
///
/// Axis values are taken from the variant's [`FontVariant::axis_values`].
/// For variants without axes, [`AxisValues::EMPTY`] is used.
pub fn build_config(
    family_name: &str,
    family_display_name: &str,
    gresource_subdir: &str,
    variant: Option<FontVariant>,
    codepoint_ranges: &'static [CodePointRange],
) -> ExportConfig {
    let (glyph_name_prefix, gresource_prefix, axes) = match variant {
        Some(variant) => {
            let variant_slug = variant.slug();
            (
                format!("{family_name}-{variant}"),
                format!("{}/{gresource_subdir}/{variant_slug}", GRESOURCE_BASE_PREFIX),
                variant.axis_values(),
            )
        }
        None => (
            family_name.to_string(),
            format!("{}/{gresource_subdir}", GRESOURCE_BASE_PREFIX),
            AxisValues::EMPTY,
        ),
    };

    ExportConfig {
        gresource_prefix,
        icons_context: "glyphs".to_string(),
        glyph_name_prefix,
        codepoint_ranges,
        axes,
        name_filter: None,
        family_display_name: family_display_name.to_string(),
    }
}
