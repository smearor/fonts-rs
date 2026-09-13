// build.rs: export Redacted glyphs and generate codepoint maps + constants.
//
// The Redacted font family has two sub-families:
// - Redacted (block bars): single static Regular instance
// - RedactedScript (scribble): Light, Regular, Bold
//
// This build script selects one variant based on the active Cargo feature
// and exports glyphs from the corresponding TTF file.

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::build_config;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::detect_active_variant_index;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::ASCII_PRINTABLE_RANGE;
use fonts_rs_model::FontVariant;

/// All Redacted variants and their mapping to TTF filenames.
const VARIANTS: &[FontVariant] = &[
    FontVariant::file("redacted", "Redacted-Regular"),
    FontVariant::file("script-light", "RedactedScript-Light"),
    FontVariant::file("script-regular", "RedactedScript-Regular"),
    FontVariant::file("script-bold", "RedactedScript-Bold"),
];

fn main() -> miette::Result<()> {
    let idx = detect_active_variant_index(VARIANTS, 0, "Redacted")?;
    let entry = &VARIANTS[idx];
    let font_path = format!("{}/{}.ttf", build_constants::RESOURCES_DIR, entry.font_file().unwrap().as_str());

    eprintln!("build.rs: active variant: {} -> {font_path}", entry);

    set_font_path_env("REDACTED_FONT_PATH", &font_path);

    let config = build_config("redacted", "Redacted", "redacted", Some(*entry), ASCII_PRINTABLE_RANGE);

    FontBuild::new(&font_path)
        .extra_hash(entry.as_str())
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
