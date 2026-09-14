// build.rs: export Libre Barcode Code 39 glyphs and generate codepoint maps + constants.

#[path = "src/definition.rs"]
mod definition;

use std::path::Path;

use definition::Code39Definition;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::build_constants;

fn main() -> std::io::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(definition::FONT_FILE);

    FontBuild::new(&font_path).compile_font_gresource().run(Code39Definition::export_glyphs)
}
