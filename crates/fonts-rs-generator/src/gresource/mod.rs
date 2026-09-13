//! GResource XML manifest model types for GTK 4 icon bundles.

pub(crate) mod compiled;
pub(crate) mod model;

pub use compiled::GResourceSpec;
pub use model::GResource;
pub use model::GResourceFile;
pub use model::GResources;
