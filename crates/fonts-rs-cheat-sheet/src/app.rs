//! Generic cheat sheet application: grid, search, history, sidebar.

use crate::detail::show_detail;
use crate::detail_context::DetailContext;
use crate::family::CheatSheetFamily;
use crate::tone_picker::TonePicker;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GroupLabel;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::DropDown;
use gtk4::Entry;
use gtk4::FlowBox;
use gtk4::HeaderBar;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::ScrolledWindow;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;
use std::cell::RefCell;
use std::rc::Rc;

/// Shared list of currently displayed glyphs.
type GlyphList<F> = Rc<RefCell<Vec<(CodePoint, <F as CheatSheetFamily>::Name)>>>;

/// A navigable history entry: either a search query or a glyph detail view.
#[derive(Clone, PartialEq, Eq)]
enum HistoryEntry<N> {
    /// A search query that filters the grid.
    Search(String),
    /// A glyph detail view (name + codepoint).
    Icon(N, CodePoint),
}

/// Update sensitivity of Back/Forward buttons based on current history position.
fn update_nav_buttons<N>(back_btn: &Button, forward_btn: &Button, history: &[HistoryEntry<N>], index: usize) {
    back_btn.set_sensitive(index > 0);
    forward_btn.set_sensitive(index + 1 < history.len());
}

/// Navigate to a history entry: search filters the grid, glyph opens the sidebar.
fn navigate_to<F: CheatSheetFamily>(
    entry: &HistoryEntry<F::Name>,
    ctx: &DetailContext,
    sidebar: &Box,
    selected_name: &Rc<RefCell<Option<F::Name>>>,
    selected_cell: &Rc<RefCell<Option<gtk4::Box>>>,
    window: &ApplicationWindow,
) {
    // Clear previous selection.
    if let Some(old) = selected_cell.borrow_mut().take() {
        old.remove_css_class("cheat-cell-selected");
    }
    match entry {
        HistoryEntry::Search(filter) => {
            ctx.search_entry.set_text(filter);
            ctx.search_entry.set_position(-1);
            *selected_name.borrow_mut() = None;
            sidebar.set_visible(false);
            window.set_default_size(460, 800);
            window.set_size_request(460, -1);
        }
        HistoryEntry::Icon(name, cp) => {
            *selected_name.borrow_mut() = Some(name.clone());
            sidebar.set_visible(true);
            window.set_default_size(784, 800);
            window.set_size_request(784, -1);
            show_detail::<F>(ctx, name, *cp);
        }
    }
}

/// Runs the cheat sheet application for family `F`.
///
/// Initializes GTK, calls [`CheatSheetFamily::init`], then builds the
/// UI on application activation.
///
/// # Errors
///
/// Returns an error if GTK or family initialization fails.
pub fn run<F: CheatSheetFamily>() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    F::init()?;

    let app = Application::builder().application_id(F::APP_ID).build();
    app.connect_activate(build_ui::<F>);
    Ok(app.run())
}

fn build_ui<F: CheatSheetFamily>(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(F::TITLE)
        .default_width(1280)
        .default_height(800)
        .width_request(460)
        .build();

    window.connect_close_request({
        let app = app.clone();
        move |_| {
            app.quit();
            glib::Propagation::Proceed
        }
    });

    // Global CSS.
    let css = r#"
        .cheat-icon-label { font-size: 42px; }
        .cheat-cell { padding: 10px; border-radius: 8px; min-width: 180px; }
        .cheat-cell:hover { background-color: rgba(100, 180, 255, 0.15); }
        .cheat-cell-selected { background-color: rgba(100, 180, 255, 0.30); }
        .cheat-meta-flow { background: transparent; }
        .cheat-meta-flow flowboxchild { background: transparent; padding: 0; }
        .cheat-name, .cheat-cp { -gtk-user-select: text; }
        .cheat-meta { font-size: 9px; opacity: 0.7; }
        button.cat-btn { background: rgba(76, 175, 80, 0.3); border-radius: 12px; padding: 2px 8px; font-size: 10px; border: none; box-shadow: none; }
        button.cat-btn:hover { background: rgba(76, 175, 80, 0.5); }
        button.kw-btn { background: rgba(255, 152, 0, 0.3); border-radius: 12px; padding: 2px 8px; font-size: 10px; border: none; box-shadow: none; }
        button.kw-btn:hover { background: rgba(255, 152, 0, 0.5); }
        .sidebar { background-color: @theme_bg_color; max-width: 300px; }
        .sidebar-title { font-size: 16px; font-weight: bold; }
        .sidebar-subtitle { font-size: 12px; opacity: 0.7; }
        .sidebar-codepoint-char { font-size: 96px; }
        .sidebar-codepoint-hex { font-size: 14px; }
        .copyable { -gtk-user-select: text; }
    "#;
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    let all_icons = F::all_glyphs();
    let total = all_icons.len();

    // Canonical ordering contract (e.g. emoji picker ordering). `None`
    // for families without a curated order - keeps the alphabetical grid.
    let ordering = F::ordering();
    let group_labels: &'static [GroupLabel] = ordering.map(|o| o.group_labels()).unwrap_or_default();
    let active_group: Rc<RefCell<Option<GroupLabel>>> = Rc::new(RefCell::new(None));

    // Default-tone picker state (tone_group id -> alternate index);
    // the browse grid swaps base cells to the chosen tone. Session
    // scoped - persistence is a follow-up.
    let tone_picker = TonePicker::new();

    // --- HeaderBar with Back/Forward history navigation ---
    let header_bar = HeaderBar::builder()
        .title_widget(&gtk4::Label::new(Some(F::TITLE)))
        .show_title_buttons(true)
        .build();

    let back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .sensitive(false)
        .build();
    let forward_btn = Button::builder().icon_name("go-next-symbolic").tooltip_text("Forward").sensitive(false).build();
    header_bar.pack_start(&back_btn);
    header_bar.pack_start(&forward_btn);

    // --- Navigation history state ---
    let history: Rc<RefCell<Vec<HistoryEntry<F::Name>>>> = Rc::new(RefCell::new(Vec::new()));
    let history_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let suppress_history: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

    // --- Top bar: title, count, search, size slider ---
    let top_bar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(12)
        .margin_bottom(8)
        .margin_start(12)
        .margin_end(12)
        .build();

    let count_bar = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Fill)
        .margin_top(2)
        .margin_bottom(2)
        .build();
    let count_label = Label::builder()
        .label(format!("{} {} available", total, F::NOUN))
        .css_classes(["dim-label"])
        .halign(Align::Start)
        .build();
    let total_label = Label::builder()
        .label(format!("{} total", total))
        .css_classes(["dim-label"])
        .halign(Align::End)
        .hexpand(true)
        .build();
    count_bar.append(&count_label);
    count_bar.append(&total_label);

    // Group jump-list: a dropdown of canonical group labels. Only
    // present when the family exposes ordering groups. The
    // `connect_selected_notify` is wired later, once `render_grid` and
    // `refresh_grid` exist.
    let mut group_dropdown: Option<DropDown> = None;
    if !group_labels.is_empty() {
        let mut items = vec!["All groups".to_string()];
        items.extend(group_labels.iter().map(|label| label.as_str().to_string()));
        let refs: Vec<&str> = items.iter().map(String::as_str).collect();
        let dropdown = DropDown::from_strings(&refs);
        dropdown.set_tooltip_text(Some("Jump to group"));
        count_bar.append(&dropdown);
        group_dropdown = Some(dropdown);
    }
    top_bar.append(&count_bar);

    let search_entry = Entry::builder()
        .placeholder_text(F::SEARCH_PLACEHOLDER)
        .halign(Align::Fill)
        .margin_top(4)
        .margin_bottom(4)
        .primary_icon_name("edit-find-symbolic")
        .primary_icon_activatable(false)
        .build();
    search_entry.connect_icon_release(|entry, icon| {
        if icon == gtk4::EntryIconPosition::Secondary {
            entry.set_text("");
        }
    });
    // Show clear button only when there is text.
    {
        let search_entry_for_icon = search_entry.clone();
        search_entry.connect_changed(move |entry| {
            if entry.text().is_empty() {
                search_entry_for_icon.set_secondary_icon_name(None);
            } else {
                search_entry_for_icon.set_secondary_icon_name(Some("edit-clear-symbolic"));
            }
        });
    }
    top_bar.append(&search_entry);

    // Icon size slider.
    let size_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Center)
        .margin_bottom(4)
        .build();
    let size_label = Label::builder().label("Icon size:").build();
    let size_value = Label::builder().label("42px").css_classes(["monospace"]).build();
    let size_slider = Scale::builder()
        .orientation(Orientation::Horizontal)
        .adjustment(&gtk4::Adjustment::new(42.0, 16.0, 128.0, 2.0, 8.0, 0.0))
        .width_request(200)
        .halign(Align::Center)
        .draw_value(false)
        .build();
    size_slider.add_mark(16.0, gtk4::PositionType::Bottom, Some("16"));
    size_slider.add_mark(32.0, gtk4::PositionType::Bottom, Some("32"));
    size_slider.add_mark(64.0, gtk4::PositionType::Bottom, Some("64"));
    size_slider.add_mark(128.0, gtk4::PositionType::Bottom, Some("128"));
    size_box.append(&size_label);
    size_box.append(&size_slider);
    size_box.append(&size_value);
    top_bar.append(&size_box);

    // --- Main layout: horizontal Box (grid | sidebar) ---
    let main_box = Box::builder().orientation(Orientation::Horizontal).vexpand(true).hexpand(true).build();
    // --- Left: scrolled icon grid ---
    let left_box = Box::builder().orientation(Orientation::Vertical).spacing(4).hexpand(true).build();
    left_box.append(&top_bar);

    let scrolled = ScrolledWindow::builder().hexpand(true).vexpand(true).build();

    let grid = FlowBox::builder()
        .column_spacing(8)
        .row_spacing(8)
        .halign(Align::Fill)
        .hexpand(true)
        .margin_top(8)
        .margin_bottom(8)
        .selection_mode(gtk4::SelectionMode::None)
        .max_children_per_line(20)
        .min_children_per_line(2)
        .build();

    scrolled.set_child(Some(&grid));
    left_box.append(&scrolled);
    main_box.append(&left_box);

    // --- Right: detail sidebar (initially hidden) ---
    let sidebar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .css_classes(["sidebar"])
        .width_request(300)
        .build();

    // Sidebar header with close button.
    let sidebar_header = Box::builder().orientation(Orientation::Horizontal).halign(Align::End).build();
    let close_btn = Button::builder()
        .icon_name("window-close-symbolic")
        .css_classes(["flat"])
        .tooltip_text("Close sidebar")
        .build();
    sidebar_header.append(&close_btn);
    sidebar.append(&sidebar_header);

    // Sidebar content area (cleared/repopulated on icon click).
    let sidebar_content = Box::builder().orientation(Orientation::Vertical).spacing(12).vexpand(true).build();
    sidebar.append(&sidebar_content);

    // Initially hidden.
    sidebar.set_visible(false);
    main_box.append(&sidebar);

    // --- Shared selection state ---
    let selected_name: Rc<RefCell<Option<F::Name>>> = Rc::new(RefCell::new(None));
    let selected_cell: Rc<RefCell<Option<gtk4::Box>>> = Rc::new(RefCell::new(None));

    // --- Close button handler ---
    {
        let sidebar_for_close = sidebar.clone();
        let content_for_close = sidebar_content.clone();
        let selected_name_for_close = selected_name.clone();
        let selected_cell_for_close = selected_cell.clone();
        let window_for_close = window.clone();
        close_btn.connect_clicked(move |_| {
            if let Some(cell) = selected_cell_for_close.borrow_mut().take() {
                cell.remove_css_class("cheat-cell-selected");
            }
            selected_name_for_close.borrow_mut().take();
            while let Some(child) = content_for_close.first_child() {
                content_for_close.remove(&child);
            }
            sidebar_for_close.set_visible(false);
            window_for_close.set_default_size(460, 800);
            window_for_close.set_size_request(460, -1);
        });
    }

    // --- Grid population ---
    const BATCH_SIZE: usize = 100;

    let current_icons: GlyphList<F> = Rc::new(RefCell::new(Vec::new()));
    let displayed_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let current_icon_size: Rc<RefCell<f64>> = Rc::new(RefCell::new(42.0));

    let icons_for_filter = all_icons.clone();
    let render_grid = {
        let active_group = active_group.clone();
        let tone_picker = tone_picker.clone();
        move |filter: &str| -> Vec<(CodePoint, F::Name)> {
            // Group jump-list filter: restrict the glyph set to the
            // selected group (None = all groups / ordering disabled).
            let group = *active_group.borrow();
            let in_group = |name: &F::Name| group.is_none_or(|label| ordering.is_some_and(|o| o.group(name.as_ref()) == Some(label)));
            // Variant collapsing: browsing shows base glyphs only; the
            // search path keeps variants reachable.
            let collapse = ordering.is_some_and(|o| o.collapse_variants());
            let is_variant = |name: &F::Name| ordering.is_some_and(|o| o.base_of(name.as_ref()).is_some());
            let base: Vec<(CodePoint, F::Name)> = icons_for_filter
                .iter()
                .filter(|(_, name)| in_group(name) && !(collapse && is_variant(name)))
                .cloned()
                .collect();

            // Default-tone swap (browse path only): a base cell whose
            // tone group has a stored index renders that alternate.
            // Presentation-only - the search path shows real glyphs.
            let swap_tone = |(cp, name): (CodePoint, F::Name)| -> (CodePoint, F::Name) {
                let Some(group) = ordering.and_then(|o| o.tone_group(name.as_ref())) else {
                    return (cp, name);
                };
                let Some(index) = tone_picker.selected(group) else {
                    return (cp, name);
                };
                let Some(&variant) = ordering.and_then(|o| o.alternates(name.as_ref()).get(index)) else {
                    return (cp, name);
                };
                let (Some(variant_name), Some(variant_cp)) = (F::parse_name(variant), F::codepoint(variant)) else {
                    return (cp, name);
                };
                (variant_cp, variant_name)
            };

            let filter_lower = filter.to_lowercase();
            if filter_lower.is_empty() {
                return base.into_iter().map(swap_tone).collect();
            }

            let search_results = F::search(filter);
            let mut filtered: Vec<(CodePoint, F::Name)> = search_results
                .iter()
                .filter_map(|name| {
                    let icon_name = F::parse_name(name)?;
                    if !in_group(&icon_name) {
                        return None;
                    }
                    let cp = F::codepoint(name)?;
                    Some((cp, icon_name))
                })
                .collect();

            // Name/hex supplement over the full group-filtered set -
            // variants must stay searchable even while collapsed.
            for (cp, name) in icons_for_filter.iter() {
                if !in_group(name) {
                    continue;
                }
                let hex = format!("{:X}", cp.as_char() as u32);
                let char_str = cp.as_char().to_string();
                if (hex.to_lowercase().contains(&filter_lower)
                    || format!("\\u{{{:X}}}", cp.as_char() as u32).to_lowercase().contains(&filter_lower)
                    || char_str.contains(filter)
                    || name.as_ref().to_lowercase().contains(&filter_lower))
                    && !filtered.iter().any(|(_, n)| n == name)
                {
                    filtered.push((*cp, name.clone()));
                }
            }

            // Sort key: canonical ordering position (variants fall back
            // to their base's index so they sort next to it), then name.
            let sort_key = |name: &F::Name| {
                ordering.and_then(|o| {
                    o.order_index(name.as_ref())
                        .or_else(|| o.base_of(name.as_ref()).and_then(|base| o.order_index(base)))
                })
            };
            filtered.sort_by(|a, b| {
                sort_key(&a.1)
                    .unwrap_or(u32::MAX)
                    .cmp(&sort_key(&b.1).unwrap_or(u32::MAX))
                    .then_with(|| a.1.cmp(&b.1))
            });
            filtered
        }
    };

    let grid_clone = grid.clone();
    let sidebar_clone = sidebar.clone();
    let search_entry_clone = search_entry.clone();
    // Shared handles for the detail sidebar + tone picking.
    let detail_ctx = DetailContext {
        content: sidebar_content.clone(),
        search_entry: search_entry.clone(),
        tone_picker: tone_picker.clone(),
    };
    let selected_name_for_grid = selected_name.clone();
    let selected_cell_for_grid = selected_cell.clone();
    let history_for_grid = history.clone();
    let history_index_for_grid = history_index.clone();
    let suppress_for_grid = suppress_history.clone();
    let back_btn_for_grid = back_btn.clone();
    let forward_btn_for_grid = forward_btn.clone();
    let window_for_grid = window.clone();
    let detail_ctx_for_grid = detail_ctx.clone();

    let populate_batch = move |start: usize, end: usize, icons: &[(CodePoint, F::Name)], icon_size: f64, _cols: u32| {
        if start == 0 {
            while let Some(child) = grid_clone.first_child() {
                grid_clone.remove(&child);
            }
            let size_css = format!(".cheat-icon-label {{ font-size: {:.0}px; }}", icon_size);
            let size_provider = gtk4::CssProvider::new();
            size_provider.load_from_string(&size_css);
            if let Some(display) = gtk4::gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(&display, &size_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER + 1);
            }
        }

        for (codepoint, name) in icons[start..end].iter() {
            let display_name = F::display_name(name);
            let hex = format!("\\u{{{:X}}}", codepoint.as_char() as u32);

            // Families with icon-theme glyphs (e.g. emoji) render the
            // registered SVG; others render the codepoint as text.
            let icon: gtk4::Widget = if let Some(icon_name) = F::glyph_icon_name(name) {
                gtk4::Image::builder()
                    .icon_name(&icon_name)
                    .pixel_size(icon_size as i32)
                    .halign(Align::Center)
                    .build()
                    .upcast()
            } else {
                let mut icon_classes = vec!["cheat-icon-label"];
                if let Some(class) = F::GLYPH_CSS_CLASS {
                    icon_classes.insert(0, class);
                }
                Label::builder()
                    .label(codepoint.as_char().to_string())
                    .css_classes(icon_classes)
                    .halign(Align::Center)
                    .build()
                    .upcast()
            };

            let name_label = Label::builder()
                .label(&display_name)
                .css_classes(["caption", "dim-label", "cheat-name"])
                .wrap(true)
                .max_width_chars(18)
                .halign(Align::Center)
                .selectable(true)
                .build();

            let codepoint_label = Label::builder()
                .label(&hex)
                .css_classes(["monospace", "caption", "cheat-cp"])
                .halign(Align::Center)
                .selectable(true)
                .build();

            // Metadata buttons: keywords (orange) and categories (green).
            let meta_box = Box::builder().orientation(Orientation::Vertical).spacing(2).halign(Align::Fill).build();

            let keywords = F::keywords(name);
            let categories = F::categories(name);

            // Category buttons (green) - wrapped via FlowBox.
            if !categories.is_empty() {
                let cat_flow = FlowBox::builder()
                    .orientation(Orientation::Horizontal)
                    .selection_mode(gtk4::SelectionMode::None)
                    .column_spacing(4)
                    .row_spacing(2)
                    .halign(Align::Center)
                    .max_children_per_line(4)
                    .css_classes(["cheat-meta-flow"])
                    .build();
                for cat_label in categories.iter().take(5) {
                    let btn = Button::builder().label(cat_label).css_classes(["cat-btn"]).build();
                    let search_entry_ref = search_entry_clone.clone();
                    let cat_label = cat_label.clone();
                    btn.connect_clicked(move |_| {
                        search_entry_ref.set_text(&cat_label);
                    });
                    cat_flow.insert(&btn, -1);
                }
                meta_box.append(&cat_flow);
            }

            // Keyword buttons (orange) - wrapped via FlowBox.
            if !keywords.is_empty() {
                let kw_flow = FlowBox::builder()
                    .orientation(Orientation::Horizontal)
                    .selection_mode(gtk4::SelectionMode::None)
                    .column_spacing(4)
                    .row_spacing(2)
                    .halign(Align::Center)
                    .max_children_per_line(4)
                    .css_classes(["cheat-meta-flow"])
                    .build();
                for kw_label in keywords.iter().take(5) {
                    let btn = Button::builder().label(kw_label).css_classes(["kw-btn"]).build();
                    let search_entry_ref = search_entry_clone.clone();
                    let kw_label = kw_label.clone();
                    btn.connect_clicked(move |_| {
                        search_entry_ref.set_text(&kw_label);
                    });
                    kw_flow.insert(&btn, -1);
                }
                meta_box.append(&kw_flow);
            }

            let vbox = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .halign(Align::Center)
                .css_classes(["cheat-cell"])
                .build();

            // Apply selection highlight if this icon is selected.
            let is_selected = selected_name_for_grid.borrow().as_ref().map(|n| n == name).unwrap_or(false);
            if is_selected {
                vbox.add_css_class("cheat-cell-selected");
                *selected_cell_for_grid.borrow_mut() = Some(vbox.clone());
            }

            vbox.append(&icon);
            vbox.append(&name_label);
            vbox.append(&codepoint_label);
            if !categories.is_empty() || !keywords.is_empty() {
                vbox.append(&meta_box);
            }

            // Click on card -> open detail sidebar (but not on buttons).
            let name_for_click = name.clone();
            let cp_for_click = *codepoint;
            let sidebar_ref = sidebar_clone.clone();
            let vbox_ref = vbox.clone();
            let selected_name_ref = selected_name_for_grid.clone();
            let selected_cell_ref = selected_cell_for_grid.clone();
            let history_ref = history_for_grid.clone();
            let history_index_ref = history_index_for_grid.clone();
            let suppress_ref = suppress_for_grid.clone();
            let back_btn_ref = back_btn_for_grid.clone();
            let forward_btn_ref = forward_btn_for_grid.clone();
            let window_ref = window_for_grid.clone();
            let detail_ctx_ref = detail_ctx_for_grid.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.connect_released(move |gesture, _n, x, y| {
                if let Some(widget) = gesture.widget()
                    && let Some(target) = widget.pick(x, y, gtk4::PickFlags::empty())
                {
                    let mut current = Some(target);
                    while let Some(w) = current {
                        if w.is::<gtk4::Button>() {
                            return;
                        }
                        current = w.parent();
                    }
                }
                // Toggle: if already selected, close sidebar.
                let already_selected = selected_name_ref.borrow().as_ref().map(|n| n == &name_for_click).unwrap_or(false);
                if already_selected {
                    if let Some(old) = selected_cell_ref.borrow_mut().take() {
                        old.remove_css_class("cheat-cell-selected");
                    }
                    *selected_name_ref.borrow_mut() = None;
                    sidebar_ref.set_visible(false);
                    window_ref.set_default_size(460, 800);
                    window_ref.set_size_request(460, -1);
                    return;
                }
                if let Some(old) = selected_cell_ref.borrow_mut().take() {
                    old.remove_css_class("cheat-cell-selected");
                }
                *selected_name_ref.borrow_mut() = Some(name_for_click.clone());
                vbox_ref.add_css_class("cheat-cell-selected");
                *selected_cell_ref.borrow_mut() = Some(vbox_ref.clone());
                sidebar_ref.set_visible(true);
                window_ref.set_default_size(784, 800);
                window_ref.set_size_request(784, -1);
                show_detail::<F>(&detail_ctx_ref, &name_for_click, cp_for_click);

                // Record icon view in history.
                if !*suppress_ref.borrow() {
                    let entry = HistoryEntry::Icon(name_for_click.clone(), cp_for_click);
                    let mut hist = history_ref.borrow_mut();
                    let mut idx = history_index_ref.borrow_mut();
                    if hist.get(*idx).is_some_and(|h| h == &entry) {
                        return;
                    }
                    hist.truncate(*idx + 1);
                    hist.push(entry);
                    *idx = hist.len() - 1;
                    update_nav_buttons(&back_btn_ref, &forward_btn_ref, &hist, *idx);
                }
            });
            vbox.add_controller(gesture);

            grid_clone.insert(&vbox, -1);
        }
    };

    // refresh_grid: resets (clear + first batch) or loads more.
    let refresh_grid = {
        let current_icons_ref = current_icons.clone();
        let displayed_count_ref = displayed_count.clone();
        let size_ref = current_icon_size.clone();
        let populate_ref = populate_batch.clone();
        let count_label_ref = count_label.clone();
        move |reset: bool| {
            let icons = current_icons_ref.borrow().clone();
            let total_icons = icons.len();
            if reset {
                let count = BATCH_SIZE.min(total_icons);
                *displayed_count_ref.borrow_mut() = count;
                let size = *size_ref.borrow();
                populate_ref(0, count, &icons, size, 0);
            } else {
                let start = *displayed_count_ref.borrow();
                if start >= total_icons {
                    return;
                }
                let end = (start + BATCH_SIZE).min(total_icons);
                let size = *size_ref.borrow();
                populate_ref(start, end, &icons, size, 0);
                *displayed_count_ref.borrow_mut() = end;
            }
            let shown = *displayed_count_ref.borrow();
            let label = if shown >= total_icons {
                format!("{} / {} {}", total_icons, total_icons, F::NOUN)
            } else {
                format!("{} / {} {} (scroll for more)", shown, total_icons, F::NOUN)
            };
            count_label_ref.set_label(&label);
        }
    };

    // Tone picking re-renders the grid so cell swaps apply
    // immediately - recompute the filtered list, then repopulate.
    {
        let render_grid_for_tones = render_grid.clone();
        let current_icons_for_tones = current_icons.clone();
        let refresh_grid_for_tones = refresh_grid.clone();
        let search_entry_for_tones = search_entry.clone();
        tone_picker.set_refresh(Rc::new(move || {
            *current_icons_for_tones.borrow_mut() = render_grid_for_tones(&search_entry_for_tones.text());
            refresh_grid_for_tones(true);
        }));
    }

    // Initial population - via render_grid so group/collapse filters apply.
    *current_icons.borrow_mut() = render_grid("");
    refresh_grid(true);

    // --- Search filtering ---
    let render_grid_for_search = render_grid.clone();
    let refresh_grid_for_search = refresh_grid.clone();
    let current_icons_for_search = current_icons.clone();
    let history_for_search = history.clone();
    let history_index_for_search = history_index.clone();
    let suppress_for_search = suppress_history.clone();
    let back_btn_for_search = back_btn.clone();
    let forward_btn_for_search = forward_btn.clone();
    search_entry.connect_changed(move |entry| {
        let filter = entry.text().to_string();
        let filtered = render_grid_for_search(&filter);
        *current_icons_for_search.borrow_mut() = filtered;
        refresh_grid_for_search(true);

        // Record search in history (unless navigating via back/forward).
        if !*suppress_for_search.borrow() {
            let entry = HistoryEntry::Search(filter);
            let mut hist = history_for_search.borrow_mut();
            let mut idx = history_index_for_search.borrow_mut();
            if hist.get(*idx).is_some_and(|h| h == &entry) {
                return;
            }
            hist.truncate(*idx + 1);
            hist.push(entry);
            *idx = hist.len() - 1;
            update_nav_buttons(&back_btn_for_search, &forward_btn_for_search, &hist, *idx);
        }
    });

    // --- Group jump-list ---
    if let Some(dropdown) = group_dropdown.take() {
        let active_group_for_dropdown = active_group.clone();
        let render_grid_for_dropdown = render_grid.clone();
        let current_icons_for_dropdown = current_icons.clone();
        let refresh_grid_for_dropdown = refresh_grid.clone();
        let search_entry_for_dropdown = search_entry.clone();
        dropdown.connect_selected_notify(move |dropdown| {
            let index = dropdown.selected() as usize;
            *active_group_for_dropdown.borrow_mut() = index.checked_sub(1).and_then(|i| group_labels.get(i)).copied();
            *current_icons_for_dropdown.borrow_mut() = render_grid_for_dropdown(&search_entry_for_dropdown.text());
            refresh_grid_for_dropdown(true);
        });
    }

    // --- Size slider ---
    let size_value_clone = size_value.clone();
    let current_icon_size_for_slider = current_icon_size.clone();
    let refresh_grid_for_size = refresh_grid.clone();
    size_slider.connect_value_changed(move |slider| {
        let val = slider.value();
        size_value_clone.set_label(&format!("{:.0}px", val));
        *current_icon_size_for_slider.borrow_mut() = val;
        refresh_grid_for_size(true);
    });

    // --- Lazy loading on scroll ---
    let refresh_grid_for_scroll = refresh_grid.clone();
    scrolled.connect_edge_reached(move |_scrolled, edge| {
        if edge == gtk4::PositionType::Bottom {
            refresh_grid_for_scroll(false);
        }
    });

    // FlowBox handles dynamic column reflow automatically - no manual calculation needed.

    // --- Back/Forward button handlers ---
    {
        let sidebar_for_back = sidebar.clone();
        let selected_name_for_back = selected_name.clone();
        let selected_cell_for_back = selected_cell.clone();
        let history_for_back = history.clone();
        let history_index_for_back = history_index.clone();
        let suppress_for_back = suppress_history.clone();
        let forward_btn_for_back = forward_btn.clone();
        let back_btn_for_handler = back_btn.clone();
        let window_for_back = window.clone();
        let detail_ctx_for_back = detail_ctx.clone();
        back_btn.connect_clicked(move |_| {
            let hist = history_for_back.borrow();
            let mut idx = history_index_for_back.borrow_mut();
            if *idx > 0 && !hist.is_empty() {
                *idx -= 1;
                let prev = hist[*idx].clone();
                drop(idx);
                drop(hist);
                *suppress_for_back.borrow_mut() = true;
                navigate_to::<F>(
                    &prev,
                    &detail_ctx_for_back,
                    &sidebar_for_back,
                    &selected_name_for_back,
                    &selected_cell_for_back,
                    &window_for_back,
                );
                *suppress_for_back.borrow_mut() = false;
                let hist = history_for_back.borrow();
                let idx = history_index_for_back.borrow();
                update_nav_buttons(&back_btn_for_handler, &forward_btn_for_back, &hist, *idx);
            }
        });
    }
    {
        let sidebar_for_fwd = sidebar.clone();
        let selected_name_for_fwd = selected_name.clone();
        let selected_cell_for_fwd = selected_cell.clone();
        let history_for_fwd = history.clone();
        let history_index_for_fwd = history_index.clone();
        let suppress_for_fwd = suppress_history.clone();
        let back_btn_for_fwd = back_btn.clone();
        let forward_btn_for_handler = forward_btn.clone();
        let window_for_fwd = window.clone();
        let detail_ctx_for_fwd = detail_ctx.clone();
        forward_btn.connect_clicked(move |_| {
            let hist = history_for_fwd.borrow();
            let mut idx = history_index_for_fwd.borrow_mut();
            if *idx + 1 < hist.len() {
                *idx += 1;
                let next = hist[*idx].clone();
                drop(idx);
                drop(hist);
                *suppress_for_fwd.borrow_mut() = true;
                navigate_to::<F>(&next, &detail_ctx_for_fwd, &sidebar_for_fwd, &selected_name_for_fwd, &selected_cell_for_fwd, &window_for_fwd);
                *suppress_for_fwd.borrow_mut() = false;
                let hist = history_for_fwd.borrow();
                let idx = history_index_for_fwd.borrow();
                update_nav_buttons(&back_btn_for_fwd, &forward_btn_for_handler, &hist, *idx);
            }
        });
    }

    // Initialize history with empty search.
    history.borrow_mut().push(HistoryEntry::Search(String::new()));
    update_nav_buttons(&back_btn, &forward_btn, &history.borrow(), 0);

    window.set_titlebar(Some(&header_bar));
    window.set_child(Some(&main_box));
    window.present();
}
