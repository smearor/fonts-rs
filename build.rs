use std::fs;
use std::io::Read;
use std::path::Path;

// Include the codepoint, resource_path, entry, and export modules directly
// so build.rs can call export_icons(). This avoids a separate crate while
// keeping the logic in one place. The module structure mirrors the crate so
// that `export.rs` can use `super::codepoint::CodePoint`,
// `super::resource_path::ResourcePath`, and `super::entry::IconEntry`.
#[path = "src/icons/codepoint.rs"]
mod codepoint;

#[path = "src/icons/name.rs"]
mod name;

#[path = "src/icons/paths.rs"]
mod paths;

#[path = "src/icons/resource_path.rs"]
mod resource_path;

#[path = "src/icons/svg_path_builder.rs"]
mod svg_path_builder;

#[path = "src/icons/entry.rs"]
mod entry;

#[path = "src/icons/export.rs"]
mod export;

// Include the generator modules as a nested module so `super::` paths work.
#[path = "generator/mod.rs"]
mod generator;

use entry::IconEntry;
use generator::codemap::IconsCodemapGenerator;
use generator::css::WebCssGenerator;
use generator::generate::NerdFontsGenerator;
use generator::icons::IconsRustGenerator;

fn main() {
    // ----------------------------
    // 1. Rebuild triggers
    // ----------------------------
    println!("cargo:rerun-if-changed=resources/nerd-fonts.gresource.xml");
    println!("cargo:rerun-if-changed=resources/icons.gresource.xml");
    println!("cargo:rerun-if-changed=resources/metadata.json");
    println!("cargo:rerun-if-changed=build.rs");

    // Trigger rebuild when the source font file changes
    let font_path = "resources/NerdFontsSymbolsOnly/SymbolsNerdFont-Regular.ttf";
    println!("cargo:rerun-if-changed={}", font_path);

    // ----------------------------
    // 2. Export icons from font (if missing or font changed)
    // ----------------------------
    let metadata_path = Path::new("resources/metadata.json");
    let hash_path = Path::new("resources/.font-hash");
    let current_hash = hash_font_file(font_path);

    let needs_export = !metadata_path.exists() || fs::read_to_string(hash_path).ok().as_deref() != Some(current_hash.as_str());

    if needs_export {
        eprintln!("build.rs: exporting icons from font...");
        let count = export::export_icons(Path::new(font_path), Path::new("resources")).expect("Failed to export icons from font");
        eprintln!("build.rs: exported {} icons", count);
        fs::write(hash_path, &current_hash).expect("Failed to write font hash");
    }

    // ----------------------------
    // 3. Compile GResource bundles
    // ----------------------------
    glib_build_tools::compile_resources(&["resources"], "resources/nerd-fonts.gresource.xml", "compiled.gresource");
    glib_build_tools::compile_resources(&["resources"], "resources/icons.gresource.xml", "icons.gresource");

    // ----------------------------
    // 4. Generate Rust constants and phf maps from metadata.json
    // ----------------------------
    let json = fs::read_to_string("resources/metadata.json").expect("Failed to read metadata.json");
    let icons: Vec<IconEntry> = serde_json::from_str(&json).expect("Invalid metadata.json");
    IconsRustGenerator::run(&icons).expect("Failed to generate Rust icons");
    IconsCodemapGenerator::run(&icons).expect("Failed to generate codepoint map");
    WebCssGenerator::run(&icons).expect("Failed to generate web CSS");
}

// ----------------------------
// Font file hashing (FNV-1a)
// ----------------------------
fn hash_font_file(path: &str) -> String {
    let mut file = fs::File::open(path).expect("Failed to open font file");
    let mut buffer = [0u8; 8192];
    let mut hash: u64 = 0xcbf29ce484222325;
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        for &byte in &buffer[..n] {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("{:016x}", hash)
}
