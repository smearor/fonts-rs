// build.rs: export Libre Barcode EAN13 glyphs and generate codepoint maps + constants.

#[path = "src/definition.rs"]
mod definition;

use std::path::Path;

use definition::Ean13Definition;
use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontDefinition;
use fonts_rs_generator::build_constants;
use fonts_rs_model::FontFamilyConfig;

fn main() -> std::io::Result<()> {
    let font_path = Path::new(build_constants::RESOURCES_DIR).join(definition::FONT_FILE);

    FontBuild::new(&font_path)
        .compile_font_gresource()
        .gresource_prefix(Ean13Definition::GRESOURCE_PREFIX)
        .run(Ean13Definition::export_glyphs)
}
