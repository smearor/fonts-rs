use serde::Deserialize;

/// One `[[artifacts]]` entry: files copied from the upstream checkout
/// into the crate's `resources/` tree, or fetched verbatim from `url`.
#[derive(Debug, Deserialize)]
pub struct Artifact {
    /// Path inside the upstream checkout: a file for 1:1 copies, or a
    /// directory scanned recursively with `pattern`. Mutually
    /// exclusive with `url`.
    pub source: Option<String>,
    /// External URL fetched verbatim at sync time (e.g. a published
    /// npm/dist artifact absent from the git checkout). Mutually
    /// exclusive with `source`; `pattern` and `exclude` do not apply.
    #[serde(default)]
    pub url: Option<String>,
    /// Path below the crate directory: the file for 1:1 copies, or the
    /// directory matched files are flattened into (basename only).
    pub target: String,
    /// Glob matched against paths relative to `source` when `source`
    /// is a directory (e.g. `**/*.svg`). Defaults to `**/*`.
    #[serde(default)]
    pub pattern: Option<String>,
    /// Globs matched against paths relative to `source` that are
    /// skipped even if `pattern` matches (curated exclusions, e.g.
    /// icons hidden from the set on purpose).
    #[serde(default)]
    pub exclude: Vec<String>,
}
