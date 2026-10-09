use crate::fetch::FetchCache;
use crate::lockfile::LockedInput;
use miette::miette;
use std::path::Path;
use toml::Table;

/// A metadata provider: generates file content from the upstream
/// checkout and/or external inputs, driven by `[[metadata]] with.*`
/// parameters.
pub trait Provider {
    /// Registry name as used in `metadata.provider`.
    fn name(&self) -> &'static str;

    /// Generates the output content for one `[[metadata]]` entry.
    fn generate(&self, context: &ProviderContext) -> miette::Result<ProviderOutput>;
}

/// Everything a provider needs for one `[[metadata]]` invocation.
pub struct ProviderContext<'a> {
    /// Root of the upstream sparse checkout; `None` when the manifest
    /// has no git `[upstream]` (URL-only vendoring).
    pub checkout: Option<&'a Path>,
    /// The `with.*` parameter table.
    pub with: &'a Table,
    /// Shared per-sync HTTP fetch cache.
    pub fetch: &'a FetchCache,
}

impl<'a> ProviderContext<'a> {
    /// Fetches `url` via the shared cache (one request per URL per
    /// sync, regardless of how many providers need it).
    pub fn http_get(&self, url: &str) -> miette::Result<Vec<u8>> {
        self.fetch.get(url)
    }

    /// The upstream checkout root. Fails for manifests without a git
    /// `[upstream]` section — such providers must source purely via
    /// `http_get`/`with` inputs.
    pub fn checkout(&self) -> miette::Result<&'a Path> {
        self.checkout.ok_or_else(|| miette!("provider requires a git [upstream] checkout"))
    }
}

/// The result of a provider run: file content plus every external
/// input fetched while producing it.
pub struct ProviderOutput {
    /// Generated file content, written to the declared output path.
    pub content: Vec<u8>,
    /// External non-git inputs fetched during generation (recorded in
    /// the lockfile for provenance).
    pub inputs: Vec<LockedInput>,
}

impl ProviderOutput {
    /// Convenience constructor for outputs without external inputs.
    pub fn file(content: Vec<u8>) -> Self {
        Self { content, inputs: Vec::new() }
    }
}

/// Reads a required string parameter from a `with` table.
pub fn required_string(with: &Table, key: &str, provider: &str) -> miette::Result<String> {
    with.get(key)
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .ok_or_else(|| miette!("Provider '{provider}' requires 'with.{key}' (string)"))
}

/// Reads a required string-list parameter from a `with` table.
pub fn required_string_list(with: &Table, key: &str, provider: &str) -> miette::Result<Vec<String>> {
    with.get(key)
        .and_then(|value| value.as_array())
        .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
        .ok_or_else(|| miette!("Provider '{provider}' requires 'with.{key}' (list of strings)"))
}
