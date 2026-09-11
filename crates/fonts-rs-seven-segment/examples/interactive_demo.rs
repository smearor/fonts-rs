//! Interactive demo for fonts-rs-seven-segment.
//!
//! Demonstrates DSEG7 glyph resolution and GTK icon rendering.
//!
//! Run with:
//! ```sh
//! cargo run --example interactive_demo --features gtk,embed-fonts
//! ```

use fonts_rs_generator::FontDefinition;
use fonts_rs_seven_segment::GlyphNameExt;
use fonts_rs_seven_segment::SevenSegmentName;
use fonts_rs_seven_segment::all_glyphs;
use fonts_rs_seven_segment::register_glyphs;

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

/// All DSEG7 glyphs exported by the build pipeline.
const GALLERY_GLYPHS: &[&str] = &[
    "dseg7-zero", "dseg7-one", "dseg7-two", "dseg7-three", "dseg7-four",
    "dseg7-five", "dseg7-six", "dseg7-seven", "dseg7-eight", "dseg7-nine",
    "dseg7-a", "dseg7-b", "dseg7-c", "dseg7-d", "dseg7-e", "dseg7-f",
    "dseg7-g", "dseg7-h", "dseg7-i", "dseg7-j", "dseg7-k", "dseg7-l",
    "dseg7-m", "dseg7-n", "dseg7-o", "dseg7-p", "dseg7-q", "dseg7-r",
    "dseg7-s", "dseg7-t", "dseg7-u", "dseg7-v", "dseg7-w", "dseg7-x",
    "dseg7-y", "dseg7-z",
    "dseg7-colon", "dseg7-hyphen", "dseg7-period", "dseg7-degree",
    "dseg7-exclam", "dseg7-space", "dseg7-nonmarkingreturn",
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
    provider.load_from_string(
        ".dseg7-glyph { color: #00ff00; -gtk-icon-size: 96px; }"
    );
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
        .label("DSEG7 Classic — Seven-Segment Display")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Browse seven-segment display glyphs exported from the DSEG7 Classic font.")
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
        .label("Enter a DSEG7 glyph name (e.g. dseg7-zero) to resolve its Unicode codepoint.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let input_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let entry = Entry::builder().placeholder_text("dseg7-zero").text("dseg7-zero").hexpand(true).build();
    input_row.append(&entry);

    let resolve_button = Button::with_label("Resolve");
    input_row.append(&resolve_button);

    section.append(&input_row);

    let result_label = Label::builder()
        .label("Codepoint: —")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    section.append(&result_label);

    let quick_label = Label::builder().label("Or pick from common glyphs:").halign(Align::Start).build();
    section.append(&quick_label);

    let dropdown = DropDown::from_strings(GALLERY_GLYPHS);
    section.append(&dropdown);

    do_resolve(&result_label, "dseg7-zero");

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
        .label("DSEG7 glyphs rendered as GTK icons via GResource registration.")
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

    for (i, glyph_name) in GALLERY_GLYPHS.iter().enumerate() {
        let resource_path = format!(
            "{}/scalable/glyphs/{}.svg",
            fonts_rs_seven_segment::naming::SevenSegmentDefinition::GRESOURCE_PREFIX,
            glyph_name,
        );
        let icon = gtk4::Image::from_resource(&resource_path);
        icon.add_css_class("dseg7-glyph");

        let label = Label::builder()
            .label(*glyph_name)
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
