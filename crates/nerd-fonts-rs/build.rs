use std::fs;
use std::io::Read;
use std::path::Path;

use nerd_fonts_generator::IconsCodemapGenerator;
use nerd_fonts_generator::IconsMetadataGenerator;
use nerd_fonts_generator::IconsRustGenerator;
use nerd_fonts_generator::NerdFontsGenerator;
use nerd_fonts_generator::WebCssGenerator;
use nerd_fonts_generator::export_icons;
use nerd_fonts_generator::generator::metadata::devicon::DeviconMetadata;
use nerd_fonts_generator::generator::metadata::fa::FaMetadata;
use nerd_fonts_generator::generator::metadata::md::MdMetadata;
use nerd_fonts_generator::generator::metadata::octicons::OcticonsMetadata;
use nerd_fonts_generator::generator::metadata::registry::IconMetadataRegistry;

use nerd_fonts_model::IconEntry;

fn main() {
    // Set cfg flag to indicate lib compilation (not build script).
    // This enables codepoint() in name.rs which needs the codepoint map.
    println!("cargo:rustc-cfg=is_lib");

    // ----------------------------
    // 1. Rebuild triggers
    // ----------------------------
    println!("cargo:rerun-if-changed=resources/nerd-fonts.gresource.xml");
    println!("cargo:rerun-if-changed=resources/icons.gresource.xml");
    println!("cargo:rerun-if-changed=resources/metadata.json");
    println!("cargo:rerun-if-changed=resources/metadata/fontawesome/categories.yml");
    println!("cargo:rerun-if-changed=resources/metadata/fontawesome/icons.yml");
    println!("cargo:rerun-if-changed=resources/metadata/fontawesome/shims.json");
    println!("cargo:rerun-if-changed=resources/metadata/materialdesign-icons.json");
    println!("cargo:rerun-if-changed=resources/metadata/devicon.json");
    println!("cargo:rerun-if-changed=resources/metadata/octicons-keywords.json");
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
        let count = export_icons(Path::new(font_path), Path::new("resources")).expect("Failed to export icons from font");
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

    // ----------------------------
    // 5. Generate metadata (keywords/categories) if feature is enabled
    // ----------------------------
    if std::env::var("CARGO_FEATURE_METADATA").is_ok() {
        eprintln!("build.rs: generating icon metadata (keywords/categories)...");

        let registry = IconMetadataRegistry::new()
            .register::<FaMetadata>(Path::new("resources/metadata/fontawesome"), "FA")
            .register::<MdMetadata>(Path::new("resources/metadata/materialdesign-icons.json"), "MD")
            .register::<DeviconMetadata>(Path::new("resources/metadata/devicon.json"), "Devicon")
            .register::<OcticonsMetadata>(Path::new("resources/metadata/octicons-keywords.json"), "Octicons");

        IconsMetadataGenerator::run_with_registry(&icons, &registry).expect("Failed to generate icon metadata");
    }
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
