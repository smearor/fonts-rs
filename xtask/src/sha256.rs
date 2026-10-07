use miette::IntoDiagnostic;
use sha2::Digest;
use sha2::Sha256;
use std::io::Read;
use std::path::Path;

/// Returns the lowercase hex SHA-256 of `bytes`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Returns the lowercase hex SHA-256 of the file at `path`,
/// streaming the content in fixed-size chunks.
pub fn sha256_file(path: &Path) -> miette::Result<String> {
    let mut file = std::fs::File::open(path).into_diagnostic()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let read = file.read(&mut buffer).into_diagnostic()?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hashes a well-known input.
    #[test]
    fn sha256_hex_matches_known_digest() {
        assert_eq!(sha256_hex(b"hello world\n"), "a948904f2f0f479b8f8197694b30184b0d2ed1c1cd2a1ec0fb85d299a192a447");
    }
}
