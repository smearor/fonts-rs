//! CLI binary for exporting Nerd Font glyphs as GTK4 symbolic SVG icons.
//!
//! This is a thin wrapper around
//! `NerdFontsDefinition::export_glyphs`.
//!
//! Usage:
//! ```sh
//! cargo run -p nerd-fonts-generator --bin export_icons -- <font.ttf> -o resources/
//! ```

use std::path::PathBuf;

use clap::Parser;
use miette::Context;
use miette::IntoDiagnostic;
use nerd_fonts_generator::FontDefinition;
use nerd_fonts_generator::NerdFontsDefinition;

/// Export Nerd Fonts to GTK4 symbolic icons + GResource.
#[derive(Parser)]
#[command(about = "Export Nerd Font glyphs as GTK4 symbolic SVG icons")]
struct Cli {
    /// Path to the TTF/OTF font file.
    font: PathBuf,

    /// Output root directory.
    #[arg(short, long, default_value = "resources")]
    output: PathBuf,
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let font_display = cli.font.display().to_string();
    let count = NerdFontsDefinition::export_glyphs(&cli.font, &cli.output)
        .into_diagnostic()
        .context(format!("Failed to export icons from {font_display}"))?;
    println!("Exported {} icons to {}", count, cli.output.display());
    Ok(())
}
