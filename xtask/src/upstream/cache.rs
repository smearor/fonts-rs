use miette::miette;
use std::path::PathBuf;

/// Returns the root of the upstream checkout cache:
/// `$XDG_CACHE_HOME/fonts-rs/upstream` (fallback `~/.cache/...`).
///
/// Checkouts are keyed by `<repo-key>/<commit>` so repeated syncs of
/// the same resolved commit are free.
pub fn cache_root() -> miette::Result<PathBuf> {
    if let Some(dir) = std::env::var_os("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(dir).join("fonts-rs/upstream"));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| miette!("Neither XDG_CACHE_HOME nor HOME is set"))?;
    Ok(PathBuf::from(home).join(".cache/fonts-rs/upstream"))
}

/// Derives a filesystem-safe cache key for a repository spec
/// (`owner/name` or URL): alphanumerics plus `-`, `.`, `_`.
pub fn repo_key(repo: &str) -> String {
    repo.chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_') { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sanitizes shorthand and URL repo specs.
    #[test]
    fn repo_key_is_filesystem_safe() {
        assert_eq!(repo_key("tailwindlabs/heroicons"), "tailwindlabs_heroicons");
        assert_eq!(repo_key("https://github.com/a/b.git"), "https___github.com_a_b.git");
    }
}
