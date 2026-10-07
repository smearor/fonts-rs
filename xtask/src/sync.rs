use crate::fetch::FetchCache;
use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::LockedArtifact;
use crate::lockfile::LockedInput;
use crate::lockfile::LockedUpstream;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::paths::crate_dir;
use crate::providers::ProviderContext;
use crate::providers::resolve_provider;
use crate::sha256::sha256_file;
use crate::sha256::sha256_hex;
use crate::upstream::GitSource;
use crate::upstream::sparse_paths;
use globset::Glob;
use globset::GlobSetBuilder;
use miette::IntoDiagnostic;
use miette::miette;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

/// Manifest file name looked up in each vendored crate.
const MANIFEST_NAME: &str = "upstream.toml";

/// Runs `cargo xtask sync -p <package>`: resolves the manifest ref,
/// ensures the cached checkout, materializes artifacts, runs
/// metadata providers and writes `upstream.lock`.
pub fn sync(package: &str) -> miette::Result<()> {
    let crate_dir = crate_dir(package)?;
    let manifest = Manifest::load(&crate_dir.join(MANIFEST_NAME))?;

    let git = GitSource::new(&manifest.upstream.repo, manifest.upstream.repo_url());
    let commit = git.resolve(&manifest.upstream.git_ref)?;
    eprintln!("{}: {} -> {commit}", manifest.upstream.repo, manifest.upstream.git_ref);

    let sparse = sparse_paths(
        manifest
            .artifacts
            .iter()
            .filter_map(|artifact| artifact.source.clone().map(|source| (source, artifact.pattern.clone(), artifact.exclude.clone()))),
        manifest
            .upstream
            .license
            .iter()
            .map(|license| license.source.clone())
            .chain(manifest.upstream.sparse.iter().cloned()),
    );
    let source = git.checkout(&commit, &sparse)?;
    eprintln!("{}: {}", if source.cache_hit { "cache hit" } else { "fetched" }, source.checkout.display());

    let fetch = FetchCache::default();
    let mut artifacts = Vec::new();
    let mut materialized: HashMap<String, (String, String)> = HashMap::new();

    if let Some(license) = &manifest.upstream.license {
        copy_file(&source.checkout, &crate_dir, &license.source, &license.target, &mut artifacts, &mut materialized)?;
    }
    for artifact in &manifest.artifacts {
        materialize(&source.checkout, &crate_dir, artifact, &fetch, &mut artifacts, &mut materialized)?;
    }

    let mut inputs: Vec<LockedInput> = Vec::new();
    for spec in &manifest.metadata {
        let provider = resolve_provider(&spec.provider)?;
        let context = ProviderContext {
            checkout: &source.checkout,
            with: &spec.with,
            fetch: &fetch,
        };
        let output = provider
            .generate(&context)
            .map_err(|e| e.wrap_err(format!("Provider '{}' failed", spec.provider)))?;
        let target = crate_dir.join(&spec.output);
        write_file(&target, &output.content)?;
        artifacts.push(LockedArtifact {
            target: spec.output.clone(),
            sha256: sha256_hex(&output.content),
            source: format!("provider:{}", spec.provider),
        });
        inputs.extend(output.inputs);
        eprintln!("provider {} -> {}", spec.provider, spec.output);
    }

    inputs.sort_by(|a, b| a.url.cmp(&b.url));
    inputs.dedup_by(|a, b| a.url == b.url);

    let lockfile = Lockfile {
        upstream: LockedUpstream {
            repo: manifest.upstream.repo.clone(),
            git_ref: manifest.upstream.git_ref.clone(),
            commit: commit.clone(),
        },
        artifacts,
        inputs,
    };

    remove_stale(&crate_dir, &lockfile)?;
    lockfile.save(&crate_dir.join(LOCKFILE_NAME))?;
    eprintln!("wrote {} ({} artifacts, {} inputs)", LOCKFILE_NAME, lockfile.artifacts.len(), lockfile.inputs.len());
    Ok(())
}

/// Copies `source` (checkout-relative file) to `target`
/// (crate-relative file) and records it.
fn copy_file(
    checkout: &Path,
    crate_dir: &Path,
    source: &str,
    target: &str,
    artifacts: &mut Vec<LockedArtifact>,
    materialized: &mut HashMap<String, (String, String)>,
) -> miette::Result<()> {
    let source_path = checkout.join(source);
    if !source_path.is_file() {
        return Err(miette!("Artifact source '{source}' does not exist in checkout"));
    }
    let sha = sha256_file(&source_path)?;
    copy_dedup(&source_path, &crate_dir.join(target), source, target, &sha, materialized)?;
    artifacts.push(LockedArtifact {
        target: target.to_string(),
        sha256: sha,
        source: source.to_string(),
    });
    Ok(())
}

/// Materializes one `[[artifacts]]` entry: a 1:1 file copy, or a
/// flattened glob copy of a directory.
fn materialize(
    checkout: &Path,
    crate_dir: &Path,
    artifact: &crate::manifest::Artifact,
    fetch: &FetchCache,
    artifacts: &mut Vec<LockedArtifact>,
    materialized: &mut HashMap<String, (String, String)>,
) -> miette::Result<()> {
    if let Some(url) = &artifact.url {
        if artifact.source.is_some() {
            return Err(miette!("Artifact sets both 'source' and 'url' - choose one"));
        }
        if artifact.pattern.is_some() || !artifact.exclude.is_empty() {
            return Err(miette!("Artifact '{url}': 'pattern'/'exclude' only apply to checkout sources"));
        }
        return fetch_url(crate_dir, artifact, url, fetch, artifacts, materialized);
    }
    let Some(source_path) = &artifact.source else {
        return Err(miette!("Artifact '{}' needs 'source' or 'url'", artifact.target));
    };
    let source = checkout.join(source_path);
    if source.is_file() {
        return copy_file(checkout, crate_dir, source_path, &artifact.target, artifacts, materialized);
    }
    if !source.is_dir() {
        return Err(miette!("Artifact source '{source_path}' does not exist in checkout"));
    }

    let pattern = artifact.pattern.as_deref().unwrap_or("**/*");
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(pattern).map_err(|e| miette!("Invalid pattern '{pattern}': {e}"))?);
    let glob = builder.build().into_diagnostic()?;

    let mut exclude_builder = GlobSetBuilder::new();
    for exclude in &artifact.exclude {
        exclude_builder.add(Glob::new(exclude).map_err(|e| miette!("Invalid exclude glob '{exclude}': {e}"))?);
    }
    let excludes = exclude_builder.build().into_diagnostic()?;

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    collect_matching(&source, &source, &glob, &excludes, &mut files)?;
    files.sort();
    for (relative, source_path) in files {
        let filename = source_path
            .file_name()
            .ok_or_else(|| miette!("Artifact '{}' produced a file without a name", source_path.display()))?;
        let target = format!("{}/{}", artifact.target.trim_end_matches('/'), filename.to_string_lossy());
        let sha = sha256_file(&source_path)?;
        copy_dedup(&source_path, &crate_dir.join(&target), &relative, &target, &sha, materialized)?;
        artifacts.push(LockedArtifact {
            target,
            sha256: sha,
            source: relative,
        });
    }
    Ok(())
}

/// Recursively collects `(checkout-relative path, absolute path)`
/// pairs matching `glob` but not `excludes` below `dir`.
fn collect_matching(
    root: &Path,
    dir: &Path,
    glob: &globset::GlobSet,
    excludes: &globset::GlobSet,
    files: &mut Vec<(String, PathBuf)>,
) -> miette::Result<()> {
    for entry in std::fs::read_dir(dir).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        if path.is_dir() {
            collect_matching(root, &path, glob, excludes, files)?;
        } else {
            let relative = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().into_owned();
            if glob.is_match(&relative) && !excludes.is_match(&relative) {
                files.push((relative, path));
            }
        }
    }
    Ok(())
}

/// Materializes a `url` artifact: fetches the URL via the shared
/// fetch cache and writes it verbatim to `target` (a single file).
/// The URL itself is recorded as the artifact `source`.
fn fetch_url(
    crate_dir: &Path,
    artifact: &crate::manifest::Artifact,
    url: &str,
    fetch: &FetchCache,
    artifacts: &mut Vec<LockedArtifact>,
    materialized: &mut HashMap<String, (String, String)>,
) -> miette::Result<()> {
    let body = fetch.get(url)?;
    let sha = sha256_hex(&body);
    if dedup_target(materialized, url, &artifact.target, &sha)? {
        write_file(&crate_dir.join(&artifact.target), &body)?;
    }
    artifacts.push(LockedArtifact {
        target: artifact.target.clone(),
        sha256: sha,
        source: url.to_string(),
    });
    Ok(())
}

/// Copies `source_path` to `target_path` unless another source file
/// with identical content was already materialized there (flattened
/// directory dedup). Conflicting contents for the same target are an
/// error.
fn copy_dedup(
    source_path: &Path,
    target_path: &Path,
    source: &str,
    target: &str,
    sha: &str,
    materialized: &mut HashMap<String, (String, String)>,
) -> miette::Result<()> {
    if dedup_target(materialized, source, target, sha)? {
        write_file(target_path, &std::fs::read(source_path).into_diagnostic()?)?;
    }
    Ok(())
}

/// Claims `target` for `source`/`sha` in the dedup map. Returns
/// `true` when the caller must write the file, `false` when an
/// identical artifact already materialized there; conflicting
/// content for the same target is an error.
fn dedup_target(
    materialized: &mut HashMap<String, (String, String)>,
    source: &str,
    target: &str,
    sha: &str,
) -> miette::Result<bool> {
    if let Some((first_source, first_sha)) = materialized.get(target) {
        if first_sha != sha {
            return Err(miette!("Artifacts '{first_source}' and '{source}' both map to '{target}' with different content"));
        }
        return Ok(false);
    }
    materialized.insert(target.to_string(), (source.to_string(), sha.to_string()));
    Ok(true)
}

/// Writes `content` to `path`, creating parent directories.
fn write_file(path: &Path, content: &[u8]) -> miette::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).into_diagnostic()?;
    }
    std::fs::write(path, content)
        .into_diagnostic()
        .map_err(|e| e.wrap_err(format!("Failed to write {}", path.display())))
}

/// Deletes files recorded in the previous lockfile that are no longer
/// produced, so removed upstream artifacts do not linger.
fn remove_stale(crate_dir: &Path, lockfile: &Lockfile) -> miette::Result<()> {
    let previous = match Lockfile::load(&crate_dir.join(LOCKFILE_NAME)) {
        Ok(Some(previous)) => previous,
        Ok(None) => return Ok(()),
        Err(error) => {
            eprintln!("warning: ignoring unparseable previous {LOCKFILE_NAME} ({error}); stale artifacts may linger");
            return Ok(());
        }
    };
    let current: HashSet<&str> = lockfile.artifacts.iter().map(|artifact| artifact.target.as_str()).collect();
    for artifact in &previous.artifacts {
        if current.contains(artifact.target.as_str()) {
            continue;
        }
        let path = crate_dir.join(&artifact.target);
        if path.is_file() {
            eprintln!("removing stale artifact {}", artifact.target);
            std::fs::remove_file(&path).into_diagnostic()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Flattened copies dedup identical content and reject conflicts.
    #[test]
    fn copy_dedup_detects_conflicts() {
        let dir = std::env::temp_dir().join(format!("xtask-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.txt");
        let b = dir.join("b.txt");
        std::fs::write(&a, b"same").unwrap();
        std::fs::write(&b, b"different").unwrap();
        let target = dir.join("out.txt");

        let mut materialized = HashMap::new();
        let sha_a = sha256_file(&a).unwrap();
        copy_dedup(&a, &target, "a", "out.txt", &sha_a, &mut materialized).unwrap();
        // Same content from another source: dedup, no error.
        copy_dedup(&a, &target, "a2", "out.txt", &sha_a, &mut materialized).unwrap();
        // Different content at the same target: conflict.
        let sha_b = sha256_file(&b).unwrap();
        assert!(copy_dedup(&b, &target, "b", "out.txt", &sha_b, &mut materialized).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
