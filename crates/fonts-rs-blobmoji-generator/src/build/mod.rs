//! Build pipeline for the Blobmoji family crates.
//!
//! Called from each font crate's `build.rs`; see [`build`].

mod input;
mod pipeline;

pub use input::BuildInput;
pub use pipeline::build;
