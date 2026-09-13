// build.rs: export Noto Emoji glyphs using CLDR annotation data.
//
// NotoEmoji-Regular.ttf is a monochrome TrueType font covering all
// Unicode emoji codepoints. Glyph names in the font are `uniXXXX` format
// (PostScript auto-generated), so we use CLDR annotation data to drive
// the export with semantic names instead.
//
// Data sources:
// - metadata/annotations.json: CLDR English annotations (tts name + keywords)
// - metadata/emoji-test.txt: Unicode emoji-test.txt (group/subgroup categories)

use std::path::Path;

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::build_config;
use fonts_rs_generator::build_constants;
use fonts_rs_noto_emoji_generator::ANNOTATIONS_FILE;
use fonts_rs_noto_emoji_generator::EMOJI_TEST_FILE;
use fonts_rs_noto_emoji_generator::FONT_FILE;
use fonts_rs_noto_emoji_generator::GLYPH_PREFIX;
use fonts_rs_noto_emoji_generator::NotoEmojiDefinition;
use fonts_rs_noto_emoji_generator::NotoEmojiMetadataGenerator;
use fonts_rs_noto_emoji_generator::parse_cldr_annotations;
use fonts_rs_noto_emoji_generator::parse_emoji_test;

fn main() -> miette::Result<()> {
    let font_path = format!("{}/{}", build_constants::RESOURCES_DIR, FONT_FILE);
    let annotations_path = format!("{}/{}", build_constants::RESOURCES_DIR, ANNOTATIONS_FILE);
    let emoji_test_path = format!("{}/{}", build_constants::RESOURCES_DIR, EMOJI_TEST_FILE);

    eprintln!("build.rs: exporting glyphs from {font_path}");

    // Set env var with absolute path so include_bytes! in fonts.rs can find it.
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string());
    let absolute_font_path = Path::new(&crate_dir).join(&font_path);
    println!("cargo:rustc-env=NOTO_EMOJI_FONT_PATH={}", absolute_font_path.display());

    // Parse CLDR annotations
    let annotations_json = std::fs::read_to_string(&annotations_path).map_err(|e| miette::miette!("Failed to read {annotations_path}: {e}"))?;
    let (name_map, keyword_map, _name_by_codepoint) = parse_cldr_annotations(&annotations_json)?;
    eprintln!("build.rs: parsed {} CLDR annotations", name_map.len());

    // Parse emoji-test.txt for categories
    let emoji_test_content = std::fs::read_to_string(&emoji_test_path).map_err(|e| miette::miette!("Failed to read {emoji_test_path}: {e}"))?;
    let category_map = parse_emoji_test(&emoji_test_content);
    eprintln!("build.rs: parsed {} emoji categories", category_map.len());

    let config = build_config::<NotoEmojiDefinition>(GLYPH_PREFIX, "Noto Emoji", None);

    let icons_dir = Path::new(build_constants::RESOURCES_DIR).join("scalable").join("emoji");

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| {
            if icons_dir.exists() {
                std::fs::remove_dir_all(&icons_dir)?;
            }
            config.export_glyphs_by_name_map(font_path, resources_dir, &name_map)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate variant info for runtime use.
    config.write_variant_info()?;

    // Generate metadata (keywords + categories) if feature is enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating emoji metadata (keywords/categories)...");

        // Read the generated metadata.json to get the full list of exported glyphs.
        let metadata_json = std::fs::read_to_string(build_constants::METADATA_PATH).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<fonts_rs_model::GlyphEntry<String>> =
            serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        NotoEmojiMetadataGenerator::new(keyword_map, category_map)
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
