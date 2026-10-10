use super::Crate;
use super::Package;

/// The selected set of vendored crates a command operates on: either
/// one crate picked via `-p` or every crate with an `upstream.toml`.
pub struct Workspace {
    /// Selected vendored crates.
    crates: Vec<Crate>,
    /// Whether the user explicitly selected a single package (`-p`).
    /// Distinguishes error propagation from aggregate reporting
    /// (`check` propagates hard errors only for single selections).
    single: bool,
}

impl Workspace {
    /// Loads the selection: `Some(package)` pins the run to that one
    /// crate, `None` scans for every vendored crate.
    pub fn load(package: Option<Package>) -> miette::Result<Self> {
        match package {
            Some(package) => Ok(Self {
                crates: vec![Crate::load(package)?],
                single: true,
            }),
            None => Ok(Self {
                crates: Crate::vendored()?,
                single: false,
            }),
        }
    }

    /// The selected vendored crates, in scan order.
    pub fn crates(&self) -> &[Crate] {
        &self.crates
    }

    /// Whether this run was pinned to a single package via `-p`.
    pub fn is_single(&self) -> bool {
        self.single
    }
}
