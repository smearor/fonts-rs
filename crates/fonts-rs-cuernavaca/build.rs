// build.rs: export Chess Cuernavaca glyphs and generate codepoint maps + constants.
//
// Cuernavaca is a chess piece font where uppercase letters represent
// white pieces and lowercase letters represent black pieces.
// The font covers ASCII letters and a few punctuation marks.

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

use definition::CuernavacaConfig;

/// `MetadataGenerator` deriving piece categories from glyph names.
///
/// Cuernavaca ships no upstream keyword/category source. The letter
/// after the `cuernavaca-` prefix identifies the piece; `d`/`f` are
/// the upside-down rook (rook+knight fairy) and `a`/`s`, `g`/`h` the
/// upside-down bishops and knights. Squares and diagram-border
/// glyphs carry no category. Cuernavaca has no keywords or aliases.
struct CuernavacaMetadataGenerator;

/// Piece category for a single-letter glyph stem, or `None`.
fn piece_category(stem: &str) -> Option<&'static str> {
    Some(match stem {
        "k" | "l" => "king",
        "q" | "w" => "queen",
        "r" | "t" | "d" | "f" => "rook",
        "b" | "v" | "a" | "s" => "bishop",
        "n" | "m" | "g" | "h" => "knight",
        "p" | "o" => "pawn",
        _ => return None,
    })
}

impl MetadataGenerator for CuernavacaMetadataGenerator {
    type Name = String;

    fn keywords_for(&self, _entry: &GlyphEntry<String>) -> Vec<String> {
        Vec::new()
    }

    fn categories_for(&self, entry: &GlyphEntry<String>) -> Vec<String> {
        piece_category(entry.name.strip_prefix("cuernavaca-").unwrap_or_default())
            .map(|category| vec![category.to_string()])
            .unwrap_or_default()
    }
}

fn main() -> miette::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join("ChessCuernavaca.ttf");

    eprintln!("build.rs: exporting glyphs from {}", font_path.display());

    set_font_path_env("CUERNAVACA_FONT_PATH", &font_path)?;

    let config = ExportConfig::<CuernavacaConfig>::new();

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(CuernavacaConfig::GRESOURCE_PREFIX)
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    // Generate metadata (piece categories - Cuernavaca ships no
    // keywords or aliases) if enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating glyph metadata (piece categories)...");

        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        CuernavacaMetadataGenerator
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
