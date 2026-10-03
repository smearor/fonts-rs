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
/// A global default index ([`TonePicker::set_default`]) applies to
/// every tone group without a per-group selection - that powers the
/// always-visible tone row in the browse sidebar while per-glyph
/// overrides remain possible from the detail page.
///
/// `refresh` is wired by the app once the grid closures exist - picking
/// or resetting a tone re-renders so the swap applies immediately.
#[derive(Clone)]
pub struct TonePicker {
    /// Selected alternate index per `tone_group` id.
    pub(super) selections: Rc<RefCell<HashMap<u32, usize>>>,
    /// Global default alternate index for all groups without a
    /// per-group selection.
    default_index: Rc<RefCell<Option<usize>>>,
    /// Re-render the grid (recompute icons + populate) after a change.
    refresh: RefreshSlot,
}

impl TonePicker {
    /// Creates an empty picker state; call [`TonePicker::set_refresh`]
    /// once the grid closures are built.
    pub fn new() -> Self {
        Self {
            selections: Rc::new(RefCell::new(HashMap::new())),
            default_index: Rc::new(RefCell::new(None)),
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

    /// Stores `index` as the global default alternate position applied
    /// to every tone group without a per-group selection, and
    /// re-renders the grid.
    pub fn set_default(&self, index: usize) {
        *self.default_index.borrow_mut() = Some(index);
        self.refresh();
    }

    /// Returns the global default alternate index, if set.
    pub fn default(&self) -> Option<usize> {
        *self.default_index.borrow()
    }

    /// Clears all tone selections (per-group and global default) and
    /// re-renders the grid.
    pub fn reset_all(&self) {
        self.selections.borrow_mut().clear();
        *self.default_index.borrow_mut() = None;
        self.refresh();
    }

    /// Returns the effective alternate index for `group`: the per-group
    /// selection if present, otherwise the global default.
    pub fn selected(&self, group: u32) -> Option<usize> {
        self.selections.borrow().get(&group).copied().or_else(|| self.default())
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
