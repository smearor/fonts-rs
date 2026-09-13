// build.rs: export DSEG7 glyphs and generate codepoint maps + constants.
//
// Detects the active Cargo feature (e.g. `classic-regular`, `modern-mini-bold`)
// and exports glyphs from the corresponding DSEG7 TTF variant.

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontVariantExt;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_config;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::FontVariant;

#[path = "src/definition.rs"]
mod definition;

use definition::SevenSegmentConfig;

/// All DSEG7 variants and their mapping to TTF filenames.
const VARIANTS: VariantList<SevenSegmentConfig> = VariantList::new(&[
    // Classic
    FontVariant::file("classic-regular", "DSEG7Classic-Regular"),
    FontVariant::file("classic-bold", "DSEG7Classic-Bold"),
    FontVariant::file("classic-italic", "DSEG7Classic-Italic"),
    FontVariant::file("classic-bold-italic", "DSEG7Classic-BoldItalic"),
    FontVariant::file("classic-light", "DSEG7Classic-Light"),
    FontVariant::file("classic-light-italic", "DSEG7Classic-LightItalic"),
    // Classic Mini
    FontVariant::file("classic-mini-regular", "DSEG7ClassicMini-Regular"),
    FontVariant::file("classic-mini-bold", "DSEG7ClassicMini-Bold"),
    FontVariant::file("classic-mini-italic", "DSEG7ClassicMini-Italic"),
    FontVariant::file("classic-mini-bold-italic", "DSEG7ClassicMini-BoldItalic"),
    FontVariant::file("classic-mini-light", "DSEG7ClassicMini-Light"),
    FontVariant::file("classic-mini-light-italic", "DSEG7ClassicMini-LightItalic"),
    // Modern
    FontVariant::file("modern-regular", "DSEG7Modern-Regular"),
    FontVariant::file("modern-bold", "DSEG7Modern-Bold"),
    FontVariant::file("modern-italic", "DSEG7Modern-Italic"),
    FontVariant::file("modern-bold-italic", "DSEG7Modern-BoldItalic"),
    FontVariant::file("modern-light", "DSEG7Modern-Light"),
    FontVariant::file("modern-light-italic", "DSEG7Modern-LightItalic"),
    // Modern Mini
    FontVariant::file("modern-mini-regular", "DSEG7ModernMini-Regular"),
    FontVariant::file("modern-mini-bold", "DSEG7ModernMini-Bold"),
    FontVariant::file("modern-mini-italic", "DSEG7ModernMini-Italic"),
    FontVariant::file("modern-mini-bold-italic", "DSEG7ModernMini-BoldItalic"),
    FontVariant::file("modern-mini-light", "DSEG7ModernMini-Light"),
    FontVariant::file("modern-mini-light-italic", "DSEG7ModernMini-LightItalic"),
]);

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let font_path = entry.font_path()?;

    eprintln!("build.rs: active variant: {} -> {font_path}", entry);

    set_font_path_env("DSEG7_FONT_PATH", &font_path);

    let config = build_config::<SevenSegmentConfig>(Some(*entry));

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
