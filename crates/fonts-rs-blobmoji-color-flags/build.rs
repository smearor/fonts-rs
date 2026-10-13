// build.rs: Blobmoji `color-flags` variant - COLRv1 vector TTF +
// pre-rendered color SVGs for the GResource.
//
// The export pipeline lives in `fonts_rs_blobmoji_generator::build`;
// this script only wires the resource-crate directories (`links`-style
// DEP_* environment variables) and the variant constant into it.

use std::path::PathBuf;

use fonts_rs_blobmoji_generator::BuildInput;
use fonts_rs_blobmoji_generator::Variant;
use fonts_rs_blobmoji_generator::build;

/// Reads a `DEP_*` path emitted by a resource crate's build script.
fn dep_dir(var: &str) -> miette::Result<PathBuf> {
    std::env::var(var)
        .map(PathBuf::from)
        .map_err(|_| miette::miette!("{var} is not set - is the resource crate declared as a dependency?"))
}

fn main() -> miette::Result<()> {
    build(&BuildInput {
        svg_dirs: vec![dep_dir("DEP_BLOBMOJI_GLYPHS_SVG_DIR")?, dep_dir("DEP_BLOBMOJI_SEQUENCES_SVG_DIR")?],
        metadata_dir: dep_dir("DEP_EMOJI_METADATA_DIR")?,
        variant: Variant::ColorFlags,
        font_path_env: "BLOBMOJI_FONT_PATH",
    })
}
