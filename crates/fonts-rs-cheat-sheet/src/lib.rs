//! Generic GTK4 cheat sheet for fonts-rs font families.
//!
//! [`CheatSheetFamily`] abstracts the per-family hooks (glyph list,
//! search, metadata, SVG source); [`run`] builds the complete
//! browser/search UI - icon grid, detail sidebar, navigation history -
//! shared by every family's cheat sheet binary.

mod app;
mod detail;
mod family;

pub use app::run;
pub use family::CheatSheetFamily;
