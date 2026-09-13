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

use std::collections::HashMap;
use std::path::Path;

use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::export_glyphs_by_name_map;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use miette::IntoDiagnostic;
use serde::Deserialize;

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "NotoEmoji-Regular.ttf";

/// CLDR annotations file name (relative to `resources/`).
pub const ANNOTATIONS_FILE: &str = "metadata/annotations.json";

/// Unicode emoji-test.txt file name (relative to `resources/`).
pub const EMOJI_TEST_FILE: &str = "metadata/emoji-test.txt";

/// Unicode codepoint ranges for emoji:
/// - BMP: Misc symbols (U+2600–U+26FF), Dingbats (U+2700–U+27BF)
/// - SMP: Supplemental symbols and pictographs (U+1F300–U+1FAFF)
const EMOJI_RANGES: &[(u32, u32)] = &[
    (0x2600, 0x26FF),
    (0x2700, 0x27BF),
    (0x1F300, 0x1F5FF),
    (0x1F600, 0x1F64F),
    (0x1F680, 0x1F6FF),
    (0x1F700, 0x1F77F),
    (0x1F780, 0x1F7FF),
    (0x1F900, 0x1F9FF),
    (0x1FA70, 0x1FAFF),
];

/// CLDR annotation entry: `tts` (name) and `default` (keywords).
///
/// Fields are optional because the JSON also contains an `identity` entry
/// with `language` instead of annotation data.
#[derive(Deserialize)]
struct CldrAnnotation {
    #[serde(rename = "tts", default)]
    tts: Vec<String>,
    #[serde(rename = "default", default)]
    default: Vec<String>,
}

/// CLDR annotations inner structure: contains `identity` and the actual annotations map.
#[derive(Deserialize)]
struct CldrAnnotationsInner {
    #[serde(default)]
    _identity: serde_json::Value,
    annotations: HashMap<String, CldrAnnotation>,
}

/// CLDR annotations root structure.
#[derive(Deserialize)]
struct CldrAnnotations {
    annotations: CldrAnnotationsInner,
}

/// Parse CLDR annotations JSON into (name→codepoint map, keyword map, name map).
///
/// The CLDR JSON uses the actual emoji character as the key.
/// The `tts` field contains the canonical name (e.g. "grinning face").
/// The `default` field contains search keywords (e.g. ["face", "grin"]).
fn parse_cldr_annotations(json: &str) -> miette::Result<(HashMap<String, u32>, HashMap<u32, Vec<String>>, HashMap<u32, String>)> {
    let data: CldrAnnotations = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse CLDR annotations: {e}"))?;

    let mut name_map = HashMap::new();
    let mut keyword_map = HashMap::new();
    let mut name_by_codepoint = HashMap::new();

    for (char_str, annotation) in &data.annotations.annotations {
        // The key is the actual emoji character(s). For single-codepoint emoji,
        // take the first char. For sequences (e.g. skin tone modifiers), skip
        // multi-codepoint entries that aren't in the font.
        let Some(ch) = char_str.chars().next() else { continue };
        if char_str.chars().count() > 1 {
            continue;
        }
        let codepoint = ch as u32;

        // Skip entries without a tts name (e.g. the identity entry)
        if annotation.tts.is_empty() {
            continue;
        }

        // Use the first tts entry as the canonical name
        if let Some(name) = annotation.tts.first() {
            let kebab = name_to_kebab(name);
            if !kebab.is_empty() {
                name_map.insert(kebab.clone(), codepoint);
                name_by_codepoint.insert(codepoint, kebab);
            }
        }

        if !annotation.default.is_empty() {
            keyword_map.insert(codepoint, annotation.default.clone());
        }
    }

    Ok((name_map, keyword_map, name_by_codepoint))
}

/// Convert a CLDR tts name to a kebab-case glyph name.
///
/// e.g. "grinning face" → "grinning-face"
///      "face with tears of joy" → "face-with-tears-of-joy"
fn name_to_kebab(name: &str) -> String {
    let kebab: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect();
    let kebab: String = kebab.split_whitespace().collect::<Vec<_>>().join("-");
    kebab
}

/// Parse emoji-test.txt to build a codepoint→category map.
///
/// The file has `# group: Category Name` and `# subgroup: Sub Name` headers,
/// followed by lines like `1F600 ; fully-qualified # 😀 E1.0 grinning face`.
fn parse_emoji_test(content: &str) -> HashMap<u32, String> {
    let mut categories = HashMap::new();
    let mut current_group = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("# group:") {
            current_group = rest.trim().to_string();
        } else if !trimmed.starts_with('#') && !trimmed.is_empty() {
            // Parse codepoint: "1F600 ; fully-qualified # ..."
            if let Some(codepoint_str) = trimmed.split_whitespace().next() {
                if let Ok(cp) = u32::from_str_radix(codepoint_str, 16) {
                    if !current_group.is_empty() {
                        categories.insert(cp, current_group.clone());
                    }
                }
            }
        }
    }

    categories
}

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
    let (name_map, keyword_map, name_by_codepoint) = parse_cldr_annotations(&annotations_json)?;
    eprintln!("build.rs: parsed {} CLDR annotations", name_map.len());

    // Parse emoji-test.txt for categories
    let emoji_test_content = std::fs::read_to_string(&emoji_test_path).map_err(|e| miette::miette!("Failed to read {emoji_test_path}: {e}"))?;
    let category_map = parse_emoji_test(&emoji_test_content);
    eprintln!("build.rs: parsed {} emoji categories", category_map.len());

    let glyph_prefix = "noto-emoji";
    let gresource_prefix = format!("{}/noto_emoji", GRESOURCE_BASE_PREFIX);

    let config = ExportConfig {
        gresource_prefix: gresource_prefix.clone(),
        icons_context: "emoji".to_string(),
        glyph_name_prefix: glyph_prefix.to_string(),
        codepoint_ranges: EMOJI_RANGES,
        axes: vec![],
        name_filter: None,
    };

    let icons_dir = Path::new(build_constants::RESOURCES_DIR).join("scalable").join("emoji");

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| {
            if icons_dir.exists() {
                std::fs::remove_dir_all(&icons_dir)?;
            }
            export_glyphs_by_name_map(font_path, resources_dir, &config, &name_map)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate variant info for runtime use.
    let out_dir = std::env::var("OUT_DIR").into_diagnostic()?;
    let variant_info = format!(
        "// @generated by build.rs — do not edit\n\n\
         /// GResource prefix for Noto Emoji.\n\
         pub const GRESOURCE_PREFIX: &str = \"{gresource_prefix}\";\n\n\
         /// Glyph name prefix for Noto Emoji.\n\
         pub const GLYPH_PREFIX: &str = \"{glyph_prefix}\";\n"
    );
    std::fs::write(Path::new(&out_dir).join("variant.rs"), variant_info).into_diagnostic()?;

    // Generate metadata (keywords + categories) if feature is enabled.
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating emoji metadata (keywords/categories)...");

        // Read the generated metadata.json to get the full list of exported glyphs.
        let metadata_json = std::fs::read_to_string(build_constants::METADATA_PATH).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<fonts_rs_model::GlyphEntry<String>> =
            serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        generate_metadata(&entries, &name_by_codepoint, &keyword_map, &category_map, &out_dir)?;
    }

    Ok(())
}

/// Generate phf::Map metadata for keywords and categories.
fn generate_metadata(
    entries: &[fonts_rs_model::GlyphEntry<String>],
    _name_by_codepoint: &HashMap<u32, String>,
    keyword_map: &HashMap<u32, Vec<String>>,
    category_map: &HashMap<u32, String>,
    out_dir: &str,
) -> miette::Result<()> {
    use std::io::Write;

    let mut keywords_code = String::new();
    let mut categories_code = String::new();

    keywords_code.push_str("// @generated by build.rs — do not edit\n\n");
    keywords_code.push_str("/// Keywords for each emoji glyph name.\n");
    keywords_code.push_str("pub static KEYWORDS: GlyphKeywordMap = GlyphKeywordMap(phf::phf_map! {\n");

    categories_code.push_str("// @generated by build.rs — do not edit\n\n");
    categories_code.push_str("/// Categories for each emoji glyph name.\n");
    categories_code.push_str("pub static CATEGORIES: GlyphCategoryMap = GlyphCategoryMap(phf::phf_map! {\n");

    for entry in entries {
        let glyph_name = &entry.name;
        let Some(code) = &entry.code else { continue };
        let cp = code.as_char() as u32;

        // Keywords
        if let Some(kws) = keyword_map.get(&cp) {
            if !kws.is_empty() {
                let kw_refs: Vec<String> = kws.iter().map(|k| format!("GlyphKeyword::new(\"{k}\")")).collect();
                keywords_code.push_str(&format!("    \"{glyph_name}\" => &[{}],\n", kw_refs.join(", ")));
            }
        }

        // Category (single category wrapped in a slice for unified API)
        if let Some(cat) = category_map.get(&cp) {
            categories_code.push_str(&format!("    \"{glyph_name}\" => &[GlyphCategory::new(\"{cat}\")],\n"));
        }
    }

    keywords_code.push_str("});\n");
    categories_code.push_str("});\n");

    let mut kw_file = std::fs::File::create(Path::new(out_dir).join("keywords.rs")).into_diagnostic()?;
    kw_file.write_all(keywords_code.as_bytes()).into_diagnostic()?;

    let mut cat_file = std::fs::File::create(Path::new(out_dir).join("categories.rs")).into_diagnostic()?;
    cat_file.write_all(categories_code.as_bytes()).into_diagnostic()?;

    Ok(())
}
