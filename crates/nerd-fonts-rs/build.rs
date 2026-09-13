// build.rs uses the generic export pipeline via FontBuild.

use std::path::Path;

use fonts_rs_generator::ExportError;
use fonts_rs_generator::FontBuild;
use nerd_fonts_generator::FontDefinition;
use nerd_fonts_generator::GlyphGenerator;
use nerd_fonts_generator::IconsCodemapGenerator;
use nerd_fonts_generator::IconsMetadataGenerator;
use nerd_fonts_generator::IconsRustGenerator;
use nerd_fonts_generator::MetadataGenerator;
use nerd_fonts_generator::NerdFontsDefinition;
use nerd_fonts_generator::WebCssGenerator;
use nerd_fonts_generator::generator::metadata::devicon::DeviconMetadata;
use nerd_fonts_generator::generator::metadata::fa::FaMetadata;
use nerd_fonts_generator::generator::metadata::md::MdMetadata;
use nerd_fonts_generator::generator::metadata::octicons::OcticonsMetadata;
use nerd_fonts_generator::generator::metadata::registry::IconMetadataRegistry;

use nerd_fonts_model::GlyphEntry;
use nerd_fonts_model::IconName;

fn main() -> miette::Result<()> {
    let font_path = "resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf";

    FontBuild::new(font_path)
        .additional_gresource("resources/nerd-fonts.gresource.xml", "compiled.gresource")
        .rerun_if_changed("resources/nerd-fonts.gresource.xml")
        .rerun_if_changed("resources/icons.gresource.xml")
        .rerun_if_changed("resources/metadata.json")
        .rerun_if_changed("resources/metadata/fontawesome/categories.yml")
        .rerun_if_changed("resources/metadata/fontawesome/icons.yml")
        .rerun_if_changed("resources/metadata/fontawesome/shims.json")
        .rerun_if_changed("resources/metadata/materialdesign-icons.json")
        .rerun_if_changed("resources/metadata/devicon.json")
        .rerun_if_changed("resources/metadata/octicons-keywords.json")
        .run_with(
            |font_path, resources_dir| {
                NerdFontsDefinition::export_glyphs(font_path, resources_dir).map_err(ExportError::from)
            },
            |json| {
                let icons: Vec<GlyphEntry<IconName>> = serde_json::from_str(json)?;

                IconsRustGenerator::run(&icons)?;
                IconsCodemapGenerator::run(&icons)?;
                WebCssGenerator::run(&icons)?;

                if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
                    eprintln!("build.rs: generating icon metadata (keywords/categories)...");

                    let registry = IconMetadataRegistry::new()
                        .register::<FaMetadata>(Path::new("resources/metadata/fontawesome"), "FA")?
                        .register::<MdMetadata>(Path::new("resources/metadata/materialdesign-icons.json"), "MD")?
                        .register::<DeviconMetadata>(Path::new("resources/metadata/devicon.json"), "Devicon")?
                        .register::<OcticonsMetadata>(Path::new("resources/metadata/octicons-keywords.json"), "Octicons")?;

                    IconsMetadataGenerator::new(&registry).run(&icons)?;
                }

                Ok(())
            },
        )
        .map_err(|e| miette::miette!("{e}"))?;

    Ok(())
}
