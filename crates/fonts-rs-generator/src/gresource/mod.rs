//! GResource XML manifest generation for GTK 4 icon bundles.

mod generate;
mod model;

pub use generate::generate_gresource_xml;
pub use model::GResource;
pub use model::GResourceFile;
pub use model::GResources;
