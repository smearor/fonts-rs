use crate::git_ref::GitRef;
use crate::git_repository::GitRepository;
use crate::upstream::ResolvedSource;
use crate::upstream::cache_root;
use crate::upstream::repo_key;
use crate::url::Url;
use miette::IntoDiagnostic;
use miette::miette;
use std::path::Path;
use std::process::Command;

/// Git upstream backend: resolves refs via `ls-remote` and maintains
/// commit-keyed sparse checkouts under the cache root.
pub struct GitSource {
    /// Repository spec as declared in the manifest.
    repo: GitRepository,
    /// Remote URL derived from `repo`.
    url: Url,
}

impl GitSource {
    /// Creates a backend for `repo` (`owner/name` shorthand, URL or
    /// SSH remote); the remote URL is derived from the repo spec.
    pub fn new(repo: &GitRepository) -> Self {
        Self {
            repo: repo.clone(),
            url: repo.remote_url(),
        }
    }

    /// Resolves `git_ref` (tag, branch, `HEAD` or a full commit SHA)
    /// to a commit SHA via `git ls-remote`.
    pub fn resolve(&self, git_ref: &GitRef) -> miette::Result<String> {
        if git_ref.is_commit_sha() {
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

    /// Resolves the remote's default HEAD via `git ls-remote <url> HEAD`.
    pub fn default_head(&self) -> miette::Result<String> {
        let output = git(&["ls-remote", &self.url, "HEAD"])?;
        output
            .lines()
            .next()
            .and_then(|line| line.split('\t').next())
            .map(str::to_string)
            .ok_or_else(|| miette!("No HEAD advertised by {}", self.url))
    }

    /// Whether `git_ref` names a tag on the remote.
    pub fn is_tag(&self, git_ref: &GitRef) -> miette::Result<bool> {
        Ok(!git(&["ls-remote", &self.url, &format!("refs/tags/{git_ref}")])?.trim().is_empty())
    }

    /// The newest version-like tag (`v?\d+.\d+...`) on the remote,
    /// compared by numeric components. Non-version tags (pre-releases,
    /// named tags) are ignored; `^{}` deref lines are skipped.
    pub fn latest_tag(&self) -> miette::Result<GitRef> {
        let output = git(&["ls-remote", "--tags", &self.url])?;
        output
            .lines()
            .filter_map(|line| line.split('\t').nth(1))
            .filter_map(|name| name.strip_prefix("refs/tags/"))
            .filter(|name| !name.ends_with("^{}"))
            .filter_map(|name| version_key(name).map(|key| (key, name)))
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(_, name)| GitRef::from(name))
            .ok_or_else(|| miette!("No version tags found on {}", self.url))
    }

    /// The re-pin target for `git_ref`: tag pins track the newest
    /// version tag, commit pins track the default HEAD; floating
    /// branch refs have no update target. Returns `(new_ref, commit)`.
    pub fn update_target(&self, git_ref: &GitRef) -> miette::Result<Option<(GitRef, String)>> {
        if !git_ref.is_commit_sha() && self.is_tag(git_ref)? {
            let tag = self.latest_tag()?;
            let commit = self.resolve(&tag)?;
            return Ok(Some((tag, commit)));
        }
        if git_ref.is_commit_sha() {
            let head = self.default_head()?;
            return Ok(Some((GitRef::from(head.clone()), head)));
        }
        Ok(None)
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
pub(crate) fn is_commit_sha(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|c| c.is_ascii_hexdigit())
}

/// Version key for tag sorting: `vX.Y.Z...` → `[X, Y, Z]`; `None`
/// for non-numeric tags (pre-releases, named tags). Requires at
/// least a major.minor pair so `v1` does not outrank real releases.
fn version_key(tag: &str) -> Option<Vec<u64>> {
    let tag = tag.strip_prefix('v').unwrap_or(tag);
    let parts: Option<Vec<u64>> = tag.split('.').map(|part| part.parse().ok()).collect();
    parts.filter(|parts| parts.len() >= 2)
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
pub fn sparse_paths(artifacts: impl Iterator<Item = (String, Option<String>, Vec<String>)>, extra: impl Iterator<Item = String>) -> Vec<String> {
    let mut paths: Vec<String> = artifacts
        .flat_map(|(source, pattern, excludes)| {
            let source = source.trim_end_matches('/').to_string();
            let include = match pattern {
                Some(pattern) => format!("{source}/{pattern}"),
                None => source.clone(),
            };
            std::iter::once(include).chain(excludes.into_iter().map(move |exclude| format!("!{source}/{exclude}")))
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

    /// Parses dotted version tags; rejects pre-releases and majors.
    #[test]
    fn version_key_parsing() {
        assert_eq!(version_key("v3.48.0"), Some(vec![3, 48, 0]));
        assert_eq!(version_key("5.15.4"), Some(vec![5, 15, 4]));
        assert_eq!(version_key("v0.50beta1"), None);
        assert_eq!(version_key("release-2024"), None);
        assert_eq!(version_key("v1"), None);
    }

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
                ("optimized/24/outline".to_string(), Some("*.svg".to_string()), Vec::new()),
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
