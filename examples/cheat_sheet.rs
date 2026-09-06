//! Nerd Font Cheat Sheet application.
//!
//! Browse all available Nerd Font icons with a search filter.
//! Displays each icon with its Unicode codepoint and CSS class name.
//!
//! Run with:
//! ```sh
//! cargo run --example cheat_sheet --features gtk,render,embed-fonts
//! ```

use nerd_fonts_gtk::init;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Entry;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;

const APP_ID: &str = "io.smearor.nerd_fonts_gtk.cheat_sheet";

/// Collect all icons from the codepoint map as (codepoint, css_class_name) pairs.
fn collect_all_icons() -> Vec<(char, String)> {
    let mut icons: Vec<(char, String)> = nerd_fonts_gtk::icons::codepoint_map::ICONS
        .entries()
        .map(|(c, name)| (*c, name.to_string()))
        .collect();
    icons.sort_by(|a, b| a.1.cmp(&b.1));
    icons
}

/// Strip the `-symbolic` suffix to get the CSS class name.
fn css_class_name(icon_name: &str) -> String {
    icon_name.strip_suffix("-symbolic").unwrap_or(icon_name).to_string()
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    init(None);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Nerd Font Cheat Sheet")
        .default_width(1024)
        .default_height(768)
        .build();

    window.connect_close_request({
        let app = app.clone();
        move |_| {
            app.quit();
            glib::Propagation::Proceed
        }
    });

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("Nerd Font Cheat Sheet")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let count_label = Label::builder().label("").css_classes(["dim-label"]).halign(Align::Center).build();
    main_box.append(&count_label);

    // Search entry
    let search_entry = Entry::builder()
        .placeholder_text("Search by name or codepoint (e.g. 'gamepad' or 'F11B')...")
        .halign(Align::Fill)
        .margin_top(8)
        .margin_bottom(8)
        .build();
    main_box.append(&search_entry);

    // CSS for the icon grid
    let grid_css = ".cheat-icon { font-size: 42px; } .cheat-cell { padding: 10px; border-radius: 8px; } .cheat-cell:hover { background-color: rgba(100, 180, 255, 0.15); } .cheat-name, .cheat-cp { -gtk-user-select: text; }";
    let grid_provider = gtk4::CssProvider::new();
    grid_provider.load_from_data(grid_css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &grid_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    // Scrolled window for the icon grid
    let scrolled = gtk4::ScrolledWindow::builder().hexpand(true).vexpand(true).build();

    let grid = gtk4::Grid::builder()
        .column_spacing(8)
        .row_spacing(8)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    scrolled.set_child(Some(&grid));
    main_box.append(&scrolled);

    let all_icons = collect_all_icons();
    let total = all_icons.len();
    count_label.set_label(&format!("{} icons available", total));

    let cols = 6u32;
    const MAX_DISPLAYED: usize = 300;

    let icons_for_filter = all_icons.clone();
    let render_grid = move |filter: &str| -> Vec<(char, String)> {
        let filter_lower = filter.to_lowercase();
        if filter_lower.is_empty() {
            return icons_for_filter.iter().take(MAX_DISPLAYED).cloned().collect();
        }
        icons_for_filter
            .iter()
            .filter(|(cp, name)| {
                let css = css_class_name(name);
                css.to_lowercase().contains(&filter_lower)
                    || format!("{:X}", *cp as u32).to_lowercase().contains(&filter_lower)
                    || format!("\\u{{{:X}}}", *cp as u32).to_lowercase().contains(&filter_lower)
            })
            .take(MAX_DISPLAYED)
            .cloned()
            .collect()
    };

    let populate_grid = move |grid: &gtk4::Grid, icons: &[(char, String)]| {
        while let Some(child) = grid.first_child() {
            grid.remove(&child);
        }

        for (i, (codepoint, name)) in icons.iter().enumerate() {
            let css_name = css_class_name(name);
            let hex = format!("\\u{{{:X}}}", *codepoint as u32);

            let icon = Label::builder()
                .label(&codepoint.to_string())
                .css_classes(["nerd-icon", "cheat-icon"])
                .halign(Align::Center)
                .build();

            let name_label = Label::builder()
                .label(&css_name)
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

            let vbox = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .halign(Align::Center)
                .css_classes(["cheat-cell"])
                .build();
            vbox.append(&icon);
            vbox.append(&name_label);
            vbox.append(&codepoint_label);

            let col = (i as u32) % cols;
            let row = (i as u32) / cols;
            grid.attach(&vbox, col as i32, row as i32, 1, 1);
        }
        grid.show();
    };

    // Initial population
    let initial_icons: Vec<(char, String)> = all_icons.iter().take(MAX_DISPLAYED).cloned().collect();
    populate_grid(&grid, &initial_icons);
    count_label.set_label(&format!("{} icons (showing first {})", total, initial_icons.len()));

    // Search filtering
    search_entry.connect_changed(glib::clone!(
        #[weak]
        grid,
        #[weak]
        count_label,
        move |entry| {
            let filter = entry.text().to_string();
            let filtered = render_grid(&filter);
            let shown = filtered.len();
            let label = if filter.is_empty() {
                format!("{} icons (showing first {})", total, shown)
            } else {
                format!("{} / {} icons", shown, total)
            };
            count_label.set_label(&label);
            populate_grid(&grid, &filtered);
        }
    ));

    let scrolled_wrap = gtk4::ScrolledWindow::builder().child(&main_box).hexpand(true).vexpand(true).build();

    window.set_child(Some(&scrolled_wrap));
    window.present();
}
