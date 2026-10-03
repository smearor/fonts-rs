//! Shared UI handles for rendering the detail page.

use crate::tone_picker::TonePicker;
use gtk4::Box;
use gtk4::Entry;
use gtk4::Stack;
use libadwaita as adw;

/// Handles a detail view needs beyond the glyph itself.
///
/// Bundles the detail container, the search entry (keyword/category
/// buttons push their term into it), the default-tone picker state and
/// the split-view handles for pushing the detail page so
/// [`crate::detail::show_detail`] and history navigation stay at a few
/// parameters.
#[derive(Clone)]
pub(super) struct DetailContext {
    /// Container that receives the detail widgets.
    pub content: Box,
    /// Search entry - keyword/category buttons set their term here.
    pub search_entry: Entry,
    /// Session-scoped default-tone selections.
    pub tone_picker: TonePicker,
    /// Split view that swaps between browse and detail page.
    pub split_view: adw::NavigationSplitView,
    /// Stack inside the detail page: `empty` placeholder vs `detail`.
    pub detail_stack: Stack,
    /// Overlay for copy-feedback toasts.
    pub toasts: adw::ToastOverlay,
    /// Category sidebar inside the browse page; category-pill clicks
    /// reveal it when collapsed. `None` for families without a
    /// categories/groups axis.
    pub category_split: Option<adw::OverlaySplitView>,
}
