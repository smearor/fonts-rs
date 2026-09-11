// build.rs: export DSEG7 glyphs and generate codepoint maps + constants.
//
// The `SevenSegment` marker and `SevenSegmentDefinition` are duplicated here
// because build.rs is a separate compilation unit that cannot import from
// the crate's lib.rs. The definitions are intentionally identical.

use std::fs;
use std::io::Read;
use std::path::Path;

use const_format::concatcp;
use fonts_rs_generator::CodemapGenerator;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::GlyphGenerator;
use fonts_rs_generator::RustConstantsGenerator;
use fonts_rs_generator::normalize_to_kebab;
use fonts_rs_model::FontFamily;
use fonts_rs_model::GRESOURCE_BASE_PREFIX;
use fonts_rs_model::GlyphEntry;
use fonts_rs_model::GlyphName;
use fonts_rs_model::sealed;
use miette::IntoDiagnostic;

/// Marker type identifying DSEG7 Classic in `GlyphName<SevenSegment>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SevenSegment {}

impl FontFamily for SevenSegment {}

impl sealed::Sealed for SevenSegment {}

/// `FontDefinition` implementation for DSEG7 Classic.
struct SevenSegmentDefinition;

impl FontDefinition for SevenSegmentDefinition {
    const GRESOURCE_PREFIX: &'static str = concatcp!(GRESOURCE_BASE_PREFIX, "/seven_segment");

    const ICONS_CONTEXT: &'static str = "glyphs";

    const CODEPOINT_RANGES: &[(u32, u32)] = &[(0x20, 0x7E)];

    type Name = GlyphName<SevenSegment>;

    type Family = SevenSegment;

    fn normalize_name(raw_glyph_name: &str) -> Option<Self::Name> {
        let name = normalize_to_kebab(raw_glyph_name)?;
        Some(GlyphName::new(format!("dseg7-{name}")))
    }
}

fn main() -> miette::Result<()> {
    println!("cargo:rustc-cfg=is_lib");

    let font_path = "resources/DSEG7Classic-Regular.ttf";
    println!("cargo:rerun-if-changed={font_path}");

    let metadata_path = Path::new("resources/metadata.json");
    let hash_path = Path::new("resources/.font-hash");
    let current_hash = hash_font_file(font_path);

    let needs_export = !metadata_path.exists()
        || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

    if needs_export {
        eprintln!("build.rs: exporting glyphs from DSEG7...");
        let count = SevenSegmentDefinition::export_glyphs(Path::new(font_path), Path::new("resources"))
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
    let entries: Vec<GlyphEntry<GlyphName<SevenSegment>>> = serde_json::from_str(&json).into_diagnostic()?;
    CodemapGenerator::run(&entries).into_diagnostic()?;
    RustConstantsGenerator::run(&entries).into_diagnostic()?;

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
