use crate::upstream::ResolvedSource;
use crate::upstream::cache_root;
use crate::upstream::repo_key;
use miette::IntoDiagnostic;
use miette::miette;
use std::path::Path;
use std::process::Command;

/// Git upstream backend: resolves refs via `ls-remote` and maintains
/// commit-keyed sparse checkouts under the cache root.
pub struct GitSource {
    /// Repository spec as declared in the manifest.
    repo: String,
    /// Remote URL derived from `repo`.
    url: String,
}

impl GitSource {
    /// Creates a backend for `repo` (`owner/name` shorthand, URL or
    /// SSH remote).
    pub fn new(repo: &str, url: String) -> Self {
        Self { repo: repo.to_string(), url }
    }

    /// Resolves `git_ref` (tag, branch, `HEAD` or a full commit SHA)
    /// to a commit SHA via `git ls-remote`.
    pub fn resolve(&self, git_ref: &str) -> miette::Result<String> {
        if is_commit_sha(git_ref) {
            return Ok(git_ref.to_string());
        }
        let output = git(&["ls-remote", &self.url, &format!("{git_ref}^{{}}"), git_ref])?;
        let mut tag_peeled = None;
        let mut tag = None;
        let mut branch = None;
        for line in output.lines() {
            let (sha, name) = line.split_once('\t').ok_or_else(|| miette!("Malformed ls-remote line: {line}"))?;
            match name {
                n if n == format!("refs/tags/{git_ref}^{{}}") => tag_peeled = Some(sha),
                n if n == format!("refs/tags/{git_ref}") => tag = Some(sha),
                n if n == format!("refs/heads/{git_ref}") => branch = Some(sha),
                _ => {}
            }
        }
        tag_peeled
            .or(tag)
            .or(branch)
            .map(str::to_string)
            .ok_or_else(|| miette!("Ref '{git_ref}' not found on {}", self.url))
    }

    /// Ensures a sparse checkout of `commit` exists in the cache and
    /// returns it. `sparse_paths` are gitignore-style patterns matched
    /// against the repository root (`dir` covers the whole tree). The
    /// cache key includes a digest of the pattern set so manifests
    /// with different file selections never share a checkout.
    pub fn checkout(&self, commit: &str, sparse_paths: &[String]) -> miette::Result<ResolvedSource> {
        let sparse_key = crate::sha256::sha256_hex(sparse_paths.join("\n").as_bytes());
        let dir = cache_root()?.join(repo_key(&self.repo)).join(format!("{commit}-{}", &sparse_key[..12]));
        if dir.is_dir() {
            return Ok(ResolvedSource {
                checkout: dir,
                cache_hit: true,
            });
        }

        let tmp = dir.with_file_name(format!(".{}.tmp", dir.file_name().unwrap_or_default().to_string_lossy()));
        if tmp.exists() {
            std::fs::remove_dir_all(&tmp).into_diagnostic()?;
        }
        if let Some(parent) = tmp.parent() {
            std::fs::create_dir_all(parent).into_diagnostic()?;
        }
        std::fs::create_dir_all(&tmp).into_diagnostic()?;

        let result = (|| -> miette::Result<()> {
            git_in(&tmp, &["init", "--quiet"])?;
            git_in(&tmp, &["remote", "add", "origin", &self.url])?;
            git_in(&tmp, &["sparse-checkout", "init", "--no-cone"])?;
            let mut set_args = vec!["sparse-checkout", "set", "--no-cone"];
            set_args.extend(sparse_paths.iter().map(String::as_str));
            git_in(&tmp, &set_args)?;
            git_in(&tmp, &["fetch", "--quiet", "--depth", "1", "--filter=blob:none", "origin", commit])?;
            git_in(&tmp, &["checkout", "--quiet", "--detach", "FETCH_HEAD"])?;
            Ok(())
        })();

        match result {
            Ok(()) => {
                std::fs::rename(&tmp, &dir).into_diagnostic()?;
                Ok(ResolvedSource {
                    checkout: dir,
                    cache_hit: false,
                })
            }
            Err(e) => {
                let _ = std::fs::remove_dir_all(&tmp);
                Err(e.wrap_err(format!("Failed to fetch {}@{commit}", self.repo)))
            }
        }
    }
}

/// Whether `value` looks like a full 40-hex commit SHA.
fn is_commit_sha(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|c| c.is_ascii_hexdigit())
}

/// Runs `git` with `args` outside a repository and returns stdout.
fn git(args: &[&str]) -> miette::Result<String> {
    run_git(None, args)
}

/// Runs `git` inside `dir` and returns stdout.
fn git_in(dir: &Path, args: &[&str]) -> miette::Result<String> {
    run_git(Some(dir), args)
}

/// Shared git subprocess wrapper: fails with stderr on non-zero exit.
fn run_git(dir: Option<&Path>, args: &[&str]) -> miette::Result<String> {
    let mut command = Command::new("git");
    command.env("GIT_TERMINAL_PROMPT", "0").args(args);
    if let Some(dir) = dir {
        command.current_dir(dir);
    }
    let output = command.output().into_diagnostic().map_err(|e| e.wrap_err("Failed to run git"))?;
    if !output.status.success() {
        return Err(miette!("git {} failed: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim()));
    }
    String::from_utf8(output.stdout).into_diagnostic()
}

/// Returns the sorted, deduplicated non-cone sparse-checkout pattern
/// set for a manifest. Artifact sources with a glob `pattern` yield
/// `source/pattern` (so e.g. `assets/**/ic_fluent_*_24_regular.svg`
/// checks out only the 24px slices of a giant tree); plain sources
/// match the file itself or, for directories, the whole subtree.
/// `extra` covers license sources and explicit `sparse` entries.
/// Artifact `exclude` globs become `!`-negated patterns so excluded
/// blobs are not checked out either.
pub fn sparse_paths(
    artifacts: impl Iterator<Item = (String, Option<String>, Vec<String>)>,
    extra: impl Iterator<Item = String>,
) -> Vec<String> {
    let mut paths: Vec<String> = artifacts
        .flat_map(|(source, pattern, excludes)| {
            let source = source.trim_end_matches('/').to_string();
            let include = match pattern {
                Some(pattern) => format!("{source}/{pattern}"),
                None => source.clone(),
            };
            std::iter::once(include).chain(
                excludes
                    .into_iter()
                    .map(move |exclude| format!("!{source}/{exclude}")),
            )
        })
        .chain(extra)
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Recognizes full SHAs and rejects refs.
    #[test]
    fn detects_commit_sha() {
        assert!(is_commit_sha("a563cf9166f4f91aa617557ed272612b7f0a2f72"));
        assert!(!is_commit_sha("v2.2.0"));
        assert!(!is_commit_sha("a563cf9166f4f91aa617557ed272612b7f0a2f7")); // 39 chars
    }

    /// Glob patterns extend the source path; excludes become `!`
    /// negations; plain sources stay whole.
    #[test]
    fn sparse_paths_combine_source_pattern_and_excludes() {
        let paths = sparse_paths(
            [
                (
                    "optimized/24/outline".to_string(),
                    Some("*.svg".to_string()),
                    Vec::new(),
                ),
                (
                    "assets".to_string(),
                    Some("*/SVG/ic_fluent_*_24_regular.svg".to_string()),
                    vec!["*/SVG/ic_fluent_flag_pride_24_*.svg".to_string()],
                ),
                ("svg".to_string(), None, Vec::new()),
            ]
            .into_iter(),
            ["LICENSE".to_string()].into_iter(),
        );
        assert_eq!(
            paths,
            [
                "!assets/*/SVG/ic_fluent_flag_pride_24_*.svg",
                "LICENSE",
                "assets/*/SVG/ic_fluent_*_24_regular.svg",
                "optimized/24/outline/*.svg",
                "svg"
            ]
        );
    }
}
