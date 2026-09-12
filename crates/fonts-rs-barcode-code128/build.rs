// build.rs: export Libre Barcode Code 128 glyphs and generate codepoint maps + constants.

#[path = "src/definition.rs"]
mod definition;

use definition::Code128Definition;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::build_constants;

fn main() -> std::io::Result<()> {
    let font_path = format!("{}/{}", build_constants::RESOURCES_DIR, definition::FONT_FILE);

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .run(|font_path, resources_dir| Code128Definition::export_glyphs(font_path, resources_dir))
}
