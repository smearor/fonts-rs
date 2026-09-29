//! Shared UI handles for rendering the detail sidebar.

use crate::tone_picker::TonePicker;
use gtk4::Box;
use gtk4::Entry;

/// Handles a detail view needs beyond the glyph itself.
///
/// Bundles the sidebar container, the search entry (keyword/category
/// buttons push their term into it) and the default-tone picker state
/// so [`crate::detail::show_detail`] and history navigation stay at
/// a few parameters.
#[derive(Clone)]
pub(super) struct DetailContext {
    /// Sidebar container that receives the detail widgets.
    pub content: Box,
    /// Search entry - keyword/category buttons set their term here.
    pub search_entry: Entry,
    /// Session-scoped default-tone selections.
    pub tone_picker: TonePicker,
}
