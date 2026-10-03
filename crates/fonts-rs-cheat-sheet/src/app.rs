//! Generic cheat sheet application: grid, search, history, sidebar.

use crate::detail::show_detail;
use crate::detail_context::DetailContext;
use crate::family::CheatSheetFamily;
use crate::tone_picker::TonePicker;
use fonts_rs_model::CodePoint;
use fonts_rs_model::GroupLabel;
use gtk4::Align;
use gtk4::Box;
use gtk4::Button;
use gtk4::DropDown;
use gtk4::Entry;
use gtk4::FlowBox;
use gtk4::Label;
use gtk4::ListBox;
use gtk4::ListBoxRow;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::ScrolledWindow;
use gtk4::Stack;
use gtk4::ToggleButton;
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;
use std::cell::RefCell;
use std::collections::BTreeMap;
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

/// Navigate to a history entry: search filters the grid, glyph pushes the detail page.
fn navigate_to<F: CheatSheetFamily>(
    entry: &HistoryEntry<F::Name>,
    ctx: &DetailContext,
    selected_name: &Rc<RefCell<Option<F::Name>>>,
    selected_cell: &Rc<RefCell<Option<gtk4::Box>>>,
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
            ctx.detail_stack.set_visible_child_name("empty");
            ctx.split_view.set_show_content(false);
        }
        HistoryEntry::Icon(name, cp) => {
            *selected_name.borrow_mut() = Some(name.clone());
            ctx.detail_stack.set_visible_child_name("detail");
            ctx.split_view.set_show_content(true);
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

    let app = adw::Application::builder().application_id(F::APP_ID).build();
    app.connect_activate(build_ui::<F>);
    Ok(app.run())
}

fn build_ui<F: CheatSheetFamily>(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
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
        .cheat-cell:hover { background-color: alpha(@accent_bg_color, 0.15); }
        .cheat-cell-selected { background-color: alpha(@accent_bg_color, 0.30); }
        .cheat-meta-flow { background: transparent; }
        .cheat-meta-flow flowboxchild { background: transparent; padding: 0; }
        button.cat-btn { background: alpha(@success_bg_color, 0.3); border-radius: 12px; padding: 2px 8px; font-size: 10px; border: none; box-shadow: none; }
        button.cat-btn:hover { background: alpha(@success_bg_color, 0.5); }
        button.kw-btn { background: alpha(@warning_bg_color, 0.3); border-radius: 12px; padding: 2px 8px; font-size: 10px; border: none; box-shadow: none; }
        button.kw-btn:hover { background: alpha(@warning_bg_color, 0.5); }
        .sidebar-title { font-size: 16px; font-weight: bold; }
        .sidebar-subtitle { font-size: 12px; opacity: 0.7; }
        .sidebar-codepoint-char { font-size: 96px; }
        .sidebar-codepoint-hex { font-size: 14px; }
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

    // --- AdwHeaderBar with WindowTitle + Back/Forward history ---
    let header_bar = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::builder().title(F::TITLE).subtitle(format!("{total} {}", F::NOUN)).build())
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

    // Icon size slider - kept in the sidebar where it is immediately
    // visible instead of a preferences dialog.
    let size_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Center)
        .margin_bottom(4)
        .build();
    let size_label = Label::builder().label("Icon size:").build();
    let size_value = Label::builder().label("42px").css_classes(["monospace"]).build();
    // Logarithmic scale: the slider runs in log2 space (4.0..9.0 for
    // 16..512px), so small sizes get the same travel as large ones and
    // the marks spread evenly across the range. `digits(-1)` keeps the
    // full precision - the value is only rounded for display.
    let size_slider = Scale::builder()
        .orientation(Orientation::Horizontal)
        .adjustment(&gtk4::Adjustment::new(42.0_f64.log2(), 4.0, 9.0, 0.05, 0.5, 0.0))
        .width_request(640)
        .halign(Align::Center)
        .draw_value(false)
        .digits(-1)
        .build();
    // Marks at the log2 position of each typical icon size.
    for (size, label) in [
        (16.0_f64, "16"),
        (24.0, "24"),
        (32.0, "32"),
        (48.0, "48"),
        (64.0, "64"),
        (96.0, "96"),
        (128.0, "128"),
        (256.0, "256"),
        (512.0, "512"),
    ] {
        size_slider.add_mark(size.log2(), gtk4::PositionType::Bottom, Some(label));
    }
    size_box.append(&size_label);
    size_box.append(&size_slider);
    size_box.append(&size_value);
    top_bar.append(&size_box);

    // Default skin tone row: buttons render the alternates of a
    // representative glyph; clicking applies the index as the global
    // default tone for every tone group. Only shown when the family's
    // ordering provides tone data.
    if let Some(ord) = ordering
        && let Some((_, base)) = all_icons
            .iter()
            .find(|(_, name)| ord.tone_group(name.as_ref()).is_some() && !ord.alternates(name.as_ref()).is_empty())
    {
        let glyph_preview = |name_str: &str| -> gtk4::Widget {
            let icon_name = F::parse_name(name_str).and_then(|n| F::glyph_icon_name(&n));
            if let Some(icon_name) = icon_name {
                return gtk4::Image::builder().icon_name(&icon_name).pixel_size(18).build().upcast();
            }
            let text = F::codepoint(name_str).map(|cp| cp.as_char().to_string()).unwrap_or_else(|| "?".to_string());
            Label::builder().label(&text).build().upcast()
        };

        let tone_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .halign(Align::Center)
            .margin_bottom(4)
            .build();
        tone_box.append(&Label::builder().label("Tone:").build());

        // First button shows the base glyph and resets every stored tone.
        let tone_buttons: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));
        let mark_active = |buttons: &Rc<RefCell<Vec<Button>>>, active: usize| {
            for (i, btn) in buttons.borrow().iter().enumerate() {
                if i == active {
                    btn.add_css_class("suggested-action");
                } else {
                    btn.remove_css_class("suggested-action");
                }
            }
        };
        {
            let buttons = tone_buttons.clone();
            let picker = tone_picker.clone();
            let base_btn = Button::builder().child(&glyph_preview(base.as_ref())).tooltip_text("Default tone").build();
            base_btn.add_css_class("suggested-action");
            base_btn.connect_clicked(move |_| {
                picker.reset_all();
                mark_active(&buttons, 0);
            });
            tone_buttons.borrow_mut().push(base_btn.clone());
            tone_box.append(&base_btn);
        }
        for (index, variant) in ord.alternates(base.as_ref()).iter().enumerate() {
            let buttons = tone_buttons.clone();
            let picker = tone_picker.clone();
            let variant_label = F::parse_name(variant).map(|n| F::display_name(&n)).unwrap_or_else(|| variant.to_string());
            let btn = Button::builder().child(&glyph_preview(variant)).tooltip_text(variant_label).build();
            btn.connect_clicked(move |_| {
                picker.set_default(index);
                mark_active(&buttons, index + 1);
            });
            tone_buttons.borrow_mut().push(btn.clone());
            tone_box.append(&btn);
        }
        top_bar.append(&tone_box);
    }

    // --- Browse page (split-view sidebar): top_bar + scrolled icon grid ---
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

    // --- Category sidebar: OverlaySplitView wrapping a sectioned list
    // (Groups + Categories) left of the browse content. `None` when the
    // family exposes neither axis; row activation is wired below once
    // `render_grid`/`refresh_grid` exist. ---
    let mut category_counts: BTreeMap<String, u32> = BTreeMap::new();
    for (_, name) in &all_icons {
        for category in F::categories(name) {
            *category_counts.entry(category).or_default() += 1;
        }
    }

    let mut group_list: Option<ListBox> = None;
    let mut category_list: Option<ListBox> = None;
    let category_split = if category_counts.is_empty() && group_labels.is_empty() {
        None
    } else {
        let sidebar_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .build();

        let section_header = |list: &ListBox, title: &str| {
            let row = ListBoxRow::new();
            row.set_activatable(false);
            row.set_selectable(false);
            row.set_child(Some(&Label::builder().label(title).css_classes(["heading"]).halign(Align::Start).build()));
            list.append(&row);
        };

        if !group_labels.is_empty() {
            let list = ListBox::builder().css_classes(["navigation-sidebar"]).build();
            section_header(&list, "Groups");
            // "All groups" resets the group filter; identified by its
            // empty widget name in the activation handler.
            let all_row = ListBoxRow::new();
            all_row.set_child(Some(&Label::builder().label("All groups").halign(Align::Start).build()));
            list.append(&all_row);
            for label in group_labels {
                let row = ListBoxRow::new();
                row.set_widget_name(label.as_str());
                row.set_child(Some(&Label::builder().label(label.as_str()).halign(Align::Start).build()));
                list.append(&row);
            }
            sidebar_box.append(&list);
            group_list = Some(list);
        }
        if !category_counts.is_empty() {
            let list = ListBox::builder().css_classes(["navigation-sidebar"]).build();
            section_header(&list, "Categories");
            for (category, count) in &category_counts {
                let row = ListBoxRow::new();
                row.set_widget_name(category);
                row.set_child(Some(&Label::builder().label(format!("{category} ({count})")).halign(Align::Start).build()));
                list.append(&row);
            }
            sidebar_box.append(&list);
            category_list = Some(list);
        }

        let sidebar_scrolled = ScrolledWindow::builder().min_content_width(200).vexpand(true).child(&sidebar_box).build();
        let split = adw::OverlaySplitView::builder()
            .sidebar_position(gtk4::PackType::Start)
            .show_sidebar(true)
            .min_sidebar_width(200.0)
            .max_sidebar_width(320.0)
            .build();
        split.set_sidebar(Some(&sidebar_scrolled));
        split.set_content(Some(&left_box));
        Some(split)
    };

    // HeaderBar toggle for the category sidebar, bidirectionally bound
    // so swipes/dismiss and the button stay in sync.
    if let Some(split) = &category_split {
        let sidebar_toggle = ToggleButton::builder().icon_name("sidebar-show-symbolic").tooltip_text("Categories").build();
        split
            .bind_property("show-sidebar", &sidebar_toggle, "active")
            .bidirectional()
            .sync_create()
            .build();
        header_bar.pack_start(&sidebar_toggle);
    }

    // --- Detail page (split-view content): own HeaderBar so collapsed
    // mode gains the free back button; Stack swaps placeholder/detail. ---
    let detail_content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .vexpand(true)
        .build();
    let detail_scrolled = ScrolledWindow::builder().vexpand(true).child(&detail_content).build();

    // Secondary HeaderBar: window controls belong to the main HeaderBar
    // only - suppress the duplicated minimize/maximize/close buttons.
    let detail_header = adw::HeaderBar::builder().show_start_title_buttons(false).show_end_title_buttons(false).build();
    let close_btn = Button::builder()
        .icon_name("window-close-symbolic")
        .css_classes(["flat"])
        .tooltip_text("Close details")
        .build();
    detail_header.pack_end(&close_btn);
    let detail_toolbar = adw::ToolbarView::new();
    detail_toolbar.add_top_bar(&detail_header);
    detail_toolbar.set_content(Some(&detail_scrolled));

    let detail_placeholder = adw::StatusPage::builder()
        .icon_name("edit-find-symbolic")
        .title("No icon selected")
        .description("Choose an icon from the grid to see its details.")
        .build();
    let detail_stack = Stack::new();
    detail_stack.add_named(&detail_placeholder, Some("empty"));
    detail_stack.add_named(&detail_toolbar, Some("detail"));
    detail_stack.set_visible_child_name("empty");

    let browse_widget: gtk4::Widget = match &category_split {
        Some(split) => split.clone().upcast(),
        None => left_box.clone().upcast(),
    };
    let browse_page = adw::NavigationPage::builder().title(F::TITLE).child(&browse_widget).build();
    let detail_page = adw::NavigationPage::builder().title("Details").child(&detail_stack).build();
    let split_view = adw::NavigationSplitView::new();
    split_view.set_sidebar(Some(&browse_page));
    split_view.set_content(Some(&detail_page));
    // The sidebar pane carries the grid - let it take most of the width.
    split_view.set_sidebar_width_fraction(0.7);
    split_view.set_min_sidebar_width(240.0);
    split_view.set_max_sidebar_width(4000.0);

    // --- Shared selection state ---
    let selected_name: Rc<RefCell<Option<F::Name>>> = Rc::new(RefCell::new(None));
    let selected_cell: Rc<RefCell<Option<gtk4::Box>>> = Rc::new(RefCell::new(None));

    // --- Close button handler: back to grid (collapsed) + placeholder ---
    {
        let split_for_close = split_view.clone();
        let stack_for_close = detail_stack.clone();
        let content_for_close = detail_content.clone();
        let selected_name_for_close = selected_name.clone();
        let selected_cell_for_close = selected_cell.clone();
        close_btn.connect_clicked(move |_| {
            if let Some(cell) = selected_cell_for_close.borrow_mut().take() {
                cell.remove_css_class("cheat-cell-selected");
            }
            selected_name_for_close.borrow_mut().take();
            while let Some(child) = content_for_close.first_child() {
                content_for_close.remove(&child);
            }
            stack_for_close.set_visible_child_name("empty");
            split_for_close.set_show_content(false);
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
            // Variant collapsing: browsing hides variants whose base is
            // itself displayed (search keeps them reachable). With a
            // style group selected the bases live in another group, so
            // the group's variants must stay visible.
            let collapse = ordering.is_some_and(|o| o.collapse_variants());
            let is_collapsed_variant = |name: &F::Name| {
                collapse
                    && ordering.is_some_and(|o| {
                        o.base_of(name.as_ref()).is_some_and(|base_name| F::parse_name(base_name).is_some_and(|base| in_group(&base)))
                    })
            };
            let base: Vec<(CodePoint, F::Name)> = icons_for_filter
                .iter()
                .filter(|(_, name)| in_group(name) && !is_collapsed_variant(name))
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

    // Toast overlay wraps the whole window content; declared early so
    // the detail context can show copy-feedback toasts.
    let toast_overlay = adw::ToastOverlay::new();

    let grid_clone = grid.clone();
    let search_entry_clone = search_entry.clone();
    // Shared handles for the detail sidebar + tone picking.
    let detail_ctx = DetailContext {
        content: detail_content.clone(),
        search_entry: search_entry.clone(),
        tone_picker: tone_picker.clone(),
        split_view: split_view.clone(),
        detail_stack: detail_stack.clone(),
        toasts: toast_overlay.clone(),
        category_split: category_split.clone(),
    };
    let selected_name_for_grid = selected_name.clone();
    let selected_cell_for_grid = selected_cell.clone();
    let history_for_grid = history.clone();
    let history_index_for_grid = history_index.clone();
    let suppress_for_grid = suppress_history.clone();
    let back_btn_for_grid = back_btn.clone();
    let forward_btn_for_grid = forward_btn.clone();
    let detail_ctx_for_grid = detail_ctx.clone();
    let toasts_for_grid = toast_overlay.clone();
    let category_split_for_grid = category_split.clone();

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
                    let split_ref = category_split_for_grid.clone();
                    btn.connect_clicked(move |_| {
                        search_entry_ref.set_text(&cat_label);
                        // Reveal the axis the user just filtered by.
                        if let Some(split) = &split_ref {
                            split.set_show_sidebar(true);
                        }
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

            // Click on card -> push detail page (but not on buttons).
            let name_for_click = name.clone();
            let cp_for_click = *codepoint;
            let vbox_ref = vbox.clone();
            let selected_name_ref = selected_name_for_grid.clone();
            let selected_cell_ref = selected_cell_for_grid.clone();
            let history_ref = history_for_grid.clone();
            let history_index_ref = history_index_for_grid.clone();
            let suppress_ref = suppress_for_grid.clone();
            let back_btn_ref = back_btn_for_grid.clone();
            let forward_btn_ref = forward_btn_for_grid.clone();
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
                // Toggle: if already selected, clear the detail page.
                let already_selected = selected_name_ref.borrow().as_ref().map(|n| n == &name_for_click).unwrap_or(false);
                if already_selected {
                    if let Some(old) = selected_cell_ref.borrow_mut().take() {
                        old.remove_css_class("cheat-cell-selected");
                    }
                    *selected_name_ref.borrow_mut() = None;
                    detail_ctx_ref.detail_stack.set_visible_child_name("empty");
                    detail_ctx_ref.split_view.set_show_content(false);
                    return;
                }
                if let Some(old) = selected_cell_ref.borrow_mut().take() {
                    old.remove_css_class("cheat-cell-selected");
                }
                *selected_name_ref.borrow_mut() = Some(name_for_click.clone());
                vbox_ref.add_css_class("cheat-cell-selected");
                *selected_cell_ref.borrow_mut() = Some(vbox_ref.clone());
                detail_ctx_ref.detail_stack.set_visible_child_name("detail");
                detail_ctx_ref.split_view.set_show_content(true);
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

            // Secondary click copies the display name (cell copy path).
            let name_for_copy = display_name.clone();
            let toasts_for_copy = toasts_for_grid.clone();
            let copy_gesture = gtk4::GestureClick::new();
            copy_gesture.set_button(gtk4::gdk::BUTTON_SECONDARY);
            copy_gesture.connect_released(move |_, _, _, _| {
                if let Some(display) = gtk4::gdk::Display::default() {
                    display.clipboard().set_text(&name_for_copy);
                    toasts_for_copy.add_toast(adw::Toast::new("Copied name"));
                }
            });
            vbox.add_controller(copy_gesture);

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
    let group_dropdown_widget = group_dropdown.clone();
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

    // --- Category sidebar row activation ---
    // Category rows: click replaces the search text, Shift+click appends
    // (space-separated AND), clicking the active row clears the search.
    // A GestureClick on the list (instead of `row-activated`) gives the
    // event modifiers for the Shift detection.
    if let Some(list) = category_list {
        let search_entry_for_list = search_entry.clone();
        let list_for_click = list.clone();
        let click = gtk4::GestureClick::new();
        click.set_button(gtk4::gdk::BUTTON_PRIMARY);
        click.connect_released(move |gesture, _, _, y| {
            let Some(row) = list_for_click.row_at_y(y as i32) else {
                return;
            };
            // Section headers are non-activatable.
            if !row.is_activatable() {
                return;
            }
            let category = row.widget_name();
            let shift = gesture.current_event_state().contains(gtk4::gdk::ModifierType::SHIFT_MASK);
            let current = search_entry_for_list.text();
            if shift && !current.is_empty() {
                search_entry_for_list.set_text(&format!("{current} {category}"));
            } else if !shift && current.as_str() == category.as_str() {
                search_entry_for_list.set_text("");
                list_for_click.unselect_all();
            } else {
                search_entry_for_list.set_text(&category);
            }
        });
        list.add_controller(click);
    }
    // Group rows mirror the dropdown: route through `set_selected` so the
    // active_group + re-render logic stays in one place. Activating the
    // selected group falls back to "All groups" (index 0).
    if let (Some(list), Some(dropdown)) = (group_list, group_dropdown_widget) {
        let list_for_click = list.clone();
        let click = gtk4::GestureClick::new();
        click.set_button(gtk4::gdk::BUTTON_PRIMARY);
        click.connect_released(move |_, _, _, y| {
            let Some(row) = list_for_click.row_at_y(y as i32) else {
                return;
            };
            // Section headers are non-activatable.
            if !row.is_activatable() {
                return;
            }
            let index = group_labels.iter().position(|label| label.as_str() == row.widget_name().as_str());
            match index {
                // Re-activating the selected group or clicking "All
                // groups" (empty widget name) resets the filter.
                Some(index) if dropdown.selected() as usize != index + 1 => dropdown.set_selected(index as u32 + 1),
                _ => dropdown.set_selected(0),
            }
            list_for_click.unselect_all();
        });
        list.add_controller(click);
    }

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
        let selected_name_for_back = selected_name.clone();
        let selected_cell_for_back = selected_cell.clone();
        let history_for_back = history.clone();
        let history_index_for_back = history_index.clone();
        let suppress_for_back = suppress_history.clone();
        let forward_btn_for_back = forward_btn.clone();
        let back_btn_for_handler = back_btn.clone();
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
                navigate_to::<F>(&prev, &detail_ctx_for_back, &selected_name_for_back, &selected_cell_for_back);
                *suppress_for_back.borrow_mut() = false;
                let hist = history_for_back.borrow();
                let idx = history_index_for_back.borrow();
                update_nav_buttons(&back_btn_for_handler, &forward_btn_for_back, &hist, *idx);
            }
        });
    }
    {
        let selected_name_for_fwd = selected_name.clone();
        let selected_cell_for_fwd = selected_cell.clone();
        let history_for_fwd = history.clone();
        let history_index_for_fwd = history_index.clone();
        let suppress_for_fwd = suppress_history.clone();
        let back_btn_for_fwd = back_btn.clone();
        let forward_btn_for_handler = forward_btn.clone();
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
                navigate_to::<F>(&next, &detail_ctx_for_fwd, &selected_name_for_fwd, &selected_cell_for_fwd);
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

    // --- Size slider: adjust icon size + refresh the grid ---
    let size_value_clone = size_value.clone();
    let current_icon_size_for_slider = current_icon_size.clone();
    let refresh_grid_for_size = refresh_grid.clone();
    size_slider.connect_value_changed(move |slider| {
        let val = 2.0_f64.powf(slider.value()).round();
        size_value_clone.set_label(&format!("{val:.0}px"));
        *current_icon_size_for_slider.borrow_mut() = val;
        refresh_grid_for_size(true);
    });

    // Collapse the split view on narrow windows: the detail pane then
    // covers the grid and its HeaderBar gains an automatic back button.
    let breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(adw::BreakpointConditionLengthType::MaxWidth, 800.0, adw::LengthUnit::Sp));
    breakpoint.add_setter(&split_view, "collapsed", Some(&true.to_value()));
    window.add_breakpoint(breakpoint);

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header_bar);
    toolbar_view.set_content(Some(&split_view));

    toast_overlay.set_child(Some(&toolbar_view));
    window.set_content(Some(&toast_overlay));
    window.present();
}
