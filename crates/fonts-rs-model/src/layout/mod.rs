//! Workspace-layout domain types.
//!
//! Types describing the fonts-rs workspace's package layout - the
//! crate directory names shared by `xtask` (vendoring/release
//! tooling) and the icon-set spec (`crate = "…"` sources).

pub mod package;

pub use package::Package;
