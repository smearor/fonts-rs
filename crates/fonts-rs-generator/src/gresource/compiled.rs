//! Compiled GResource bundle specification for build-time compilation.

use std::path::PathBuf;

/// Specifies a GResource bundle to compile at build time.
///
/// Each spec is compiled as
/// `glib_build_tools::compile_resources(&["resources"], xml, output)`.
pub struct GResourceSpec {
    /// GResource XML manifest path (relative to crate root).
    pub xml: PathBuf,
    /// Compiled output filename.
    pub output: PathBuf,
}

impl GResourceSpec {
    /// Create a new GResource bundle specification.
    ///
    /// `xml` is the GResource XML manifest path (relative to crate root).
    /// `output` is the compiled output filename.
    pub fn new(xml: impl Into<PathBuf>, output: impl Into<PathBuf>) -> Self {
        Self {
            xml: xml.into(),
            output: output.into(),
        }
    }
}
