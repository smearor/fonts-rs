// build.rs: Noto Emoji `bitmap-noflags` variant - CBDT bitmap TTF
// without flag emoji + pre-rendered color SVGs for the GResource.
//
// The export pipeline lives in `fonts_rs_noto_emoji_generator::build`;
// this script only wires the resource-crate directories (`links`-style
// DEP_* environment variables) and the variant constant into it.

use std::path::PathBuf;

use fonts_rs_noto_emoji_generator::BuildInput;
use fonts_rs_noto_emoji_generator::Variant;
use fonts_rs_noto_emoji_generator::build;

/// Reads a `DEP_*` path emitted by a resource crate's build script.
fn dep_dir(var: &str) -> miette::Result<PathBuf> {
    std::env::var(var)
        .map(PathBuf::from)
        .map_err(|_| miette::miette!("{var} is not set - is the resource crate declared as a build-dependency?"))
}

fn main() -> miette::Result<()> {
    build(&BuildInput {
        svg_dirs: vec![dep_dir("DEP_NOTO_EMOJI_GLYPHS_SVG_DIR")?, dep_dir("DEP_NOTO_EMOJI_SEQUENCES_SVG_DIR")?],
        metadata_dir: dep_dir("DEP_NOTO_EMOJI_GLYPHS_METADATA_DIR")?,
        variant: Variant::BitmapNoflags,
        font_path_env: "NOTO_EMOJI_FONT_PATH",
    })
}
