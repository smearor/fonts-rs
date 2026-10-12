// build.rs: Blobmoji hybrid build - pre-rendered SVGs + TTF for pixel-drawing.
//
// The TTF variant is selected by the active Cargo feature (one of four
// variants in the 2x2 color x flags matrix). GResource SVGs are always
// full-color from the pre-rendered `resources/svg/` source, regardless of
// variant. The selected TTF is bundled for `pixel-drawing` (ab_glyph) and
// the GTK font.gresource.
//
// Glyph names come from CLDR annotation data (`annotations.json`,
// vendored by fonts-rs-emoji-metadata and located via
// DEP_EMOJI_METADATA_DIR): the emoji sequence encoded in
// each `emoji_u*.svg` filename is mapped to the CLDR `tts` name
// (kebab-cased). Sequences without a CLDR name keep their raw filename
// stem. Emoji sequences are allocated PUA codepoints by `SvgDirSource`.

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use fonts_rs_generator::CodepointStrategy;
use fonts_rs_generator::EmojiUFilenameParser;
use fonts_rs_generator::ExportConfig;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontVariantExt;
use fonts_rs_generator::GlyphSource;
use fonts_rs_generator::MetadataGenerator;
use fonts_rs_generator::SvgDirSource;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::emoji::ANNOTATIONS_FILE;
use fonts_rs_generator::emoji::EMOJI_TEST_FILE;
use fonts_rs_generator::emoji::EmojiMetadataGenerator;
use fonts_rs_generator::emoji::EmojiOrderingGenerator;
use fonts_rs_generator::emoji::ORDERING_FILE;
use fonts_rs_generator::emoji::parse_cldr_annotations;
use fonts_rs_generator::emoji::parse_emoji_ordering;
use fonts_rs_generator::emoji::parse_emoji_test;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::CodePoint;
use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::FontVariant;
use fonts_rs_model::GlyphEntry;

#[path = "src/definition.rs"]
mod definition;

use definition::BlobmojiConfig;

/// Reads a `DEP_*` path emitted by a resource crate's build script.
fn dep_dir(var: &str) -> miette::Result<PathBuf> {
    std::env::var(var)
        .map(PathBuf::from)
        .map_err(|_| miette::miette!("{var} is not set - is the resource crate declared as a dependency?"))
}

/// Basename of a `metadata/`-prefixed resource constant.
fn file_name(constant: &str) -> &str {
    constant.rsplit('/').next().unwrap_or(constant)
}

/// All Blobmoji variants and their mapping to TTF filenames.
///
/// 2x2 matrix: color (COLRv1 vs bitmap) x flags (with vs without).
/// Default is `color-flags` (index 0): COLRv1 is vector-based and
/// roughly half the size of the CBDT strikes. Note that COLRv1 has
/// empty `glyf` outlines, so `embed_fonts_render` needs a `bitmap-*`
/// feature for colored `ab_glyph` output.
const VARIANTS: VariantList<BlobmojiConfig> = VariantList::new(&[
    FontVariant::file("color-flags", "Noto-COLRv1"),
    FontVariant::file("color-noflags", "Noto-COLRv1-noflags"),
    FontVariant::file("bitmap-flags", "NotoColorEmoji"),
    FontVariant::file("bitmap-noflags", "NotoColorEmoji-noflags"),
]);

/// Variation selector-16 (U+FE0F).
///
/// Stripped from CLDR keys and SVG filename codepoints so that
/// `emoji_u2764.svg` and `emoji_u2764_fe0f.svg` both match the CLDR
/// entry "❤️" (U+2764 U+FE0F).
const VARIATION_SELECTOR_16: char = '\u{FE0F}';

/// Parses CLDR annotations into a codepoint-sequence → kebab-name map.
///
/// Unlike `fonts_rs_generator::emoji::parse_cldr_annotations` - which
/// skips multi-codepoint emoji - this keeps sequences (ZWJ, keycap, flag)
/// because the pre-rendered SVG set covers them. Variation selectors are
/// stripped from the key sequence.
///
/// Accepts both CLDR JSON shapes: the base file wraps the map under
/// `annotations.annotations`, the derived file under
/// `annotationsDerived.annotations`.
fn parse_cldr_sequence_names(json: &str) -> miette::Result<HashMap<Vec<CodePoint>, String>> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| miette::miette!("Failed to parse CLDR annotations: {e}"))?;
    let annotations = value
        .get("annotations")
        .or_else(|| value.get("annotationsDerived"))
        .and_then(|root| root.get("annotations"))
        .and_then(|map| map.as_object())
        .ok_or_else(|| miette::miette!("CLDR annotations map not found"))?;
    let mut names = HashMap::new();
    for (emoji, annotation) in annotations {
        let Some(name) = annotation
            .get("tts")
            .and_then(|tts| tts.as_array())
            .and_then(|tts| tts.first())
            .and_then(|name| name.as_str())
        else {
            continue;
        };
        let Some(kebab) = normalize_to_kebab(name) else { continue };
        let codepoints: Vec<CodePoint> = emoji.chars().filter(|c| *c != VARIATION_SELECTOR_16).map(CodePoint::from).collect();
        if codepoints.is_empty() {
            continue;
        }
        names.insert(codepoints, kebab);
    }
    Ok(names)
}

/// Returns `true` if the codepoint sequence is a flag emoji.
///
/// Covers regional-indicator pairs (e.g. 🇩🇪 = U+1F1E9 U+1F1EA) and
/// subdivision flags, which start with U+1F3F4 (waving black flag)
/// followed by tag characters (e.g. 🏴󠁧󠁢󠁥󠁮󠁧󠁿 England).
fn is_flag_emoji(codepoints: &[CodePoint]) -> bool {
    const REGIONAL_INDICATOR_START: char = '\u{1F1E6}';
    const REGIONAL_INDICATOR_END: char = '\u{1F1FF}';
    const WAVING_BLACK_FLAG: char = '\u{1F3F4}';
    if codepoints.len() < 2 {
        return false;
    }
    if codepoints[0].as_char() == WAVING_BLACK_FLAG {
        return true;
    }
    codepoints
        .iter()
        .all(|cp| (REGIONAL_INDICATOR_START..=REGIONAL_INDICATOR_END).contains(&cp.as_char()))
}

/// FNV-1a hash over the sorted SVG filenames in `svg_dir`.
///
/// Mixed into `FontBuild::extra_hash` so changes to `resources/svg/`
/// force a re-export even when the TTF file is unchanged - the default
/// hash only covers the font file.
fn hash_svg_dir(svg_dir: &Path) -> u64 {
    let mut names: Vec<String> = std::fs::read_dir(svg_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names.iter().fold(0xcbf29ce484222325_u64, |hash, name| {
        name.bytes().fold(hash, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3))
    })
}

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let font_path = entry.font_path()?;

    eprintln!("build.rs: active variant: {} -> {}", entry, font_path.display());

    // Expose the active variant's bitmap capability as a cfg - cargo
    // feature cfgs cannot see the build-time default variant, so tests
    // gate on this instead.
    println!("cargo::rustc-check-cfg=cfg(blobmoji_bitmap)");
    if entry.as_str().starts_with("bitmap") {
        println!("cargo:rustc-cfg=blobmoji_bitmap");
    }

    // Set TTF path for pixel-drawing (ab_glyph) at runtime.
    set_font_path_env("BLOBMOJI_FONT_PATH", &font_path)?;

    // Shared emoji metadata (CLDR/UTS-#51) lives in the
    // fonts-rs-emoji-metadata resource crate.
    let metadata_dir = dep_dir("DEP_EMOJI_METADATA_DIR")?;

    // Parse CLDR annotations into a sequence → name map for the
    // SvgDirSource name resolver. The base file covers single-codepoint
    // emoji; the derived file adds sequences (ZWJ, flags, keycaps).
    let mut sequence_names = HashMap::new();
    for file in ["annotations.json", "annotations-derived.json"] {
        let path = metadata_dir.join(file);
        let json = std::fs::read_to_string(&path).map_err(|e| miette::miette!("Failed to read {}: {e}", path.display()))?;
        sequence_names.extend(parse_cldr_sequence_names(&json)?);
    }
    eprintln!("build.rs: parsed {} CLDR sequence names", sequence_names.len());

    let config = ExportConfig::<BlobmojiConfig>::with_variant(*entry);
    let glyph_name_prefix = config.glyph_name_prefix.clone();
    let svg_dir = Path::new(build_constants::RESOURCES_DIR).join("svg");
    let include_flags = !entry.as_str().contains("noflags");

    // GResource SVGs: copy/rename pre-rendered SVGs from resources/svg/
    // using CLDR semantic names. Full color, original artwork quality.
    // Runs inside FontBuild's export closure so metadata.json and
    // icons.gresource.xml land in OUT_DIR where the GResource compilation
    // and code generation expect them.
    // The resolver closure consumes its own copy; the ordering
    // generator below borrows `sequence_names` again.
    let resolver_names = sequence_names.clone();
    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(BlobmojiConfig::GRESOURCE_PREFIX)
        .extra_hash(format!("{}-{:016x}", entry.as_str(), hash_svg_dir(&svg_dir)))
        .run(|_font_path, resources_dir| {
            SvgDirSource::new(&svg_dir)
                .filename_parser(EmojiUFilenameParser)
                .codepoint_strategy(CodepointStrategy::FromFilename)
                .name_resolver(move |parsed| {
                    parsed.codepoints.as_ref().and_then(|codepoints| {
                        let key: Vec<CodePoint> = codepoints.iter().filter(|cp| cp.as_char() != VARIATION_SELECTOR_16).copied().collect();
                        resolver_names.get(&key).cloned()
                    })
                })
                .name_prefix(&glyph_name_prefix)
                .filter(move |codepoints| include_flags || !is_flag_emoji(codepoints))
                .export(&config, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    // Generate variant info for runtime use.
    config.write_variant_info()?;

    // Generate metadata (keywords + categories + ordering) if enabled.
    // Keywords come from CLDR annotation data, categories from Unicode
    // emoji-test.txt - the same sources as fonts-rs-noto-emoji, reused
    // via EmojiMetadataGenerator. Shortcodes/emoticons from the ordering
    // file become aliases (resolve_alias + search hit them).
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating emoji metadata (keywords/categories/ordering)...");

        let annotations_path = metadata_dir.join(file_name(ANNOTATIONS_FILE));
        let emoji_test_path = metadata_dir.join(file_name(EMOJI_TEST_FILE));

        let annotations_json = std::fs::read_to_string(&annotations_path).map_err(|e| miette::miette!("Failed to read {}: {e}", annotations_path.display()))?;
        let (_name_map, keyword_map, _name_by_codepoint) = parse_cldr_annotations(&annotations_json)?;

        let emoji_test_content = std::fs::read_to_string(&emoji_test_path).map_err(|e| miette::miette!("Failed to read {}: {e}", emoji_test_path.display()))?;
        let category_map = parse_emoji_test(&emoji_test_content);

        // Read the generated metadata.json to get the full list of exported glyphs.
        let metadata_json = std::fs::read_to_string(build_constants::metadata_path()).map_err(|e| miette::miette!("Failed to read metadata.json: {e}"))?;
        let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&metadata_json).map_err(|e| miette::miette!("Failed to parse metadata.json: {e}"))?;

        // Emoji picker ordering from googlefonts/emoji-metadata. The
        // resolver joins each ordered codepoint sequence onto the
        // exported glyph name, mirroring SvgDirSource's CLDR resolver +
        // name_prefix + raw-stem fallback, then drops names that are not
        // exported (unknown upstream emoji or filtered flags).
        let ordering_path = metadata_dir.join(file_name(ORDERING_FILE));
        let ordering_json = std::fs::read_to_string(&ordering_path).map_err(|e| miette::miette!("Failed to read {}: {e}", ordering_path.display()))?;
        let ordering = parse_emoji_ordering(&ordering_json)?;

        let exported_names: HashSet<&str> = entries.iter().map(|entry| entry.name.as_str()).collect();
        let resolve_glyph_name = |sequence: &[CodePoint]| -> Option<String> {
            let key: Vec<CodePoint> = sequence.iter().filter(|cp| cp.as_char() != VARIATION_SELECTOR_16).copied().collect();
            let stem = sequence_names.get(&key).cloned().unwrap_or_else(|| {
                let raw: Vec<String> = key.iter().map(|cp| format!("{:x}", cp.as_char() as u32)).collect();
                format!("emoji-u{}", raw.join("-"))
            });
            let name = format!("{glyph_name_prefix}-{}", normalize_to_kebab(&stem)?);
            exported_names.contains(name.as_str()).then_some(name)
        };

        let alias_pairs = EmojiOrderingGenerator::new(&ordering)
            .run(resolve_glyph_name)
            .map_err(|e| miette::miette!("Failed to generate ordering tables: {e}"))?;

        EmojiMetadataGenerator::new(keyword_map, category_map)
            .with_aliases(alias_pairs)
            .run(&entries)
            .map_err(|e| miette::miette!("Failed to generate metadata: {e}"))?;
    }

    Ok(())
}
