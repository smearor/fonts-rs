// build.rs: export Redacted glyphs and generate codepoint maps + constants.
//
// The Redacted font family has two sub-families:
// - Redacted (block bars): single static Regular instance
// - RedactedScript (scribble): Light, Regular, Bold
//
// This build script selects one variant based on the active Cargo feature
// and exports glyphs from the corresponding TTF file.

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontVariantExt;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_config;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::FontVariant;

#[path = "src/definition.rs"]
mod definition;

use definition::RedactedConfig;

/// All Redacted variants and their mapping to TTF filenames.
const VARIANTS: VariantList<RedactedConfig> = VariantList::new(&[
    FontVariant::file("redacted", "Redacted-Regular"),
    FontVariant::file("script-light", "RedactedScript-Light"),
    FontVariant::file("script-regular", "RedactedScript-Regular"),
    FontVariant::file("script-bold", "RedactedScript-Bold"),
]);

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let font_path = entry.font_path()?;

    eprintln!("build.rs: active variant: {} -> {}", entry, font_path.display());

    set_font_path_env("REDACTED_FONT_PATH", &font_path)?;

    let config = build_config::<RedactedConfig>(Some(*entry));

    FontBuild::new(&font_path)
        .extra_hash(entry.as_str())
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
