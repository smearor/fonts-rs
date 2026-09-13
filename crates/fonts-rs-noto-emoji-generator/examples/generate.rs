//! CLI tool for generating Noto Emoji metadata.
//!
//! Usage:
//!   cargo run -p fonts-rs-noto-emoji-generator --example generate -- --json <metadata.json> --annotations <annotations.json> --emoji-test <emoji-test.txt> keywords
//!   cargo run -p fonts-rs-noto-emoji-generator --example generate -- --json <metadata.json> --annotations <annotations.json> --emoji-test <emoji-test.txt> categories

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use fonts_rs_model::GlyphEntry;
use fonts_rs_noto_emoji_generator::NotoEmojiMetadataGenerator;
use fonts_rs_noto_emoji_generator::parse_cldr_annotations;
use fonts_rs_noto_emoji_generator::parse_emoji_test;
use miette::Context;
use miette::IntoDiagnostic;

/// Generate phf::Map metadata tables for Noto Emoji.
#[derive(Parser)]
#[command(about = "Generate code from Noto Emoji metadata")]
struct Cli {
    /// Path to the generated `metadata.json` file (glyph entries).
    #[arg(short, long)]
    json: PathBuf,

    /// Path to CLDR `annotations.json` file.
    #[arg(long)]
    annotations: PathBuf,

    /// Path to Unicode `emoji-test.txt` file.
    #[arg(long)]
    emoji_test: PathBuf,

    /// Which generator to run.
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate keywords phf::Map content.
    Keywords,
    /// Generate categories phf::Map content.
    Categories,
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    let json = std::fs::read_to_string(&cli.json)
        .into_diagnostic()
        .with_context(|| format!("Failed to read {}", cli.json.display()))?;
    let entries: Vec<GlyphEntry<String>> = serde_json::from_str(&json)
        .into_diagnostic()
        .context("Failed to parse metadata JSON")?;

    let annotations = std::fs::read_to_string(&cli.annotations)
        .into_diagnostic()
        .with_context(|| format!("Failed to read {}", cli.annotations.display()))?;
    let (_name_map, keyword_map, _name_by_codepoint) = parse_cldr_annotations(&annotations)?;

    let emoji_test = std::fs::read_to_string(&cli.emoji_test)
        .into_diagnostic()
        .with_context(|| format!("Failed to read {}", cli.emoji_test.display()))?;
    let category_map: HashMap<u32, String> = parse_emoji_test(&emoji_test);

    match cli.command {
        Command::Keywords => {
            let output = NotoEmojiMetadataGenerator::generate_keywords(&entries, &keyword_map);
            println!("{output}");
        }
        Command::Categories => {
            let output = NotoEmojiMetadataGenerator::generate_categories(&entries, &category_map);
            println!("{output}");
        }
    }

    Ok(())
}
