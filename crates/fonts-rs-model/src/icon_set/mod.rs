//! Serde model for application-defined custom icon sets (`icons.toml`).
//!
//! These types are the wire format shared between the build-time
//! generator (`fonts_rs_generator::IconSetBuild` *reads* `icons.toml`)
//! and authoring tools like the icon-set builder (which *writes* it).
//! All types derive `Serialize` and `Deserialize`; validation is pure
//! data — filesystem-dependent checks stay in the generator.

pub mod error;
pub mod file;
pub mod post_processor;
pub mod source_location;
pub mod source_spec;
pub mod spec;
pub mod whitelist_entry;

pub use crate::layout::Package;
pub use error::IconSetSpecError;
pub use file::IconSetFile;
pub use post_processor::IconSetPostProcessor;
pub use source_location::IconSetSourceLocation;
pub use source_spec::IconSetSourceSpec;
pub use spec::IconSetSpec;
pub use whitelist_entry::IconWhitelistEntry;
