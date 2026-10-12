//! Emits the vendored resource directories to dependent build scripts.
//!
//! Dependents declaring this crate as a build-dependency read the
//! `DEP_NOTO_EMOJI_GLYPHS_SVG_DIR` and `DEP_NOTO_EMOJI_GLYPHS_METADATA_DIR`
//! environment variables.

use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from) else {
        eprintln!("CARGO_MANIFEST_DIR not set");
        process::exit(1);
    };
    let resources = manifest_dir.join("resources");
    println!("cargo:svg_dir={}", resources.join("svg").display());
    println!("cargo:metadata_dir={}", resources.join("metadata").display());
    println!("cargo::rerun-if-changed=resources");
}
