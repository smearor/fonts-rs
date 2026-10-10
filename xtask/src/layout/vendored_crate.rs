use super::Package;
use super::Workspace;
use miette::IntoDiagnostic;
use std::path::PathBuf;

/// A vendored crate: the package name plus its on-disk crate
/// directory carrying `upstream.toml` and `upstream.lock`.
#[derive(Debug)]
pub struct Crate {
    /// Vendored crate name (`crates/<package>`).
    pub package: Package,
    /// Crate root directory.
    pub dir: PathBuf,
}

impl Crate {
    /// Resolves the crate directory for `package`
    /// (`crates/<package>`), failing when it does not exist.
    pub fn load(package: Package) -> miette::Result<Self> {
        let dir = package.crate_dir()?;
        Ok(Self { package, dir })
    }

    /// Every `crates/*` directory that carries an `upstream.toml`,
    /// sorted by package name.
    pub fn vendored() -> miette::Result<Vec<Self>> {
        let crates = Workspace::root().join("crates");
        let mut vendored = Vec::new();
        for entry in std::fs::read_dir(&crates).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            let dir = entry.path();
            if dir.is_dir() && dir.join("upstream.toml").is_file() {
                vendored.push(Self {
                    package: Package::from(entry.file_name().to_string_lossy().into_owned()),
                    dir,
                });
            }
        }
        vendored.sort_by(|a, b| a.package.cmp(&b.package));
        Ok(vendored)
    }
}
