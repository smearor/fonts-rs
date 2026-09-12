// build.rs: export DSEG14 glyphs and generate codepoint maps + constants.
//
// Detects the active Cargo feature (e.g. `classic-regular`, `modern-mini-bold`)
// and exports glyphs from the corresponding DSEG14 TTF variant.

use std::fs;
use std::io::Read;
use std::path::Path;

use fonts_rs_generator::CodemapGenerator;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::GlyphGenerator;
use fonts_rs_generator::RustConstantsGenerator;
use fonts_rs_generator::export_glyphs_with_config;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphEntry;
use miette::IntoDiagnostic;

/// All DSEG14 variants and their mapping to TTF filenames.
const VARIANTS: &[(&str, &str)] = &[
    // Classic
    ("classic-regular",          "DSEG14Classic-Regular"),
    ("classic-bold",             "DSEG14Classic-Bold"),
    ("classic-italic",           "DSEG14Classic-Italic"),
    ("classic-bold-italic",      "DSEG14Classic-BoldItalic"),
    ("classic-light",            "DSEG14Classic-Light"),
    ("classic-light-italic",     "DSEG14Classic-LightItalic"),
    // Classic Mini
    ("classic-mini-regular",     "DSEG14ClassicMini-Regular"),
    ("classic-mini-bold",        "DSEG14ClassicMini-Bold"),
    ("classic-mini-italic",      "DSEG14ClassicMini-Italic"),
    ("classic-mini-bold-italic", "DSEG14ClassicMini-BoldItalic"),
    ("classic-mini-light",       "DSEG14ClassicMini-Light"),
    ("classic-mini-light-italic","DSEG14ClassicMini-LightItalic"),
    // Modern
    ("modern-regular",           "DSEG14Modern-Regular"),
    ("modern-bold",              "DSEG14Modern-Bold"),
    ("modern-italic",            "DSEG14Modern-Italic"),
    ("modern-bold-italic",       "DSEG14Modern-BoldItalic"),
    ("modern-light",             "DSEG14Modern-Light"),
    ("modern-light-italic",      "DSEG14Modern-LightItalic"),
    // Modern Mini
    ("modern-mini-regular",      "DSEG14ModernMini-Regular"),
    ("modern-mini-bold",         "DSEG14ModernMini-Bold"),
    ("modern-mini-italic",       "DSEG14ModernMini-Italic"),
    ("modern-mini-bold-italic",  "DSEG14ModernMini-BoldItalic"),
    ("modern-mini-light",        "DSEG14ModernMini-Light"),
    ("modern-mini-light-italic", "DSEG14ModernMini-LightItalic"),
];

fn main() -> miette::Result<()> {
    println!("cargo:rustc-cfg=is_lib");

    let active = VARIANTS
        .iter()
        .filter(|(feat, _)| std::env::var(format!("CARGO_FEATURE_{}", feat.replace('-', "_").to_uppercase())).is_ok())
        .collect::<Vec<_>>();

    if active.is_empty() {
        eprintln!("build.rs: no DSEG14 variant feature active, defaulting to classic-regular");
    } else if active.len() > 1 {
        eprintln!("build.rs: expected at most one DSEG14 variant feature, found {active_len}", active_len = active.len());
        for (feat, _) in &active {
            eprintln!("  active: {feat}");
        }
        eprintln!("build.rs: available variants:");
        for (feat, _) in VARIANTS {
            eprintln!("  {feat}");
        }
        return Err(miette::miette!("expected at most one DSEG14 variant feature, found {}", active.len()));
    }

    let (feature, font_base) = if active.is_empty() {
        ("classic-regular", "DSEG14Classic-Regular")
    } else {
        *active[0]
    };
    let font_path = format!("resources/{font_base}.ttf", font_base = font_base);
    println!("cargo:rerun-if-changed={font_path}");

    eprintln!("build.rs: active variant: {feature} -> {font_path}");

    // Set env var with absolute path so include_bytes! in fonts.rs can find it.
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string());
    let absolute_font_path = Path::new(&crate_dir).join(&font_path);
    println!("cargo:rustc-env=DSEG14_FONT_PATH={}", absolute_font_path.display());

    let variant_slug = feature.replace('-', "_");
    let glyph_prefix = format!("dseg14-{feature}");
    let gresource_prefix = format!("{}/fourteen_segment/{}", GRESOURCE_BASE_PREFIX, variant_slug);

    let config = ExportConfig {
        gresource_prefix: gresource_prefix.clone(),
        icons_context: "glyphs".to_string(),
        glyph_name_prefix: glyph_prefix.clone(),
        codepoint_ranges: &[(0x20, 0x7E)],
    };

    let metadata_path = Path::new("resources/metadata.json");
    let hash_path = Path::new("resources/.font-hash");
    let current_hash = hash_font_file(&font_path);

    let needs_export = !metadata_path.exists()
        || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

    if needs_export {
        eprintln!("build.rs: exporting glyphs from {font_path}...");
        let count = export_glyphs_with_config(Path::new(&font_path), Path::new("resources"), &config)
            .into_diagnostic()?;
        eprintln!("build.rs: exported {count} glyphs");
        fs::write(hash_path, &current_hash).into_diagnostic()?;
    }

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/icons.gresource.xml",
        "icons.gresource",
    );

    let json = fs::read_to_string("resources/metadata.json").into_diagnostic()?;
    let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&json).into_diagnostic()?;
    CodemapGenerator::run(&entries).into_diagnostic()?;
    RustConstantsGenerator::run(&entries).into_diagnostic()?;

    // Generate variant info (GRESOURCE_PREFIX, glyph prefix) for runtime use.
    let out_dir = std::env::var("OUT_DIR").into_diagnostic()?;
    let variant_info = format!(
        "// @generated by build.rs — do not edit\n\n\
         /// GResource prefix for the active DSEG14 variant.\n\
         pub const GRESOURCE_PREFIX: &str = \"{gresource_prefix}\";\n\n\
         /// Glyph name prefix for the active DSEG14 variant.\n\
         pub const GLYPH_PREFIX: &str = \"{glyph_prefix}\";\n"
    );
    fs::write(Path::new(&out_dir).join("variant.rs"), variant_info).into_diagnostic()?;

    Ok(())
}

fn hash_font_file(path: &str) -> String {
    let mut file = fs::File::open(path).expect("Failed to open font file");
    let mut buffer = [0u8; 8192];
    let mut hash: u64 = 0xcbf29ce484222325;
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        for &byte in &buffer[..n] {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("{hash:016x}")
}
