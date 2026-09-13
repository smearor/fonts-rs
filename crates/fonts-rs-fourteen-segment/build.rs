// build.rs: export DSEG14 glyphs and generate codepoint maps + constants.
//
// Detects the active Cargo feature (e.g. `classic-regular`, `modern-mini-bold`)
// and exports glyphs from the corresponding DSEG14 TTF variant.

use fonts_rs_generator::FontBuild;
use fonts_rs_generator::FontVariantExt;
use fonts_rs_generator::VariantList;
use fonts_rs_generator::build_config;
use fonts_rs_generator::set_font_path_env;
use fonts_rs_model::FontVariant;

#[path = "src/definition.rs"]
mod definition;

use definition::FourteenSegmentConfig;

/// All DSEG14 variants and their mapping to TTF filenames.
const VARIANTS: VariantList<FourteenSegmentConfig> = VariantList::new(&[
    // Classic
    FontVariant::file("classic-regular", "DSEG14Classic-Regular"),
    FontVariant::file("classic-bold", "DSEG14Classic-Bold"),
    FontVariant::file("classic-italic", "DSEG14Classic-Italic"),
    FontVariant::file("classic-bold-italic", "DSEG14Classic-BoldItalic"),
    FontVariant::file("classic-light", "DSEG14Classic-Light"),
    FontVariant::file("classic-light-italic", "DSEG14Classic-LightItalic"),
    // Classic Mini
    FontVariant::file("classic-mini-regular", "DSEG14ClassicMini-Regular"),
    FontVariant::file("classic-mini-bold", "DSEG14ClassicMini-Bold"),
    FontVariant::file("classic-mini-italic", "DSEG14ClassicMini-Italic"),
    FontVariant::file("classic-mini-bold-italic", "DSEG14ClassicMini-BoldItalic"),
    FontVariant::file("classic-mini-light", "DSEG14ClassicMini-Light"),
    FontVariant::file("classic-mini-light-italic", "DSEG14ClassicMini-LightItalic"),
    // Modern
    FontVariant::file("modern-regular", "DSEG14Modern-Regular"),
    FontVariant::file("modern-bold", "DSEG14Modern-Bold"),
    FontVariant::file("modern-italic", "DSEG14Modern-Italic"),
    FontVariant::file("modern-bold-italic", "DSEG14Modern-BoldItalic"),
    FontVariant::file("modern-light", "DSEG14Modern-Light"),
    FontVariant::file("modern-light-italic", "DSEG14Modern-LightItalic"),
    // Modern Mini
    FontVariant::file("modern-mini-regular", "DSEG14ModernMini-Regular"),
    FontVariant::file("modern-mini-bold", "DSEG14ModernMini-Bold"),
    FontVariant::file("modern-mini-italic", "DSEG14ModernMini-Italic"),
    FontVariant::file("modern-mini-bold-italic", "DSEG14ModernMini-BoldItalic"),
    FontVariant::file("modern-mini-light", "DSEG14ModernMini-Light"),
    FontVariant::file("modern-mini-light-italic", "DSEG14ModernMini-LightItalic"),
]);

fn main() -> miette::Result<()> {
    let entry = VARIANTS.detect_and_get_active_variant(0)?;
    let font_path = entry.font_path()?;

    eprintln!("build.rs: active variant: {} -> {font_path}", entry);

    set_font_path_env("DSEG14_FONT_PATH", &font_path)?;

    let config = build_config::<FourteenSegmentConfig>(Some(*entry));

    FontBuild::new(&font_path)
        .run(|font_path, resources_dir| config.export_glyphs(font_path, resources_dir))
        .map_err(|e| miette::miette!("{e}"))?;

    config.write_variant_info()?;

    Ok(())
}
