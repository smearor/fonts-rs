//! The combined cross-family glyph grid - search entry, family filter
//! chips and a `FlowBox` of selectable icon cells.

use crate::app::margins;
use fonts_rs_cheat_sheet::DynCheatSheetFamily;
use fonts_rs_iconset_builder::ICON_SOURCES;
use fonts_rs_iconset_builder::IconSelection;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

/// Cells rendered per family before filtering kicks in - keeps the
/// combined grid responsive over ~20k glyphs.
const MAX_CELLS_PER_FAMILY: usize = 600;

/// Filter state shared between the search entry, chips and the
/// FlowBox filter func.
struct FilterState {
    /// Case-insensitive substring on the grid icon name.
    query: String,
    /// `ICON_SOURCES` indices currently enabled in the chips row.
    families: BTreeSet<usize>,
}

/// The left pane: browse every icon-capable family in one grid.
pub struct GlyphGrid {
    /// Root widget (search + chips + scrolled flowbox).
    widget: gtk4::Box,
}

impl GlyphGrid {
    /// Builds the grid over `families` (same order as `ICON_SOURCES`).
    /// `on_change` fires whenever the selection model changes so the
    /// sidebar can rebuild.
    pub fn new(families: Vec<Rc<dyn DynCheatSheetFamily>>, selection: Rc<RefCell<IconSelection>>, on_change: impl Fn() + 'static) -> Rc<Self> {
        let filter = Rc::new(RefCell::new(FilterState {
            query: String::new(),
            families: (0..ICON_SOURCES.len()).collect(),
        }));
        let on_change = Rc::new(on_change);

        let search = gtk4::SearchEntry::builder().placeholder_text("Search all families…").build();
        let chips = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        let flowbox = gtk4::FlowBox::new();
        flowbox.set_selection_mode(gtk4::SelectionMode::None);
        flowbox.set_min_children_per_line(4);
        flowbox.set_max_children_per_line(8);
        flowbox.set_row_spacing(8);
        flowbox.set_column_spacing(8);
        flowbox.set_homogeneous(true);
        margins(&flowbox, 12);

        // One cell per glyph (capped); cell state mirrors the shared
        // selection so toggles survive filtering. `family_index` aligns
        // 1:1 with `ICON_SOURCES`; the family's `upstream_icon` reports
        // subdirectory/stem/post-processors for non-conforming names
        // (skipped silently rather than emitted wrong).
        for (family_index, family) in families.iter().enumerate() {
            let mut shown = 0usize;
            for (codepoint, icon_name) in family.all_glyphs() {
                let Some(upstream) = (ICON_SOURCES[family_index].upstream_icon)(&icon_name) else {
                    continue;
                };
                let cell = build_cell(family.clone(), icon_name.clone(), codepoint, upstream.stem, selection.clone(), on_change.clone());
                set_meta(&cell.widget(), family_index, &icon_name);
                flowbox.insert(&cell.widget(), -1);
                shown += 1;
                if shown >= MAX_CELLS_PER_FAMILY {
                    break;
                }
            }
        }

        flowbox.set_filter_func({
            let filter = filter.clone();
            move |child| {
                let Some(widget) = child.child() else {
                    return false;
                };
                let state = filter.borrow();
                state.families.contains(&meta_index(&widget)) && (state.query.is_empty() || meta_icon_name(&widget).contains(&state.query))
            }
        });
        search.connect_search_changed({
            let filter = filter.clone();
            let flowbox = flowbox.clone();
            move |entry| {
                filter.borrow_mut().query = entry.text().to_string();
                flowbox.invalidate_filter();
            }
        });

        // Family chips - one per cheat-sheet family (a family can feed
        // several `ICON_SOURCES` entries, e.g. Eva fill/outline).
        margins(&chips, 8);
        chips.append(&gtk4::Label::new(Some("Families:")));
        for (index, family) in families.iter().enumerate() {
            let label = family.title().split(" - ").next().unwrap_or_else(|| family.title());
            let chip = gtk4::ToggleButton::builder().label(label).active(true).build();
            chip.connect_toggled({
                let filter = filter.clone();
                let flowbox = flowbox.clone();
                move |button| {
                    if button.is_active() {
                        filter.borrow_mut().families.insert(index);
                    } else {
                        filter.borrow_mut().families.remove(&index);
                    }
                    flowbox.invalidate_filter();
                }
            });
            chips.append(&chip);
        }

        let scrolled = gtk4::ScrolledWindow::builder().vexpand(true).child(&flowbox).build();
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        widget.append(&search);
        widget.append(&chips);
        widget.append(&scrolled);
        Rc::new(Self { widget })
    }

    /// The pane's root widget.
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
}

/// One selectable glyph cell - a toggle showing the rendered icon plus
/// its upstream stem.
struct GlyphCell {
    /// The toggle button (FlowBoxChild wraps it).
    button: gtk4::ToggleButton,
}

impl GlyphCell {
    /// The widget inserted into the FlowBox.
    fn widget(&self) -> gtk4::ToggleButton {
        self.button.clone()
    }
}

/// Builds a cell wired into the selection model.
fn build_cell(
    family: Rc<dyn DynCheatSheetFamily>,
    icon_name: String,
    codepoint: fonts_rs_model::CodePoint,
    upstream: String,
    selection: Rc<RefCell<IconSelection>>,
    on_change: Rc<dyn Fn()>,
) -> GlyphCell {
    // Mirror the cheat sheet's cell rendering: icon-theme families
    // (Lucide, Fluent, …) resolve a registered SVG icon; font families
    // (Eva, Tabler) render the codepoint with the family's CSS class.
    let icon: gtk4::Widget = if let Some(name) = family.glyph_icon_name(&icon_name) {
        gtk4::Image::builder()
            .icon_name(name)
            .pixel_size(32)
            .halign(gtk4::Align::Center)
            .build()
            .upcast()
    } else {
        let mut classes = vec!["builder-glyph"];
        if let Some(class) = family.glyph_css_class() {
            classes.insert(0, class);
        }
        gtk4::Label::builder()
            .label(codepoint.as_char().to_string())
            .css_classes(classes)
            .halign(gtk4::Align::Center)
            .build()
            .upcast()
    };
    let label = gtk4::Label::builder()
        .label(&upstream)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(14)
        .build();
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    content.append(&icon);
    content.append(&label);
    let button = gtk4::ToggleButton::builder().child(&content).build();
    button.set_active(selection.borrow().contains(&icon_name));
    button.connect_toggled(move |button| {
        let mut selection = selection.borrow_mut();
        if button.is_active() {
            selection.add(&icon_name);
        } else {
            selection.remove_name(&icon_name);
        }
        drop(selection);
        on_change();
    });
    GlyphCell { button }
}

/// Filter metadata stored on each cell widget via `Widget::set_data` -
/// the FlowBox child wrapper does not expose application state.
fn set_meta(widget: &impl IsA<gtk4::Widget>, index: usize, icon_name: &str) {
    unsafe {
        widget.set_data("iconset-source-index", index);
        widget.set_data("iconset-icon-name", icon_name.to_string());
    }
}

/// The tagged `ICON_SOURCES` index of a cell widget (0 when unset).
fn meta_index(widget: &impl IsA<gtk4::Widget>) -> usize {
    unsafe { widget.data::<usize>("iconset-source-index").map(|data| *data.as_ref()).unwrap_or_default() }
}

/// The tagged grid icon name of a cell widget.
fn meta_icon_name(widget: &impl IsA<gtk4::Widget>) -> String {
    unsafe { widget.data::<String>("iconset-icon-name").map(|data| data.as_ref().clone()).unwrap_or_default() }
}
