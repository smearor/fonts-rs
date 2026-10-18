//! The [`Scaffold`] copy-paste block - `Cargo.toml` deps, `build.rs`
//! and module stub for the emitted `icons.toml`.

use crate::builder_settings::BuilderSettings;
use crate::icon_selection::IconSelection;

/// Generates the project-side snippets the target app needs to consume
/// the emitted `icons.toml`.
pub struct Scaffold;

impl Scaffold {
    /// The paste-ready scaffold: `[dependencies]` entries for the
    /// selected source crates, `fonts-rs-generator` under
    /// `[build-dependencies]`, the one-line `build.rs` and the
    /// `include!` module stub - exactly the CUSTOM_ICON_SET surface.
    pub fn text(selection: &IconSelection, settings: &BuilderSettings) -> String {
        let mut deps = String::new();
        for crate_name in used_source_crates(selection) {
            deps.push_str(&format!("{crate_name} = \"0.5\"\n"));
        }

        format!(
            "# Cargo.toml\n\
             [dependencies]\n\
             {deps}\
             \n\
             [build-dependencies]\n\
             fonts-rs-generator = \"0.5\"\n\
             \n\
             # build.rs\n\
             fn main() -> miette::Result<()> {{\n\
             \x20   fonts_rs_generator::IconSetBuild::from_file(\"icons.toml\")?.run()?;\n\
             \x20   Ok(())\n\
             }}\n\
             \n\
             # src/iconset.rs (generated marker module)\n\
             include!(concat!(env!(\"OUT_DIR\"), \"/iconset.rs\"));\n\
             \n\
             # usage\n\
             {name}::install()?;            # registers fonts + icon theme\n\
             gtk4::Image::from_icon_name(\"{prefix}-<icon>-symbolic\");\n",
            deps = deps,
            name = settings.name,
            prefix = settings.icon_prefix,
        )
    }
}

/// Distinct source crate names that have at least one picked icon.
fn used_source_crates(selection: &IconSelection) -> Vec<&'static str> {
    let mut crates: Vec<&'static str> = selection.groups().map(|(_key, source, _icons)| source.crate_name).collect();
    crates.sort_unstable();
    crates.dedup();
    crates
}
