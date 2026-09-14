//! Interactive demo for fonts-rs-seven-segment.
//!
//! Demonstrates DSEG7 glyph resolution and GTK icon rendering.
//!
//! Run with:
//! ```sh
//! cargo run --example interactive_demo --features gtk,embed-fonts
//! ```

use fonts_rs_seven_segment::GlyphNameExt;
use fonts_rs_seven_segment::SevenSegmentName;
use fonts_rs_seven_segment::all_glyphs;
use fonts_rs_seven_segment::register_glyphs;
use fonts_rs_seven_segment::variant;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::DropDown;
use gtk4::Entry;
use gtk4::Frame;
use gtk4::Grid;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.fonts_rs.seven_segment_demo";

/// Base glyph names (without variant prefix) exported by the build pipeline.
const GALLERY_GLYPH_BASES: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "colon",
    "hyphen",
    "period",
    "degree",
    "exclam",
    "space",
    "nonmarkingreturn",
];

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

fn build_ui(app: &Application) {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(".dseg7-glyph { color: #00ff00; -gtk-icon-size: 96px; }");
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("fonts-rs-seven-segment Demo")
        .default_width(900)
        .default_height(700)
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
        .label(&format!("DSEG7 — Seven-Segment Display ({})", variant::GLYPH_PREFIX))
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Browse seven-segment display glyphs exported from the DSEG7 font.")
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
// Section 1: Glyph Resolution
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
        .label("1. Glyph Name Resolution")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Enter a glyph name (e.g. zero) to resolve its Unicode codepoint.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let input_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let entry = Entry::builder().placeholder_text("zero").text("zero").hexpand(true).build();
    input_row.append(&entry);

    let resolve_button = Button::with_label("Resolve");
    input_row.append(&resolve_button);

    section.append(&input_row);

    let result_label = Label::builder().label("Codepoint: —").halign(Align::Start).css_classes(["dim-label"]).build();
    section.append(&result_label);

    let quick_label = Label::builder().label("Or pick from common glyphs:").halign(Align::Start).build();
    section.append(&quick_label);

    let dropdown_names: Vec<String> = GALLERY_GLYPH_BASES.iter().map(|base| format!("{}-{}", variant::GLYPH_PREFIX, base)).collect();
    let dropdown_refs: Vec<&str> = dropdown_names.iter().map(|s| s.as_str()).collect();
    let dropdown = DropDown::from_strings(&dropdown_refs);
    section.append(&dropdown);

    do_resolve(&result_label, "zero");

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

    Frame::builder().label("Glyph Resolution").child(&section).build()
}

fn do_resolve(result_label: &Label, name: &str) {
    if name.is_empty() {
        result_label.set_label("Codepoint: —");
        return;
    }
    let glyph_name = SevenSegmentName::new(name.to_string());
    let cp = glyph_name.codepoint();
    match cp {
        Some(c) => result_label.set_label(&format!("Codepoint: U+{:04X} ('{}')", c.as_char() as u32, c.as_char())),
        None => result_label.set_label("Codepoint: no Unicode mapping (font-specific glyph)"),
    }
}

// ---------------------------------------------------------------------------
// Section 2: Glyph Gallery
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

    let header = Label::builder().label("2. Glyph Gallery").css_classes(["title-3"]).halign(Align::Start).build();
    section.append(&header);

    let desc = Label::builder()
        .label(&format!("DSEG7 glyphs ({}) rendered as GTK icons via GResource registration.", variant::GLYPH_PREFIX))
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let grid = Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let cols = 6u32;

    for (i, base_name) in GALLERY_GLYPH_BASES.iter().enumerate() {
        let full_name = format!("{}-{}", variant::GLYPH_PREFIX, base_name);
        let resource_path = format!("{}/scalable/glyphs/{}.svg", variant::GRESOURCE_PREFIX, full_name,);
        let icon = gtk4::Image::from_resource(&resource_path);
        icon.add_css_class("dseg7-glyph");

        let label = Label::builder()
            .label(&full_name)
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

    // Show total glyph count
    let count = all_glyphs().count();
    let count_label = Label::builder()
        .label(&format!("Total exported glyphs: {count}"))
        .halign(Align::Center)
        .css_classes(["dim-label"])
        .build();
    section.append(&count_label);

    Frame::builder().label("Glyph Gallery").child(&section).build()
}
