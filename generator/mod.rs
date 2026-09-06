//! Generator module for build.rs context.
//!
//! Re-exports source files from `src/generator/` so that `super::` paths
//! resolve correctly when compiled as part of the build script.

#[path = "../src/generator/error.rs"]
pub mod error;

#[path = "../src/generator/generate.rs"]
pub mod generate;

#[path = "../src/generator/icons.rs"]
pub mod icons;

#[path = "../src/generator/codemap.rs"]
pub mod codemap;

#[path = "../src/generator/css.rs"]
pub mod css;

pub use crate::entry::IconEntry;
