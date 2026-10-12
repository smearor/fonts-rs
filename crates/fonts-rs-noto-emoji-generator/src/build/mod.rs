//! Build pipeline for the Noto Emoji family crates.
//!
//! Called from each font crate's `build.rs`; see [`build`].

mod input;
mod pipeline;

pub use input::BuildInput;
pub use pipeline::build;
