// build.rs: export Doto glyphs and generate codepoint maps + constants.
//
// Doto is a variable font with two axes: wght (weight) and rond (roundness).
// This build script selects one instance from a 5x5 matrix based on the
// active Cargo feature and exports glyphs at that variation location.

use std::fs;
use std::path::Path;

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontFamilyConfig;
use fonts_rs_generator::build_config;
use fonts_rs_generator::build_constants;
use fonts_rs_generator::detect_active_variant_index;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::AxisValue;
use fonts_rs_model::FontVariant;

#[path = "src/definition.rs"]
mod definition;

use definition::DotoConfig;

/// TTF font file name (relative to `resources/`).
pub const FONT_FILE: &str = "Doto.ttf";

/// All Doto variants in the 5x5 matrix: wght x rond.
const VARIANTS: &[FontVariant] = &[
    // wght=100 (ultra-light)
    FontVariant::axes("ultra-light-square", &[AxisValue::new("wght", 100.0), AxisValue::new("ROND", 0.0)]),
    FontVariant::axes("ultra-light-soft-square", &[AxisValue::new("wght", 100.0), AxisValue::new("ROND", 25.0)]),
    FontVariant::axes("ultra-light-medium", &[AxisValue::new("wght", 100.0), AxisValue::new("ROND", 50.0)]),
    FontVariant::axes("ultra-light-soft-dot", &[AxisValue::new("wght", 100.0), AxisValue::new("ROND", 75.0)]),
    FontVariant::axes("ultra-light-dot", &[AxisValue::new("wght", 100.0), AxisValue::new("ROND", 100.0)]),
    // wght=300 (light)
    FontVariant::axes("light-square", &[AxisValue::new("wght", 300.0), AxisValue::new("ROND", 0.0)]),
    FontVariant::axes("light-soft-square", &[AxisValue::new("wght", 300.0), AxisValue::new("ROND", 25.0)]),
    FontVariant::axes("light-medium", &[AxisValue::new("wght", 300.0), AxisValue::new("ROND", 50.0)]),
    FontVariant::axes("light-soft-dot", &[AxisValue::new("wght", 300.0), AxisValue::new("ROND", 75.0)]),
    FontVariant::axes("light-dot", &[AxisValue::new("wght", 300.0), AxisValue::new("ROND", 100.0)]),
    // wght=500 (regular)
    FontVariant::axes("regular-square", &[AxisValue::new("wght", 500.0), AxisValue::new("ROND", 0.0)]),
    FontVariant::axes("regular-soft-square", &[AxisValue::new("wght", 500.0), AxisValue::new("ROND", 25.0)]),
    FontVariant::axes("regular-medium", &[AxisValue::new("wght", 500.0), AxisValue::new("ROND", 50.0)]),
    FontVariant::axes("regular-soft-dot", &[AxisValue::new("wght", 500.0), AxisValue::new("ROND", 75.0)]),
    FontVariant::axes("regular-dot", &[AxisValue::new("wght", 500.0), AxisValue::new("ROND", 100.0)]),
    // wght=700 (bold)
    FontVariant::axes("bold-square", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 0.0)]),
    FontVariant::axes("bold-soft-square", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 25.0)]),
    FontVariant::axes("bold-medium", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 50.0)]),
    FontVariant::axes("bold-soft-dot", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 75.0)]),
    FontVariant::axes("bold-dot", &[AxisValue::new("wght", 700.0), AxisValue::new("ROND", 100.0)]),
    // wght=900 (extra-bold)
    FontVariant::axes("extra-bold-square", &[AxisValue::new("wght", 900.0), AxisValue::new("ROND", 0.0)]),
    FontVariant::axes("extra-bold-soft-square", &[AxisValue::new("wght", 900.0), AxisValue::new("ROND", 25.0)]),
    FontVariant::axes("extra-bold-medium", &[AxisValue::new("wght", 900.0), AxisValue::new("ROND", 50.0)]),
    FontVariant::axes("extra-bold-soft-dot", &[AxisValue::new("wght", 900.0), AxisValue::new("ROND", 75.0)]),
    FontVariant::axes("extra-bold-dot", &[AxisValue::new("wght", 900.0), AxisValue::new("ROND", 100.0)]),
];

fn main() -> miette::Result<()> {
    let idx = detect_active_variant_index(VARIANTS, 12, "Doto")?;
    let entry = &VARIANTS[idx];
    let font_path = format!("{}/{}", build_constants::RESOURCES_DIR, FONT_FILE);

    eprintln!("build.rs: active variant: {} -> {font_path}", entry);

    set_font_path_env("DOTO_FONT_PATH", &font_path);

    let config = build_config::<DotoConfig>(Some(*entry));

    let icons_dir = DotoConfig::icons_dir(Path::new(build_constants::RESOURCES_DIR));

    FontBuild::new(&font_path)
        .extra_hash(entry.as_str())
        .run(|font_path, resources_dir| {
            if icons_dir.exists() {
                fs::remove_dir_all(&icons_dir)?;
            }
            config.export_glyphs(font_path, resources_dir)
        })
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
