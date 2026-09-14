// build.rs: export Libre Barcode Code 128 glyphs and generate codepoint maps + constants.

#[path = "src/definition.rs"]
mod definition;

use std::path::Path;

use definition::Code128Definition;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::build_constants;

fn main() -> std::io::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(definition::FONT_FILE);

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .run(Code128Definition::export_glyphs)
}
