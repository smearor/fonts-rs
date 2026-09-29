//! Default-tone state for emoji-style variant pickers.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Re-render callback invoked after a tone selection change.
type RefreshFn = Rc<dyn Fn()>;

/// Shared slot for the refresh callback, wired once the grid closures exist.
type RefreshSlot = Rc<RefCell<Option<RefreshFn>>>;

/// Session-scoped default-tone selection for tone-variant glyph groups.
///
/// Maps a [`GlyphOrdering::tone_group`](fonts_rs_model::GlyphOrdering::tone_group)
/// id to the chosen position inside that group's `alternates` list. The
/// index transfers to every base glyph sharing the group because entries
/// with the same `tone_group` have the same alternate shape.
///
/// `refresh` is wired by the app once the grid closures exist - picking
/// or resetting a tone re-renders so the swap applies immediately.
#[derive(Clone)]
pub struct TonePicker {
    /// Selected alternate index per `tone_group` id.
    pub(super) selections: Rc<RefCell<HashMap<u32, usize>>>,
    /// Re-render the grid (recompute icons + populate) after a change.
    refresh: RefreshSlot,
}

impl TonePicker {
    /// Creates an empty picker state; call [`TonePicker::set_refresh`]
    /// once the grid closures are built.
    pub fn new() -> Self {
        Self {
            selections: Rc::new(RefCell::new(HashMap::new())),
            refresh: Rc::new(RefCell::new(None)),
        }
    }

    /// Installs the callback invoked after `select`/`reset`.
    pub fn set_refresh(&self, refresh: RefreshFn) {
        *self.refresh.borrow_mut() = Some(refresh);
    }

    /// Remembers `index` as the default alternate position for `group`
    /// and re-renders the grid.
    pub fn select(&self, group: u32, index: usize) {
        self.selections.borrow_mut().insert(group, index);
        self.refresh();
    }

    /// Clears the default tone for `group` and re-renders the grid.
    pub fn reset(&self, group: u32) {
        self.selections.borrow_mut().remove(&group);
        self.refresh();
    }

    /// Returns the stored alternate index for `group`, if any.
    pub fn selected(&self, group: u32) -> Option<usize> {
        self.selections.borrow().get(&group).copied()
    }

    /// Runs the installed refresh callback, if set.
    fn refresh(&self) {
        if let Some(refresh) = self.refresh.borrow().as_ref() {
            refresh();
        }
    }
}

impl Default for TonePicker {
    fn default() -> Self {
        Self::new()
    }
}
