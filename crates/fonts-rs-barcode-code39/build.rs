// build.rs: export Libre Barcode Code 39 glyphs and generate codepoint maps + constants.

use std::fs;
use std::path::Path;

#[path = "src/definition.rs"]
mod definition;

use definition::Code39Definition;
use fonts_rs_generator::CodemapGenerator;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::GlyphGenerator;
use fonts_rs_generator::RustConstantsGenerator;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::hash_font_file;
use fonts_rs_model::GlyphEntry;

fn main() -> std::io::Result<()> {
    println!("cargo:rustc-cfg=is_lib");

    let font_path = "resources/LibreBarcode39-Regular.ttf";
    println!("cargo:rerun-if-changed={font_path}");

    let metadata_path = Path::new(build_constants::METADATA_PATH);
    let hash_path = Path::new(build_constants::HASH_PATH);
    let current_hash = hash_font_file(font_path);

    let needs_export = !metadata_path.exists()
        || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

    if needs_export {
        eprintln!("build.rs: exporting glyphs from {font_path}...");
        let count = Code39Definition::export_glyphs(Path::new(font_path), Path::new(build_constants::RESOURCES_DIR))?;
        eprintln!("build.rs: exported {count} glyphs");
        fs::write(hash_path, &current_hash)?;
    }

    glib_build_tools::compile_resources(
        &[build_constants::RESOURCES_DIR],
        build_constants::ICONS_GRESOURCE_XML,
        build_constants::ICONS_GRESOURCE,
    );

    glib_build_tools::compile_resources(
        &[build_constants::RESOURCES_DIR],
        build_constants::FONT_GRESOURCE_XML,
        build_constants::FONT_GRESOURCE,
    );

    let json = fs::read_to_string(build_constants::METADATA_PATH)?;
    let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&json)?;
    CodemapGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;
    RustConstantsGenerator::run(&entries).map_err(|e| std::io::Error::other(e.to_string()))?;

    Ok(())
}
