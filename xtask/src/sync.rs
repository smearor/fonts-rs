use crate::fetch::FetchCache;
use crate::layout::Crate;
use crate::lockfile::LOCKFILE_NAME;
use crate::lockfile::LockedArtifact;
use crate::lockfile::LockedInput;
use crate::lockfile::LockedUpstream;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
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

impl Crate {
    /// Runs `cargo xtask sync -p <package>`: resolves the manifest
    /// ref, ensures the cached checkout, materializes artifacts,
    /// runs metadata providers and writes `upstream.lock`. With
    /// `update`, re-pins `ref` to the latest upstream state first
    /// (newest version tag for tag pins, default HEAD for commit
    /// pins; floating branch refs need no rewrite) and persists the
    /// new pin in `upstream.toml`.
    pub fn sync(&self, update: bool) -> miette::Result<()> {
        let crate_dir = self.dir.clone();
        let manifest_path = crate_dir.join(MANIFEST_NAME);
        let mut manifest = Manifest::load(&manifest_path)?;

        // Git upstream is optional: manifests made purely of `url`
        // artifacts have no checkout at all.
        let mut upstream_context = None;
        if let Some(upstream) = &mut manifest.upstream {
            let git = GitSource::new(&upstream.repo);
            if update
                && let Some((new_ref, _)) = git.update_target(&upstream.git_ref)?
                && new_ref != upstream.git_ref
            {
                eprintln!("{}: {} -> {new_ref} (re-pinned)", upstream.repo, upstream.git_ref);
                rewrite_manifest_ref(&manifest_path, &upstream.git_ref, &new_ref)?;
                upstream.git_ref = new_ref;
            }
            let commit = git.resolve(&upstream.git_ref)?;
            eprintln!("{}: {} -> {commit}", upstream.repo, upstream.git_ref);

            let sparse = sparse_paths(
                manifest.artifacts.iter().filter(|artifact| artifact.url.is_none()).filter_map(|artifact| {
                    artifact
                        .source
                        .clone()
                        .map(|source| (source, artifact.pattern.clone(), artifact.exclude.clone()))
                }),
                upstream
                    .license
                    .iter()
                    .map(|license| license.source.clone())
                    .chain(upstream.sparse.iter().cloned()),
            );
            let source = git.checkout(&commit, &sparse)?;
            eprintln!("{}: {}", if source.cache_hit { "cache hit" } else { "fetched" }, source.checkout.display());
            upstream_context = Some((upstream, commit, source));
        }
        let upstream = upstream_context;
        let checkout_path = upstream.as_ref().map(|(_, _, source)| source.checkout.clone());

        let fetch = FetchCache::default();
        let mut artifacts = Vec::new();
        let mut materialized: HashMap<String, (String, String)> = HashMap::new();

        if let Some((upstream, _, source)) = &upstream
            && let Some(license) = &upstream.license
        {
            copy_file(&source.checkout, &crate_dir, &license.source, &license.target, &mut artifacts, &mut materialized)?;
        }
        let empty_checkout = PathBuf::new();
        for artifact in &manifest.artifacts {
            materialize(
                checkout_path.as_deref().unwrap_or(&empty_checkout),
                &crate_dir,
                artifact,
                &fetch,
                &mut artifacts,
                &mut materialized,
            )?;
        }

        let mut inputs: Vec<LockedInput> = Vec::new();
        for spec in &manifest.metadata {
            let provider = resolve_provider(&spec.provider)?;
            let context = ProviderContext {
                checkout: checkout_path.as_deref(),
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
            upstream: upstream.map(|(upstream, commit, _)| LockedUpstream {
                repo: upstream.repo.clone(),
                git_ref: upstream.git_ref.clone(),
                commit,
            }),
            artifacts,
            inputs,
        };

        remove_stale(&crate_dir, &lockfile)?;
        lockfile.save(&crate_dir.join(LOCKFILE_NAME))?;
        eprintln!("wrote {} ({} artifacts, {} inputs)", LOCKFILE_NAME, lockfile.artifacts.len(), lockfile.inputs.len());
        Ok(())
    }
}

/// Rewrites `ref = "<old>"` to `ref = "<new>"` in `upstream.toml`,
/// replacing the whole line (stale pin annotations are dropped — the
/// bump is reviewed in the commit diff anyway).
fn rewrite_manifest_ref(manifest_path: &Path, old_ref: &str, new_ref: &str) -> miette::Result<()> {
    let text = std::fs::read_to_string(manifest_path).into_diagnostic()?;
    let mut replaced = false;
    let lines: Vec<String> = text
        .lines()
        .map(|line| {
            if !replaced && line.trim_start().starts_with("ref =") && line.contains(&format!("\"{old_ref}\"")) {
                replaced = true;
                format!("ref = \"{new_ref}\"")
            } else {
                line.to_string()
            }
        })
        .collect();
    if !replaced {
        return Err(miette!("ref = \"{old_ref}\" not found in {}", manifest_path.display()));
    }
    let mut body = lines.join("\n");
    if text.ends_with('\n') {
        body.push('\n');
    }
    std::fs::write(manifest_path, body)
        .into_diagnostic()
        .map_err(|e| e.wrap_err(format!("Failed to write {}", manifest_path.display())))
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
        return match &artifact.source {
            None => {
                if artifact.pattern.is_some() || !artifact.exclude.is_empty() {
                    return Err(miette!("Artifact '{url}': 'pattern'/'exclude' need a 'source' path inside the archive"));
                }
                fetch_url(crate_dir, artifact, url, fetch, artifacts, materialized)
            }
            Some(source) => materialize_archive(crate_dir, artifact, url, source, fetch, artifacts, materialized),
        };
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
fn collect_matching(root: &Path, dir: &Path, glob: &globset::GlobSet, excludes: &globset::GlobSet, files: &mut Vec<(String, PathBuf)>) -> miette::Result<()> {
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

/// Materializes a `url`+`source` artifact: fetches an archive
/// (`.tgz`/`.tar.gz`/`.zip`) and extracts `source` — a member file
/// for 1:1 copies, or a member directory globbed with
/// `pattern`/`exclude` and flattened into `target` like a checkout
/// directory. Provenance is `url#member`.
fn materialize_archive(
    crate_dir: &Path,
    artifact: &crate::manifest::Artifact,
    url: &str,
    source: &str,
    fetch: &FetchCache,
    artifacts: &mut Vec<LockedArtifact>,
    materialized: &mut HashMap<String, (String, String)>,
) -> miette::Result<()> {
    let body = fetch.get(url)?;

    let pattern = artifact.pattern.as_deref().unwrap_or("**/*");
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(pattern).map_err(|e| miette!("Invalid pattern '{pattern}': {e}"))?);
    let glob = builder.build().into_diagnostic()?;
    let mut exclude_builder = GlobSetBuilder::new();
    for exclude in &artifact.exclude {
        exclude_builder.add(Glob::new(exclude).map_err(|e| miette!("Invalid exclude glob '{exclude}': {e}"))?);
    }
    let excludes = exclude_builder.build().into_diagnostic()?;
    let prefix = format!("{}/", source.trim_end_matches('/'));

    let mut emit = |member: &str, content: &[u8]| -> miette::Result<bool> {
        let target = if member == source {
            artifact.target.clone()
        } else if let Some(relative) = member.strip_prefix(&prefix) {
            if !glob.is_match(relative) || excludes.is_match(relative) {
                return Ok(false);
            }
            let Some(basename) = std::path::Path::new(relative).file_name() else {
                return Ok(false);
            };
            format!("{}/{}", artifact.target.trim_end_matches('/'), basename.to_string_lossy())
        } else {
            return Ok(false);
        };
        let provenance = format!("{url}#{member}");
        let sha = sha256_hex(content);
        if dedup_target(materialized, &provenance, &target, &sha)? {
            write_file(&crate_dir.join(&target), content)?;
        }
        artifacts.push(LockedArtifact {
            target,
            sha256: sha,
            source: provenance,
        });
        Ok(true)
    };

    let mut matched = 0usize;
    if url.ends_with(".tgz") || url.ends_with(".tar.gz") {
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(body.as_slice()));
        for entry in archive.entries().into_diagnostic()? {
            let mut entry = entry.into_diagnostic()?;
            if !entry.header().entry_type().is_file() {
                continue;
            }
            let member = entry.path().into_diagnostic()?.to_string_lossy().into_owned();
            let mut content = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut content).into_diagnostic()?;
            if emit(&member, &content)? {
                matched += 1;
            }
        }
    } else if url.ends_with(".zip") {
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(body)).into_diagnostic()?;
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index).into_diagnostic()?;
            if entry.is_dir() {
                continue;
            }
            let member = entry.name().into_diagnostic()?.into_owned();
            let mut content = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut content).into_diagnostic()?;
            if emit(&member, &content)? {
                matched += 1;
            }
        }
    } else {
        return Err(miette!("Artifact '{url}': only .tgz/.tar.gz/.zip archives are supported"));
    }
    if matched == 0 {
        return Err(miette!("Artifact '{url}#{source}': no archive members matched"));
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
fn dedup_target(materialized: &mut HashMap<String, (String, String)>, source: &str, target: &str, sha: &str) -> miette::Result<bool> {
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
    /// Rewrites the ref line and drops its stale annotation.
    #[test]
    fn rewrites_manifest_ref() {
        let dir = std::env::temp_dir().join("xtask-test-rewrite-ref");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = dir.join("upstream.toml");
        std::fs::write(&manifest, "[upstream]\nrepo = \"a/b\"\nref = \"v1.0.0\"  # tag; vendored 2020-01-01\n").unwrap();
        rewrite_manifest_ref(&manifest, "v1.0.0", "v1.1.0").unwrap();
        let text = std::fs::read_to_string(&manifest).unwrap();
        assert!(text.contains("ref = \"v1.1.0\"\n"), "got: {text}");
        assert!(!text.contains("vendored 2020-01-01"), "stale comment kept: {text}");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// Missing refs surface as errors, not silent no-ops.
    #[test]
    fn rewrite_manifest_ref_fails_on_absent_ref() {
        let dir = std::env::temp_dir().join("xtask-test-rewrite-ref-missing");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = dir.join("upstream.toml");
        std::fs::write(&manifest, "[upstream]\nrepo = \"a/b\"\nref = \"v1.0.0\"\n").unwrap();
        assert!(rewrite_manifest_ref(&manifest, "v9.9.9", "v1.1.0").is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

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
