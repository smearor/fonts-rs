use std::path::PathBuf;

/// A resolved upstream source: the local sparse checkout of the
/// commit the manifest ref pinned to.
#[derive(Debug)]
pub struct ResolvedSource {
    /// Local checkout directory (cache-managed).
    pub checkout: PathBuf,
    /// Whether the checkout came from the cache (true) or was fetched
    /// fresh this run (false).
    pub cache_hit: bool,
}
