// build.rs: export Dicefont glyphs and generate codepoint maps + constants.
//
// Dicefont is a static icon font with die face glyphs for D2, D4, D6, D8,
// D10, D12, D20, and dot-d6 variants. All glyphs are in the Private Use
// Area (U+F100–U+F19B).

use std::path::Path;

use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GlyphEntry;

#[path = "src/definition.rs"]
mod definition;

use definition::DicefontConfig;

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "dicefont.ttf";

/// Die-type categories: the standard dice plus d2, d8, and the
/// Fudge dice (whose `f` name segment maps to the "fudge" label).
const DICE_CATEGORIES: [&str; 8] = ["d2", "d4", "d6", "d8", "d10", "d12", "d20", "f"];

/// `MetadataGenerator` deriving die-type categories from glyph names.
///
/// Dicefont ships no upstream keyword/category source, so the die
/// type appears as its own '-'-separated segment in the glyph name:
/// `dicefont-d20-4` -> "d20", `dicefont-dot-d6-2` -> "d6" (the dotted
/// variants stay d6 dice), `dicefont-f-minus` -> "fudge". Segment
/// matching avoids "d2" hitting inside "d20". Dicefont has no
/// keywords or aliases.
struct DicefontMetadataGenerator;

impl MetadataGenerator for DicefontMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, _entry: &GlyphEntry<String>) -> Vec<String> {
        Vec::new()
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        entry
            .name
            .strip_prefix("dicefont-")
            .unwrap_or_default()
            .split('-')
            .find(|segment| DICE_CATEGORIES.contains(segment))
            .map(|die| vec![if die == "f" { "fudge".to_string() } else { die.to_string() }])
            .unwrap_or_default()
    }
}

fn main() -> miette::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(FONT_FILE);

    eprintln!("build.rs: exporting glyphs from {}", font_path.display());

    set_font_path_env("DICEFONT_FONT_PATH", &font_path)?;

    let config = ExportConfig::<DicefontConfig>::new();

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(DicefontConfig::GRESOURCE_PREFIX)
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    // Generate metadata (die-type categories - Dicefont ships no
    // keywords or aliases) if enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating glyph metadata (die-type categories)...");

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        DicefontMetadataGenerator
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
