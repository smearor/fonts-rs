//! Serde model types for GResource XML manifest serialization.

use serde::Serialize;

/// Root `<gresources>` element.
#[derive(Debug, Serialize)]
#[serde(rename = "gresources")]
pub struct GResources {
    /// One or more `<gresource>` bundles.
    #[serde(rename = "gresource")]
    pub gresource: GResource,
}

/// A single `<gresource prefix="...">` bundle.
#[derive(Debug, Serialize)]
#[serde(rename = "gresource")]
pub struct GResource {
    /// The GResource prefix (e.g. `/io/smearor/nerd_fonts`).
    #[serde(rename = "@prefix")]
    pub prefix: String,

    /// File entries within this bundle.
    #[serde(rename = "file")]
    pub files: Vec<GResourceFile>,
}

/// A `<file>` element within a `<gresource>`.
#[derive(Debug, Serialize)]
#[serde(rename = "file")]
pub struct GResourceFile {
    /// The file path relative to the resource root.
    #[serde(rename = "$text")]
    pub path: String,
}
