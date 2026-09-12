// build.rs: export DSEG7 glyphs and generate codepoint maps + constants.
//
// Detects the active Cargo feature (e.g. `classic-regular`, `modern-mini-bold`)
// and exports glyphs from the corresponding DSEG7 TTF variant.

use std::fs;
use std::path::Path;

use fonts_rs_generator::CodemapGenerator;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::GlyphGenerator;
use fonts_rs_generator::RustConstantsGenerator;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::export_glyphs_with_config;
use fonts_rs_generator::hash_font_file;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphEntry;
use miette::IntoDiagnostic;

/// All DSEG7 variants and their mapping to TTF filenames.
const VARIANTS: &[(&str, &str)] = &[
    // Classic
    ("classic-regular",          "DSEG7Classic-Regular"),
    ("classic-bold",             "DSEG7Classic-Bold"),
    ("classic-italic",           "DSEG7Classic-Italic"),
    ("classic-bold-italic",      "DSEG7Classic-BoldItalic"),
    ("classic-light",            "DSEG7Classic-Light"),
    ("classic-light-italic",     "DSEG7Classic-LightItalic"),
    // Classic Mini
    ("classic-mini-regular",     "DSEG7ClassicMini-Regular"),
    ("classic-mini-bold",        "DSEG7ClassicMini-Bold"),
    ("classic-mini-italic",      "DSEG7ClassicMini-Italic"),
    ("classic-mini-bold-italic", "DSEG7ClassicMini-BoldItalic"),
    ("classic-mini-light",       "DSEG7ClassicMini-Light"),
    ("classic-mini-light-italic","DSEG7ClassicMini-LightItalic"),
    // Modern
    ("modern-regular",           "DSEG7Modern-Regular"),
    ("modern-bold",              "DSEG7Modern-Bold"),
    ("modern-italic",            "DSEG7Modern-Italic"),
    ("modern-bold-italic",       "DSEG7Modern-BoldItalic"),
    ("modern-light",             "DSEG7Modern-Light"),
    ("modern-light-italic",      "DSEG7Modern-LightItalic"),
    // Modern Mini
    ("modern-mini-regular",      "DSEG7ModernMini-Regular"),
    ("modern-mini-bold",         "DSEG7ModernMini-Bold"),
    ("modern-mini-italic",       "DSEG7ModernMini-Italic"),
    ("modern-mini-bold-italic",  "DSEG7ModernMini-BoldItalic"),
    ("modern-mini-light",        "DSEG7ModernMini-Light"),
    ("modern-mini-light-italic", "DSEG7ModernMini-LightItalic"),
];

fn main() -> miette::Result<()> {
    println!("cargo:rustc-cfg=is_lib");

    let active = VARIANTS
        .iter()
        .filter(|(feat, _)| std::env::var(format!("CARGO_FEATURE_{}", feat.replace('-', "_").to_uppercase())).is_ok())
        .collect::<Vec<_>>();

    if active.is_empty() {
        eprintln!("build.rs: no DSEG7 variant feature active, defaulting to classic-regular");
    } else if active.len() > 1 {
        eprintln!("build.rs: expected at most one DSEG7 variant feature, found {active_len}", active_len = active.len());
        for (feat, _) in &active {
            eprintln!("  active: {feat}");
        }
        eprintln!("build.rs: available variants:");
        for (feat, _) in VARIANTS {
            eprintln!("  {feat}");
        }
        return Err(miette::miette!("expected at most one DSEG7 variant feature, found {}", active.len()));
    }

    let (feature, font_base) = if active.is_empty() {
        ("classic-regular", "DSEG7Classic-Regular")
    } else {
        *active[0]
    };
    let font_path = format!("{}/{font_base}.ttf", build_constants::RESOURCES_DIR, font_base = font_base);
    println!("cargo:rerun-if-changed={font_path}");

    eprintln!("build.rs: active variant: {feature} -> {font_path}");

    // Set env var with absolute path so include_bytes! in fonts.rs can find it.
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string());
    let absolute_font_path = Path::new(&crate_dir).join(&font_path);
    println!("cargo:rustc-env=DSEG7_FONT_PATH={}", absolute_font_path.display());

    let variant_slug = feature.replace('-', "_");
    let glyph_prefix = format!("dseg7-{feature}");
    let gresource_prefix = format!("{}/seven_segment/{}", GRESOURCE_BASE_PREFIX, variant_slug);

    let config = ExportConfig {
        gresource_prefix: gresource_prefix.clone(),
        icons_context: "glyphs".to_string(),
        glyph_name_prefix: glyph_prefix.clone(),
        codepoint_ranges: &[(0x20, 0x7E)],
    };

    let metadata_path = Path::new(build_constants::METADATA_PATH);
    let hash_path = Path::new(build_constants::HASH_PATH);
    let current_hash = hash_font_file(&font_path);

    let needs_export = !metadata_path.exists()
        || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

    if needs_export {
        eprintln!("build.rs: exporting glyphs from {font_path}...");
        let count = export_glyphs_with_config(Path::new(&font_path), Path::new(build_constants::RESOURCES_DIR), &config)
            .into_diagnostic()?;
        eprintln!("build.rs: exported {count} glyphs");
        fs::write(hash_path, &current_hash).into_diagnostic()?;
    }

    glib_build_tools::compile_resources(
        &[build_constants::RESOURCES_DIR],
        build_constants::ICONS_GRESOURCE_XML,
        build_constants::ICONS_GRESOURCE,
    );

    let json = fs::read_to_string(build_constants::METADATA_PATH).into_diagnostic()?;
    let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&json).into_diagnostic()?;
    CodemapGenerator::run(&entries).into_diagnostic()?;
    RustConstantsGenerator::run(&entries).into_diagnostic()?;

    // Generate variant info (GRESOURCE_PREFIX, glyph prefix) for runtime use.
    let out_dir = std::env::var("OUT_DIR").into_diagnostic()?;
    let variant_info = format!(
        "// @generated by build.rs — do not edit\n\n\
         /// GResource prefix for the active DSEG7 variant.\n\
         pub const GRESOURCE_PREFIX: &str = \"{gresource_prefix}\";\n\n\
         /// Glyph name prefix for the active DSEG7 variant.\n\
         pub const GLYPH_PREFIX: &str = \"{glyph_prefix}\";\n"
    );
    fs::write(Path::new(&out_dir).join("variant.rs"), variant_info).into_diagnostic()?;

    Ok(())
}
