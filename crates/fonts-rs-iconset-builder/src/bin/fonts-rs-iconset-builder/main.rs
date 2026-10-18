//! Icon Set Builder - a cross-family glyph browser that emits an
//! `icons.toml` for `IconSetBuild`.
//!
//! One combined grid over the icon-capable families (filter chips +
//! shared search), a selection sidebar grouped by source with per-entry
//! rename editing and live exported-name collision detection, plus
//! Save (`icons.toml`), Copy scaffold and Preview (`IconSetBuild::preview`).

mod app;

use libadwaita as adw;
use miette::Result;

fn main() -> Result<adw::glib::ExitCode> {
    app::run()
}
