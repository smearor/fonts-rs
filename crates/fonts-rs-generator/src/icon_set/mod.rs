//! Custom icon sets - TOML-driven glyph subsets for applications.
//!
//! An application declares a minimal icon set in a root-level
//! `icons.toml` and invokes [`IconSetBuild`] from its `build.rs`:
//!
//! ```ignore
//! fn main() -> miette::Result<()> {
//!     fonts_rs_generator::IconSetBuild::from_file("icons.toml")?.run()?;
//!     Ok(())
//! }
//! ```
//!
//! The build reads the [`IconSetSpec`] (defined in `fonts-rs-model` -
//! the schema is shared with authoring tools), resolves each
//! `[[iconset.sources]]` table via [`IconSetSourceResolver`], merges the
//! whitelisted glyphs through `CombinedSource`, and generates the
//! complete `iconset.rs` module via [`IconSetModuleGenerator`].
//!
//! See `concepts/planned/CUSTOM_ICON_SET.md` for the design.

pub mod build;
pub mod error;
pub mod module_generator;
pub mod post_processor;
pub mod source_resolver;

pub use build::IconSetBuild;
pub use error::IconSetBuildError;
pub use module_generator::IconSetModuleGenerator;
pub use post_processor::IconSetPostProcessorExt;
pub use source_resolver::IconSetSourceResolver;
