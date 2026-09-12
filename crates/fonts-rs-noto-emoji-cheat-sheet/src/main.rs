//! Noto Emoji Cheat Sheet application.
//!
//! Browse all available Noto Emoji glyphs with a search filter.
//! Displays each emoji with its Unicode codepoint, glyph name,
//! keywords, and categories. Search covers glyph names, keywords,
//! and categories. Click an emoji to open a detail sidebar.
//!
//! Run with:
//! ```sh
//! cargo run -p fonts-rs-noto-emoji-cheat-sheet
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use fonts_rs_model::CodePoint;
use fonts_rs_noto_emoji::GlyphNameExt;
use fonts_rs_noto_emoji::all_emoji;
use fonts_rs_noto_emoji::register_glyphs;
use fonts_rs_noto_emoji::variant::GRESOURCE_PREFIX;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::Entry;
use gtk4::FlowBox;
use gtk4::HeaderBar;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::ScrolledWindow;
use gtk4::TextView;
use gtk4::WrapMode;
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

#[cfg(feature = "metadata")]
use fonts_rs_noto_emoji::metadata;

const APP_ID: &str = "io.smearor.fonts_rs.noto_emoji_cheat_sheet";

/// A navigable history entry: either a search query or an emoji detail view.
#[derive(Clone, PartialEq, Eq)]
enum HistoryEntry {
    Search(String),
    Emoji(String, CodePoint),
}

/// Update sensitivity of Back/Forward buttons based on current history position.
fn update_nav_buttons(back_btn: &Button, forward_btn: &Button, history: &[HistoryEntry], index: usize) {
    back_btn.set_sensitive(index > 0);
    forward_btn.set_sensitive(index + 1 < history.len());
}

/// Navigate to a history entry: search filters the grid, emoji opens the sidebar.
fn navigate_to(
    entry: &HistoryEntry,
    search_entry: &Entry,
    sidebar: &Box,
    content: &Box,
    selected_name: &Rc<RefCell<Option<String>>>,
    selected_cell: &Rc<RefCell<Option<gtk4::Box>>>,
    window: &ApplicationWindow,
) {
    if let Some(old) = selected_cell.borrow_mut().take() {
        old.remove_css_class("cheat-cell-selected");
    }
    match entry {
        HistoryEntry::Search(filter) => {
            search_entry.set_text(filter);
            search_entry.set_position(-1);
            *selected_name.borrow_mut() = None;
            sidebar.set_visible(false);
            window.set_default_size(460, 800);
            window.set_size_request(460, -1);
        }
        HistoryEntry::Emoji(name, cp) => {
            *selected_name.borrow_mut() = Some(name.clone());
            sidebar.set_visible(true);
            window.set_default_size(784, 800);
            window.set_size_request(784, -1);
            show_detail(content, name, *cp, search_entry);
        }
    }
}

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    Ok(app.run())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Noto Emoji - Cheat Sheet")
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

    let all_emoji_list = all_emoji();
    let total = all_emoji_list.len();

    let header_bar = HeaderBar::builder()
        .title_widget(&gtk4::Label::new(Some("Noto Emoji - Cheat Sheet")))
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

    let history: Rc<RefCell<Vec<HistoryEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let history_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let suppress_history: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

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
        .label(format!("{} emoji available", total))
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
    top_bar.append(&count_bar);

    let search_entry = Entry::builder()
        .placeholder_text("Search by name, keyword, or category (e.g. 'face', 'fire', 'Smileys')...")
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

    let main_box = Box::builder().orientation(Orientation::Horizontal).vexpand(true).hexpand(true).build();
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

    let sidebar_header = Box::builder().orientation(Orientation::Horizontal).halign(Align::End).build();
    let close_btn = Button::builder()
        .icon_name("window-close-symbolic")
        .css_classes(["flat"])
        .tooltip_text("Close sidebar")
        .build();
    sidebar_header.append(&close_btn);
    sidebar.append(&sidebar_header);

    let sidebar_content = Box::builder().orientation(Orientation::Vertical).spacing(12).vexpand(true).build();
    sidebar.append(&sidebar_content);

    sidebar.set_visible(false);
    main_box.append(&sidebar);

    let selected_name: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let selected_cell: Rc<RefCell<Option<gtk4::Box>>> = Rc::new(RefCell::new(None));

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

    const BATCH_SIZE: usize = 100;

    let current_emoji: Rc<RefCell<Vec<(CodePoint, String)>>> = Rc::new(RefCell::new(Vec::new()));
    let displayed_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let current_icon_size: Rc<RefCell<f64>> = Rc::new(RefCell::new(42.0));

    let emoji_for_filter = all_emoji_list.clone();
    let render_grid = move |filter: &str| -> Vec<(CodePoint, String)> {
        let filter_lower = filter.to_lowercase();
        if filter_lower.is_empty() {
            return emoji_for_filter.to_vec();
        }

        #[cfg(feature = "metadata")]
        {
            let search_results = metadata::search_emoji(filter);
            let mut filtered: Vec<(CodePoint, String)> = search_results
                .iter()
                .filter_map(|name| {
                    let glyph_name = fonts_rs_noto_emoji::naming::NotoEmojiName::new(name.to_string());
                    let cp = glyph_name.codepoint()?;
                    Some((cp, name.to_string()))
                })
                .collect();

            for (cp, name) in emoji_for_filter.iter() {
                let hex = format!("{:X}", cp.as_char() as u32);
                let char_str = cp.as_char().to_string();
                if (hex.to_lowercase().contains(&filter_lower)
                    || format!("\\u{{{:X}}}", cp.as_char() as u32).to_lowercase().contains(&filter_lower)
                    || char_str.contains(filter)
                    || name.to_lowercase().contains(&filter_lower))
                    && !filtered.iter().any(|(_, n)| n == name)
                {
                    filtered.push((*cp, name.clone()));
                }
            }

            filtered.sort_by(|a, b| a.1.cmp(&b.1));
            filtered
        }

        #[cfg(not(feature = "metadata"))]
        {
            emoji_for_filter
                .iter()
                .filter(|(cp, name)| {
                    let hex = format!("{:X}", cp.as_char() as u32);
                    let char_str = cp.as_char().to_string();
                    name.to_lowercase().contains(&filter_lower) || hex.to_lowercase().contains(&filter_lower) || char_str.contains(filter)
                })
                .cloned()
                .collect()
        }
    };

    let grid_clone = grid.clone();
    let sidebar_clone = sidebar.clone();
    let content_clone = sidebar_content.clone();
    let search_entry_clone = search_entry.clone();
    let selected_name_for_grid = selected_name.clone();
    let selected_cell_for_grid = selected_cell.clone();
    let history_for_grid = history.clone();
    let history_index_for_grid = history_index.clone();
    let suppress_for_grid = suppress_history.clone();
    let back_btn_for_grid = back_btn.clone();
    let forward_btn_for_grid = forward_btn.clone();
    let window_for_grid = window.clone();

    let populate_batch = move |start: usize, end: usize, emojis: &[(CodePoint, String)], icon_size: f64, _cols: u32| {
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

        for (codepoint, name) in emojis[start..end].iter() {
            let hex = format!("\\u{{{:X}}}", codepoint.as_char() as u32);

            let icon = Label::builder()
                .label(codepoint.as_char().to_string())
                .css_classes(["cheat-icon-label"])
                .halign(Align::Center)
                .build();

            let name_label = Label::builder()
                .label(name)
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

            let meta_box = Box::builder().orientation(Orientation::Vertical).spacing(2).halign(Align::Fill).build();

            #[cfg(feature = "metadata")]
            {
                let keywords = metadata::emoji_keywords(name);
                let category = metadata::emoji_category(name);

                if let Some(cat) = category {
                    let cat_flow = FlowBox::builder()
                        .orientation(Orientation::Horizontal)
                        .selection_mode(gtk4::SelectionMode::None)
                        .column_spacing(4)
                        .row_spacing(2)
                        .halign(Align::Center)
                        .max_children_per_line(4)
                        .css_classes(["cheat-meta-flow"])
                        .build();
                    let cat_label = cat.to_string();
                    let btn = Button::builder().label(&cat_label).css_classes(["cat-btn"]).build();
                    let search_entry_ref = search_entry_clone.clone();
                    btn.connect_clicked(move |_| {
                        search_entry_ref.set_text(&cat_label);
                    });
                    cat_flow.insert(&btn, -1);
                    meta_box.append(&cat_flow);
                }

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
                    for kw in keywords.iter().take(5) {
                        let kw_label = kw.to_string();
                        let btn = Button::builder().label(&kw_label).css_classes(["kw-btn"]).build();
                        let search_entry_ref = search_entry_clone.clone();
                        btn.connect_clicked(move |_| {
                            search_entry_ref.set_text(&kw_label);
                        });
                        kw_flow.insert(&btn, -1);
                    }
                    meta_box.append(&kw_flow);
                }
            }

            let vbox = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .halign(Align::Center)
                .css_classes(["cheat-cell"])
                .build();

            let is_selected = selected_name_for_grid.borrow().as_ref().map(|n| n == name).unwrap_or(false);
            if is_selected {
                vbox.add_css_class("cheat-cell-selected");
                *selected_cell_for_grid.borrow_mut() = Some(vbox.clone());
            }

            vbox.append(&icon);
            vbox.append(&name_label);
            vbox.append(&codepoint_label);

            #[cfg(feature = "metadata")]
            {
                let keywords = metadata::emoji_keywords(name);
                let category = metadata::emoji_category(name);
                if category.is_some() || !keywords.is_empty() {
                    vbox.append(&meta_box);
                }
            }

            let name_for_click = name.clone();
            let cp_for_click = *codepoint;
            let sidebar_ref = sidebar_clone.clone();
            let content_ref = content_clone.clone();
            let search_entry_ref = search_entry_clone.clone();
            let vbox_ref = vbox.clone();
            let selected_name_ref = selected_name_for_grid.clone();
            let selected_cell_ref = selected_cell_for_grid.clone();
            let history_ref = history_for_grid.clone();
            let history_index_ref = history_index_for_grid.clone();
            let suppress_ref = suppress_for_grid.clone();
            let back_btn_ref = back_btn_for_grid.clone();
            let forward_btn_ref = forward_btn_for_grid.clone();
            let window_ref = window_for_grid.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.connect_released(move |gesture, _n, x, y| {
                if let Some(widget) = gesture.widget() {
                    if let Some(target) = widget.pick(x, y, gtk4::PickFlags::empty()) {
                        let mut current = Some(target);
                        while let Some(w) = current {
                            if w.is::<gtk4::Button>() {
                                return;
                            }
                            current = w.parent();
                        }
                    }
                }
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
                show_detail(&content_ref, &name_for_click, cp_for_click, &search_entry_ref);

                if !*suppress_ref.borrow() {
                    let entry = HistoryEntry::Emoji(name_for_click.clone(), cp_for_click);
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

    let refresh_grid = {
        let current_emoji_ref = current_emoji.clone();
        let displayed_count_ref = displayed_count.clone();
        let size_ref = current_icon_size.clone();
        let populate_ref = populate_batch.clone();
        let count_label_ref = count_label.clone();
        move |reset: bool| {
            let emojis = current_emoji_ref.borrow().clone();
            let total_emojis = emojis.len();
            if reset {
                let count = BATCH_SIZE.min(total_emojis);
                *displayed_count_ref.borrow_mut() = count;
                let size = *size_ref.borrow();
                populate_ref(0, count, &emojis, size, 0);
            } else {
                let start = *displayed_count_ref.borrow();
                if start >= total_emojis {
                    return;
                }
                let end = (start + BATCH_SIZE).min(total_emojis);
                let size = *size_ref.borrow();
                populate_ref(start, end, &emojis, size, 0);
                *displayed_count_ref.borrow_mut() = end;
            }
            let shown = *displayed_count_ref.borrow();
            let label = if shown >= total_emojis {
                format!("{} / {} emoji", total_emojis, total_emojis)
            } else {
                format!("{} / {} emoji (scroll for more)", shown, total_emojis)
            };
            count_label_ref.set_label(&label);
        }
    };

    *current_emoji.borrow_mut() = all_emoji_list.clone();
    refresh_grid(true);

    let render_grid_for_search = render_grid.clone();
    let refresh_grid_for_search = refresh_grid.clone();
    let current_emoji_for_search = current_emoji.clone();
    let history_for_search = history.clone();
    let history_index_for_search = history_index.clone();
    let suppress_for_search = suppress_history.clone();
    let back_btn_for_search = back_btn.clone();
    let forward_btn_for_search = forward_btn.clone();
    search_entry.connect_changed(move |entry| {
        let filter = entry.text().to_string();
        let filtered = render_grid_for_search(&filter);
        *current_emoji_for_search.borrow_mut() = filtered;
        refresh_grid_for_search(true);

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

    let size_value_clone = size_value.clone();
    let current_icon_size_for_slider = current_icon_size.clone();
    let refresh_grid_for_size = refresh_grid.clone();
    size_slider.connect_value_changed(move |slider| {
        let val = slider.value();
        size_value_clone.set_label(&format!("{:.0}px", val));
        *current_icon_size_for_slider.borrow_mut() = val;
        refresh_grid_for_size(true);
    });

    let refresh_grid_for_scroll = refresh_grid.clone();
    scrolled.connect_edge_reached(move |_scrolled, edge| {
        if edge == gtk4::PositionType::Bottom {
            refresh_grid_for_scroll(false);
        }
    });

    {
        let search_entry_for_back = search_entry.clone();
        let sidebar_for_back = sidebar.clone();
        let content_for_back = sidebar_content.clone();
        let selected_name_for_back = selected_name.clone();
        let selected_cell_for_back = selected_cell.clone();
        let history_for_back = history.clone();
        let history_index_for_back = history_index.clone();
        let suppress_for_back = suppress_history.clone();
        let forward_btn_for_back = forward_btn.clone();
        let back_btn_for_handler = back_btn.clone();
        let window_for_back = window.clone();
        back_btn.connect_clicked(move |_| {
            let hist = history_for_back.borrow();
            let mut idx = history_index_for_back.borrow_mut();
            if *idx > 0 && !hist.is_empty() {
                *idx -= 1;
                let prev = hist[*idx].clone();
                drop(idx);
                drop(hist);
                *suppress_for_back.borrow_mut() = true;
                navigate_to(
                    &prev,
                    &search_entry_for_back,
                    &sidebar_for_back,
                    &content_for_back,
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
        let search_entry_for_fwd = search_entry.clone();
        let sidebar_for_fwd = sidebar.clone();
        let content_for_fwd = sidebar_content.clone();
        let selected_name_for_fwd = selected_name.clone();
        let selected_cell_for_fwd = selected_cell.clone();
        let history_for_fwd = history.clone();
        let history_index_for_fwd = history_index.clone();
        let suppress_for_fwd = suppress_history.clone();
        let back_btn_for_fwd = back_btn.clone();
        let forward_btn_for_handler = forward_btn.clone();
        let window_for_fwd = window.clone();
        forward_btn.connect_clicked(move |_| {
            let hist = history_for_fwd.borrow();
            let mut idx = history_index_for_fwd.borrow_mut();
            if *idx + 1 < hist.len() {
                *idx += 1;
                let next = hist[*idx].clone();
                drop(idx);
                drop(hist);
                *suppress_for_fwd.borrow_mut() = true;
                navigate_to(
                    &next,
                    &search_entry_for_fwd,
                    &sidebar_for_fwd,
                    &content_for_fwd,
                    &selected_name_for_fwd,
                    &selected_cell_for_fwd,
                    &window_for_fwd,
                );
                *suppress_for_fwd.borrow_mut() = false;
                let hist = history_for_fwd.borrow();
                let idx = history_index_for_fwd.borrow();
                update_nav_buttons(&back_btn_for_fwd, &forward_btn_for_handler, &hist, *idx);
            }
        });
    }

    history.borrow_mut().push(HistoryEntry::Search(String::new()));
    update_nav_buttons(&back_btn, &forward_btn, &history.borrow(), 0);

    window.set_titlebar(Some(&header_bar));
    window.set_child(Some(&main_box));
    window.present();
}

/// Populate the sidebar content with details for the given emoji.
fn show_detail(content: &Box, name: &str, codepoint: CodePoint, search_entry: &Entry) {
    while let Some(child) = content.first_child() {
        content.remove(&child);
    }

    let title_label = Label::builder()
        .label(name)
        .css_classes(["sidebar-title"])
        .halign(Align::Start)
        .selectable(true)
        .wrap(true)
        .build();
    content.append(&title_label);

    let subtitle_label = Label::builder()
        .label("Noto Emoji")
        .css_classes(["sidebar-subtitle"])
        .halign(Align::Start)
        .build();
    content.append(&subtitle_label);

    let name_heading = Label::builder()
        .label("Glyph Name")
        .css_classes(["heading"])
        .halign(Align::Start)
        .margin_top(8)
        .build();
    content.append(&name_heading);

    let name_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).halign(Align::Fill).build();
    let name_val = Label::builder()
        .label(name)
        .css_classes(["monospace", "copyable"])
        .halign(Align::Fill)
        .selectable(true)
        .wrap(true)
        .max_width_chars(20)
        .build();
    let name_copy = Button::builder().icon_name("edit-copy-symbolic").tooltip_text("Copy glyph name").build();
    let name_for_copy = name.to_string();
    name_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&name_for_copy);
        }
    });
    name_row.append(&name_val);
    name_row.append(&name_copy);
    content.append(&name_row);

    let cp_heading = Label::builder()
        .label("Codepoint")
        .css_classes(["heading"])
        .halign(Align::Start)
        .margin_top(8)
        .build();
    content.append(&cp_heading);

    let cp_char_label = Label::builder()
        .label(codepoint.as_char().to_string())
        .css_classes(["sidebar-codepoint-char", "copyable"])
        .halign(Align::Center)
        .selectable(true)
        .wrap(true)
        .width_request(200)
        .build();
    content.append(&cp_char_label);

    let cp_hex_label = Label::builder()
        .label(format!("U+{:04X}", codepoint.as_char() as u32))
        .css_classes(["monospace", "sidebar-codepoint-hex", "copyable"])
        .halign(Align::Center)
        .selectable(true)
        .build();
    content.append(&cp_hex_label);

    let cp_copy = Button::builder()
        .icon_name("edit-copy-symbolic")
        .label("Copy character")
        .tooltip_text("Copy character to clipboard")
        .halign(Align::Center)
        .build();
    let cp_char = codepoint.as_char().to_string();
    cp_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&cp_char);
        }
    });
    content.append(&cp_copy);

    #[cfg(feature = "metadata")]
    {
        let keywords = metadata::emoji_keywords(name);
        let category = metadata::emoji_category(name);

        if let Some(cat) = category {
            let cat_heading = Label::builder()
                .label("Category")
                .css_classes(["heading"])
                .halign(Align::Start)
                .margin_top(12)
                .build();
            content.append(&cat_heading);

            let cat_box = FlowBox::builder()
                .column_spacing(4)
                .row_spacing(4)
                .halign(Align::Start)
                .selection_mode(gtk4::SelectionMode::None)
                .build();
            let cat_label = cat.to_string();
            let btn = Button::builder().label(&cat_label).css_classes(["cat-btn"]).build();
            let search_entry_ref = search_entry.clone();
            btn.connect_clicked(move |_| {
                search_entry_ref.set_text(&cat_label);
            });
            cat_box.insert(&btn, -1);
            content.append(&cat_box);
        }

        if !keywords.is_empty() {
            let kw_heading = Label::builder()
                .label("Keywords")
                .css_classes(["heading"])
                .halign(Align::Start)
                .margin_top(12)
                .build();
            content.append(&kw_heading);

            let kw_box = FlowBox::builder()
                .column_spacing(4)
                .row_spacing(4)
                .halign(Align::Start)
                .selection_mode(gtk4::SelectionMode::None)
                .build();
            for kw in keywords.iter() {
                let kw_label = kw.to_string();
                let btn = Button::builder().label(&kw_label).css_classes(["kw-btn"]).build();
                let search_entry_ref = search_entry.clone();
                btn.connect_clicked(move |_| {
                    search_entry_ref.set_text(&kw_label);
                });
                kw_box.insert(&btn, -1);
            }
            content.append(&kw_box);
        }
    }

    let resource_path = format!("{}/scalable/emoji/{}.svg", GRESOURCE_PREFIX, name);
    if let Ok(bytes) = gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE) {
        let svg_xml = String::from_utf8_lossy(bytes.as_ref()).into_owned();

        let svg_heading = Label::builder()
            .label("SVG")
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_top(12)
            .build();
        content.append(&svg_heading);

        let text_view = TextView::builder().editable(false).css_classes(["monospace"]).wrap_mode(WrapMode::Char).build();
        text_view.buffer().set_text(&svg_xml);

        let scrolled = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .min_content_height(150)
            .max_content_height(300)
            .child(&text_view)
            .build();
        content.append(&scrolled);

        let svg_copy = Button::builder()
            .icon_name("edit-copy-symbolic")
            .label("Copy SVG")
            .tooltip_text("Copy SVG XML to clipboard")
            .halign(Align::Start)
            .build();
        let svg_xml_for_copy = svg_xml;
        svg_copy.connect_clicked(move |_| {
            if let Some(display) = gtk4::gdk::Display::default() {
                display.clipboard().set_text(&svg_xml_for_copy);
            }
        });
        content.append(&svg_copy);
    }
}
