//! Icon Set Builder - cross-family glyph selection that emits an
//! `icons.toml` for [`IconSetBuild`].
//!
//! The library holds the GTK-free pieces: the [`IconSource`] registry
//! (which crates feed `[[iconset.sources]]`), the [`IconSelection`]
//! model (picked upstream icons + renames per source, live exported-name
//! collision detection), the [`Scaffold`] paste block, and the
//! [`Preview`] runner driving `IconSetBuild::preview` over a temp dir.
//!
//! The `fonts-rs-iconset-builder` binary wraps these in a combined
//! cross-family browser: one grid over the icon-capable cheat-sheet
//! families, selection sidebar, settings pane, Save/Copy/Preview.

mod builder_settings;
mod icon_selection;
mod icon_source;
mod preview;
mod project_export;
mod scaffold;

pub use builder_settings::BuilderSettings;
pub use icon_selection::IconSelection;
pub use icon_selection::IconSelectionError;
pub use icon_selection::SelectedIcon;
pub use icon_selection::SourceKey;
pub use icon_source::ICON_SOURCES;
pub use icon_source::IconSource;
pub use preview::ExportedIcon;
pub use preview::Preview;
pub use preview::PreviewError;
pub use project_export::ProjectExport;
pub use project_export::ProjectExportError;
pub use scaffold::Scaffold;
