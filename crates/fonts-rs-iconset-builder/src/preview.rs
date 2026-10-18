//! The [`Preview`] runner - exercises `IconSetBuild` against a temp
//! dir and reports what a real build would produce.

use fonts_rs_generator::IconSetBuild;
use fonts_rs_generator::IconSetBuildError;
use fonts_rs_model::CodePoint;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

/// One exported icon as the pipeline produces it.
pub struct ExportedIcon {
    /// Final icon name (`{prefix}-{name}-symbolic`).
    pub name: String,
    /// Hashed PUA codepoint (`U+E000`-range).
    pub codepoint: Option<CodePoint>,
    /// Exported SVG file inside the preview dir.
    pub file: PathBuf,
}

/// What a live `IconSetBuild::preview` run produced - `icons`,
/// plus the temp `dir` the exported SVGs landed in (keep it alive to
/// render them).
pub struct Preview {
    /// Exported icons in spec order.
    pub icons: Vec<ExportedIcon>,
    /// Temp dir holding the exported `*.svg` + `metadata.json`; deleted
    /// by the caller when the preview pane closes.
    pub dir: PathBuf,
}

impl Preview {
    /// Writes `icons_toml` to a temp spec file and runs
    /// [`IconSetBuild::preview`] over it.
    ///
    /// # Errors
    ///
    /// [`PreviewError::Spec`] when the spec file can't be written or
    /// parsed, [`PreviewError::Build`] for pipeline failures
    /// (`SourceNotFound`, `UnknownIcon`, …). Note: crate sources resolve
    /// via `cargo metadata` run in the *current* directory - preview
    /// only finds sources resolvable from the caller's CWD.
    pub fn run(icons_toml: &str) -> Result<Self, PreviewError> {
        let dir = temp_dir("fonts-rs-iconset-builder-preview")?;
        fs::write(dir.join("icons.toml"), icons_toml)?;
        fs::create_dir_all(dir.join("out"))?;

        let build = IconSetBuild::from_file(dir.join("icons.toml"))?;
        let entries = build.preview(&dir.join("out"))?;

        Ok(Self {
            icons: entries
                .into_iter()
                .map(|entry| ExportedIcon {
                    name: entry.name,
                    codepoint: entry.code,
                    // `entry.file` is gresource-relative
                    // (`resources/scalable/…`); the export drops the
                    // `resources/` root into `out` directly.
                    file: dir.join("out").join(entry.file.strip_prefix("resources/").unwrap_or(&entry.file)),
                })
                .collect(),
            dir,
        })
    }

    /// Deletes the preview temp dir.
    pub fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

/// Errors from the preview path.
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    /// Temp-dir or spec-file I/O failed.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// `icons.toml` parse/validate or the pipeline run failed.
    #[error(transparent)]
    Build(#[from] IconSetBuildError),
}

/// A unique temp directory under `std::env::temp_dir`.
fn temp_dir(prefix: &str) -> Result<PathBuf, io::Error> {
    let unique = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let dir = std::env::temp_dir().join(format!("{prefix}-{}-{unique}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder_settings::BuilderSettings;
    use crate::icon_selection::IconSelection;

    /// End-to-end: emitted `icons.toml` → `IconSetBuild::preview` -
    /// exercises the `cargo metadata` resolver fallback (workspace
    /// context) and the real export pipeline.
    #[test]
    fn emitted_spec_runs_the_real_pipeline() {
        let mut selection = IconSelection::new();
        selection.add("lucide-check-symbolic");
        selection.add("tabler-heart-symbolic");
        // Fluent's snake-case upstream stems export via the suggested
        // `{ name, rename }` pair the family reports.
        selection.add("fluent-panel-left-add-symbolic");
        let lucide = crate::icon_source::ICON_SOURCES.iter().position(|s| s.crate_name == "fonts-rs-lucide").unwrap();
        selection.add("lucide-trash-symbolic");
        selection.set_rename((lucide, None), "trash", Some("delete".to_string()));

        let settings = BuilderSettings {
            name: "AppIcons".to_string(),
            display_name: "App Icons".to_string(),
            icon_prefix: "app".to_string(),
            gresource_prefix: "/com/example/app/icons".to_string(),
            icons_context: "actions".to_string(),
        };
        let toml = selection.to_toml(&settings).unwrap();

        let preview = Preview::run(&toml).expect("preview failed - is this running inside the fonts-rs workspace?");
        let names: Vec<&str> = preview.icons.iter().map(|icon| icon.name.as_str()).collect();
        assert_eq!(names, ["app-check-symbolic", "app-delete-symbolic", "app-heart-symbolic", "app-panel-left-add-symbolic"]);
        for icon in &preview.icons {
            assert!(icon.file.exists(), "missing exported svg {}", icon.file.display());
            assert!(icon.codepoint.is_some());
        }
        preview.cleanup();
    }
}
