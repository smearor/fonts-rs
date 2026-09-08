//! Interactive demo for nerd-fonts-gtk.
//!
//! Demonstrates icon name resolution and GTK icon name resolution.
//!
//! Run with:
//! ```sh
//! cargo run --example interactive_demo --features gtk,embed-fonts
//! ```

use nerd_fonts_model::IconName;
use nerd_fonts_rs::gtk::resolve_gtk_nerd_icon;
use nerd_fonts_rs::icons::IconNameExt;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::DropDown;
use gtk4::Entry;
use gtk4::Frame;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.nerd_fonts_gtk.interactive_demo";

/// A curated selection of Nerd Font icons to display in the icon gallery.
const GALLERY_ICONS: &[&str] = &[
    "nf-fa-gamepad",
    "nf-md-cube",
    "nf-linux-tux",
    "nf-fa-star",
    "nf-md-cog",
    "nf-fa-heart",
    "nf-md-battery",
    "nf-fa-music",
    "nf-md-weather_sunny",
    "nf-fa-camera",
    "nf-md-email",
    "nf-fa-bug",
];

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    nerd_fonts_rs::init().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("nerd-fonts-gtk Interactive Demo")
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
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("nerd-fonts-gtk Interactive Demo")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Explore icon resolution and browse the Nerd Font icon gallery.")
        .halign(Align::Center)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    main_box.append(&build_resolution_section());
    main_box.append(&build_gallery_section());

    let scrolled = gtk4::ScrolledWindow::builder().child(&main_box).hexpand(true).vexpand(true).build();

    window.set_child(Some(&scrolled));
    window.present();
}

// ---------------------------------------------------------------------------
// Section 1: Icon Resolution
// ---------------------------------------------------------------------------

fn build_resolution_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("1. Icon Name Resolution")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Enter a Nerd Font icon name (e.g. nf-fa-gamepad) to resolve its Unicode codepoint and GTK icon name.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let input_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let entry = Entry::builder().placeholder_text("nf-fa-gamepad").text("nf-fa-gamepad").hexpand(true).build();
    input_row.append(&entry);

    let resolve_button = Button::with_label("Resolve");
    input_row.append(&resolve_button);

    section.append(&input_row);

    let result_label = Label::builder()
        .label("Codepoint: —\nGTK icon name: —")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    section.append(&result_label);

    // Quick-select dropdown
    let quick_label = Label::builder().label("Or pick from common icons:").halign(Align::Start).build();
    section.append(&quick_label);

    let icon_names: Vec<&str> = GALLERY_ICONS.to_vec();
    let dropdown = DropDown::from_strings(&icon_names);
    section.append(&dropdown);

    // Initial resolution
    do_resolve(&result_label, "nf-fa-gamepad");

    resolve_button.connect_clicked(glib::clone!(
        #[weak]
        entry,
        #[weak]
        result_label,
        move |_| {
            let name = entry.text().to_string();
            do_resolve(&result_label, &name);
        }
    ));

    entry.connect_activate(glib::clone!(
        #[weak]
        resolve_button,
        move |_| {
            resolve_button.emit_clicked();
        }
    ));

    dropdown.connect_selected_notify(glib::clone!(
        #[weak]
        entry,
        #[weak]
        resolve_button,
        move |dd| {
            if let Some(text) = dd.selected_item().and_then(|i| i.downcast::<gtk4::StringObject>().ok()) {
                entry.set_text(text.string().as_str());
                resolve_button.emit_clicked();
            }
        }
    ));

    Frame::builder().label("Icon Resolution").child(&section).build()
}

fn do_resolve(result_label: &Label, name: &str) {
    if name.is_empty() {
        result_label.set_label("Codepoint: —\nGTK icon name: —");
        return;
    }
    let cp = IconName::from_glyph_name(name.strip_prefix("nf-").unwrap_or(name))
        .and_then(|n| n.codepoint())
        .map(|c| c.as_char());
    let gtk_name = resolve_gtk_nerd_icon(name);
    match cp {
        Some(c) => result_label.set_label(&format!("Codepoint: U+{:04X} ('{}')\nGTK icon name: {}", c as u32, c, gtk_name.as_deref().unwrap_or("—"))),
        None => result_label.set_label(&format!("Codepoint: not found\nGTK icon name: {}", gtk_name.as_deref().unwrap_or("—"))),
    }
}

// ---------------------------------------------------------------------------
// Section 2: Icon Gallery
// ---------------------------------------------------------------------------

fn build_gallery_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder().label("2. Icon Gallery").css_classes(["title-3"]).halign(Align::Start).build();
    section.append(&header);

    let desc = Label::builder()
        .label("Nerd Font icons rendered as labels using the @font-face CSS loaded by init().")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    // Icon grid - use Labels with Nerd Font codepoints + nerd-icon CSS class
    let grid = gtk4::Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let cols = 6u32;

    let gallery_css = ".nerd-icon-large { font-size: 32px; } .nerd-icon-clickable { padding: 6px; border-radius: 6px; } .nerd-icon-clickable:hover { background-color: rgba(100, 180, 255, 0.15); }";
    let gallery_provider = gtk4::CssProvider::new();
    gallery_provider.load_from_data(gallery_css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &gallery_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    for (i, icon_name) in GALLERY_ICONS.iter().enumerate() {
        let codepoint = IconName::from_glyph_name(icon_name.strip_prefix("nf-").unwrap_or(icon_name))
            .and_then(|n| n.codepoint())
            .map(|c| c.as_char().to_string())
            .unwrap_or_else(|| "?".to_string());

        let icon = Label::builder()
            .label(&codepoint)
            .css_classes(["nerd-icon", "nerd-icon-large", "nerd-icon-clickable"])
            .halign(Align::Center)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        let label = Label::builder()
            .label(*icon_name)
            .css_classes(["caption", "dim-label"])
            .wrap(true)
            .max_width_chars(18)
            .build();

        let vbox = Box::builder().orientation(Orientation::Vertical).spacing(4).halign(Align::Center).build();
        vbox.append(&icon);
        vbox.append(&label);

        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        grid.attach(&vbox, col as i32, row as i32, 1, 1);
    }

    section.append(&grid);

    Frame::builder().label("Icon Gallery").child(&section).build()
}
