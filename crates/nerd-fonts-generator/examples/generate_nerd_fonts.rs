//! Example: using `nerd-fonts-generator` as a library.
//!
//! Demonstrates how to construct [`GlyphEntry<IconName>`] data and use the
//! [`GlyphGenerator`] trait to generate Rust constants, phf codepoint
//! maps, and web CSS — all in-memory, without writing files.
//!
//! Run with:
//! ```sh
//! cargo run -p nerd-fonts-generator --example generate -- rust --json resources/metadata.json
//! cargo run -p nerd-fonts-generator --example generate -- codemap
//! cargo run -p nerd-fonts-generator --example generate -- css
//! ```

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use miette::Context;
use miette::IntoDiagnostic;
use nerd_fonts_generator::GlyphGenerator;
use nerd_fonts_generator::IconsCodemapGenerator;
use nerd_fonts_generator::IconsRustGenerator;
use nerd_fonts_generator::WebCssGenerator;
use nerd_fonts_model::CodePoint;
use nerd_fonts_model::GlyphEntry;
use nerd_fonts_model::IconName;
use nerd_fonts_model::ResourcePath;

/// Generate Rust constants, phf codepoint maps, and web CSS from icon metadata.
#[derive(Parser)]
#[command(about = "Generate code from Nerd Font icon metadata")]
struct Cli {
    /// Path to a `metadata.json` file to load icons from.
    ///
    /// If omitted, a small built-in sample set is used.
    #[arg(short, long, global = true)]
    json: Option<PathBuf>,

    /// Which generator to run.
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate Rust icon name constants.
    Rust,
    /// Generate phf::Map codepoint lookup tables.
    Codemap,
    /// Generate web CSS with per-icon content mappings.
    Css,
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    let icons = match &cli.json {
        Some(path) => {
            let content = std::fs::read_to_string(path)
                .into_diagnostic()
                .context(format!("Failed to read {}", path.display()))?;
            serde_json::from_str::<Vec<GlyphEntry<IconName>>>(&content)
                .into_diagnostic()
                .context("Failed to parse metadata JSON")?
        }
        None => {
            eprintln!("No --json provided, using built-in sample icons");
            sample_icons()
        }
    };

    match cli.command {
        Command::Rust => {
            let output = IconsRustGenerator::generate(&icons).into_diagnostic().context("IconsRustGenerator failed")?;
            println!("{output}");
        }
        Command::Codemap => {
            let output = IconsCodemapGenerator::generate(&icons)
                .into_diagnostic()
                .context("IconsCodemapGenerator failed")?;
            println!("{output}");
        }
        Command::Css => {
            let output = WebCssGenerator::generate(&icons).into_diagnostic().context("WebCssGenerator failed")?;
            println!("{output}");
        }
    }

    Ok(())
}

fn sample_icons() -> Vec<GlyphEntry<IconName>> {
    let names = ["nf-fa-gamepad", "nf-fa-star", "nf-md-home", "nf-oct-mark-github"];

    names
        .iter()
        .enumerate()
        .filter_map(|(i, name)| {
            let icon_name = IconName::parse(name)?;
            let code = CodePoint::from(char::from_u32(0xF11B + i as u32)?);
            Some(GlyphEntry {
                code: Some(code),
                name: icon_name.clone(),
                file: PathBuf::from(format!("resources/icons/{icon_name}.svg")),
                resource_path: ResourcePath::from(&icon_name),
            })
        })
        .collect()
}
