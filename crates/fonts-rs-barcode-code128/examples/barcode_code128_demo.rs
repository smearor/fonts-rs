//! Interactive demo for fonts-rs-barcode-code128.
//!
//! Demonstrates Libre Barcode Code 128 font rendering with a live input field.
//!
//! Run with:
//! ```sh
//! cargo run --example barcode_code128_demo --features gtk
//! ```

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Entry;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.fonts_rs.barcode_code128_demo";

const FONT_FAMILY: &str = "LibreBarcode128";
const FONT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/LibreBarcode128-Regular.ttf");

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    fonts_rs_barcode_code128::register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

fn build_ui(app: &Application) {
    let css = format!(".barcode-display {{\n    font-family: '{FONT_FAMILY}';\n    font-size: 192px;\n    font-feature-settings: \"calt\";\n}}\n");

    let provider = gtk4::CssProvider::new();
    provider.load_from_string(&css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Libre Barcode Code 128 Demo")
        .default_width(700)
        .default_height(300)
        .build();

    if let Some(font_map) = window.pango_context().font_map() {
        if let Err(e) = font_map.add_font_file(FONT_PATH) {
            eprintln!("Failed to load font: {e}");
        }
    } else {
        eprintln!("No Pango font map available");
    }

    window.connect_close_request({
        let app = app.clone();
        move |_| {
            app.quit();
            glib::Propagation::Proceed
        }
    });

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let title = Label::builder()
        .label("Libre Barcode Code 128")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Type text below. The barcode renders using the Code 128 font.\nCode 128 supports all 128 ASCII characters.")
        .halign(Align::Center)
        .wrap(true)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    let write_label = Label::builder().label("Decoded").css_classes(["heading"]).halign(Align::Start).build();
    main_box.append(&write_label);

    let entry = Entry::builder()
        .placeholder_text("Type text or numbers...")
        .text("Hello123")
        .hexpand(true)
        .build();
    main_box.append(&entry);

    let readonly_label_heading = Label::builder()
        .label("Barcode")
        .css_classes(["heading"])
        .halign(Align::Start)
        .margin_top(8)
        .build();
    main_box.append(&readonly_label_heading);

    let barcode_label = Label::builder()
        .label("Hello123")
        .halign(Align::Center)
        .valign(Align::Center)
        .vexpand(true)
        .css_classes(["barcode-display"])
        .build();
    main_box.append(&barcode_label);

    entry.connect_changed(glib::clone!(
        #[weak]
        barcode_label,
        move |e| {
            barcode_label.set_label(&e.text());
        }
    ));

    window.set_child(Some(&main_box));
    window.present();
}
