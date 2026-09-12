// build.rs: export Libre Barcode Code 39 glyphs and generate codepoint maps + constants.

use std::fs;
use std::io::Read;
use std::path::Path;

#[path = "src/definition.rs"]
mod definition;

use definition::Code39Definition;
use fonts_rs_generator::CodemapGenerator;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::GlyphGenerator;
use fonts_rs_generator::RustConstantsGenerator;
use fonts_rs_model::GlyphEntry;

fn main() -> std::io::Result<()> {
    println!("cargo:rustc-cfg=is_lib");

    let font_path = "resources/LibreBarcode39-Regular.ttf";
    println!("cargo:rerun-if-changed={font_path}");

    let metadata_path = Path::new("resources/metadata.json");
    let hash_path = Path::new("resources/.font-hash");
    let current_hash = hash_font_file(font_path);

    let needs_export = !metadata_path.exists()
        || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

    if needs_export {
        eprintln!("build.rs: exporting glyphs from {font_path}...");
        let count = Code39Definition::export_glyphs(Path::new(font_path), Path::new("resources"))?;
        eprintln!("build.rs: exported {count} glyphs");
        fs::write(hash_path, &current_hash)?;
    }

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/icons.gresource.xml",
        "icons.gresource",
    );

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/font.gresource.xml",
        "font.gresource",
    );

    let json = fs::read_to_string("resources/metadata.json")?;
    let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&json)?;
    CodemapGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;
    RustConstantsGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;

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
