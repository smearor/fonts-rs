//! Shared build-time helpers for font crates with variants.
//!
//! Provides common utilities used by `build.rs` scripts across font family
//! crates that support multiple font variants via Cargo features.

use std::marker::PhantomData;
use std::path::Path;

use fonts_rs_model::AxisValues;
use fonts_rs_model::FontVariant;

use crate::export_config::ExportConfig;
use crate::font_family_config::FontFamilyConfig;

/// Set `cargo:rustc-env` with the absolute font path for `include_bytes!` in lib.rs.
///
/// Constructs the absolute path from `CARGO_MANIFEST_DIR` and the given relative
/// font path, then emits a `cargo:rustc-env` directive so that `include_bytes!`
/// can locate the font file at compile time.
pub fn set_font_path_env(env_var: &str, font_path: &str) {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string());
    let absolute_font_path = Path::new(&crate_dir).join(font_path);
    println!("cargo:rustc-env={env_var}={}", absolute_font_path.display());
}

/// Build an [`ExportConfig`] from family info and optional variant.
///
/// Constructs `glyph_name_prefix` from `X::GLYPH_NAME_PREFIX` and the optional
/// variant, and `gresource_prefix` from `X::GRESOURCE_PREFIX` and the variant
/// slug:
///
/// - With variant: `glyph_name_prefix = "{X::FONT_FAMILY_NAME}-{variant}"`,
///   `gresource_prefix = "{X::GRESOURCE_PREFIX}/{variant_slug}"`
/// - Without variant: `glyph_name_prefix = "{X::FONT_FAMILY_NAME}"`,
///   `gresource_prefix = "{X::GRESOURCE_PREFIX}"`
///
/// Axis values are taken from the variant's [`FontVariant::axis_values`].
/// For variants without axes, [`AxisValues::EMPTY`] is used.
///
/// The `X` type parameter provides `GRESOURCE_PREFIX`, `FONT_FAMILY_NAME`,
/// `FAMILY_DISPLAY_NAME`, `ICONS_CONTEXT`, and `CODEPOINT_RANGES` via the
/// [`FontFamilyConfig`] trait.
pub fn build_config<X: FontFamilyConfig>(variant: Option<FontVariant>) -> ExportConfig<X> {
    let family_name = X::FONT_FAMILY_NAME;
    let family_display_name = X::FAMILY_DISPLAY_NAME;
    let (glyph_name_prefix, gresource_prefix, axes) = match variant {
        Some(variant) => {
            let variant_slug = variant.slug();
            (format!("{family_name}-{variant}"), format!("{}/{}", X::GRESOURCE_PREFIX, variant_slug), variant.axis_values())
        }
        None => (family_name.to_string(), X::GRESOURCE_PREFIX.to_string(), AxisValues::EMPTY),
    };

    ExportConfig {
        gresource_prefix,
        glyph_name_prefix,
        axes,
        name_filter: None,
        family_display_name: family_display_name.to_string(),
        _marker: PhantomData,
    }
}
