//! Generic GTK4 cheat sheet for fonts-rs font families.
//!
//! [`CheatSheetFamily`] abstracts the per-family hooks (glyph list,
//! search, metadata, SVG source); [`run`] builds the complete
//! browser/search UI - icon grid, detail sidebar, navigation history -
//! shared by every family's cheat sheet binary.
//!
//! [`run_multi`] multiplexes several families in one window with a
//! header dropdown switcher; families enter as
//! [`Rc<dyn DynCheatSheetFamily>`](DynCheatSheetFamily) values via
//! [`FamilyAdapter`].

mod app;
mod detail;
mod detail_context;
mod dyn_family;
mod family;
mod family_adapter;
mod family_kind;
mod tone_picker;
mod xml_highlight;

pub use app::run;
pub use app::run_multi;
pub use dyn_family::DynCheatSheetFamily;
pub use family::CheatSheetFamily;
pub use family_adapter::FamilyAdapter;
pub use family_kind::FamilyKind;
