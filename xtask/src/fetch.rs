use miette::IntoDiagnostic;
use miette::miette;
use std::cell::RefCell;
use std::collections::HashMap;
use std::process::Command;
use std::process::Stdio;

/// Per-sync HTTP fetch cache: providers sharing a URL (e.g. two
/// iconify providers on the same `icons.json`) fetch it only once.
#[derive(Default)]
pub struct FetchCache {
    /// URL → response body.
    bodies: RefCell<HashMap<String, Vec<u8>>>,
}

impl FetchCache {
    /// Returns the body of `url`, fetching and caching on first use.
    pub fn get(&self, url: &str) -> miette::Result<Vec<u8>> {
        if let Some(body) = self.bodies.borrow().get(url) {
            return Ok(body.clone());
        }
        let body = http_get(url)?;
        self.bodies.borrow_mut().insert(url.to_string(), body.clone());
        Ok(body)
    }
}

/// Fetches `url` over HTTP(S) via curl and returns the response body.
///
/// Kept as a small subprocess wrapper so the transport can be swapped
/// for a proper HTTP client later without touching providers.
pub fn http_get(url: &str) -> miette::Result<Vec<u8>> {
    let output = Command::new("curl")
        .args(["--silent", "--show-error", "--location", "--fail", url])
        .stderr(Stdio::piped())
        .output()
        .into_diagnostic()
        .map_err(|e| e.wrap_err("Failed to run curl"))?;
    if !output.status.success() {
        return Err(miette!("HTTP GET {url} failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(output.stdout)
}
