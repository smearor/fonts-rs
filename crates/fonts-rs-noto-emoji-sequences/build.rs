//! Emits the vendored resource directory to dependent build scripts.
//!
//! Dependents declaring this crate as a build-dependency read the
//! `DEP_NOTO_EMOJI_SEQUENCES_SVG_DIR` environment variable.

use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from) else {
        eprintln!("CARGO_MANIFEST_DIR not set");
        process::exit(1);
    };
    println!("cargo:svg_dir={}", manifest_dir.join("resources").join("svg").display());
    println!("cargo::rerun-if-changed=resources");
}
