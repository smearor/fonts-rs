//! Interactive demo for fonts-rs-barcode-ean13.
//!
//! Demonstrates Libre Barcode EAN13 font rendering with a live input field.
//!
//! Run with:
//! ```sh
//! cargo run --example barcode_ean13_demo --features gtk
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

const APP_ID: &str = "io.smearor.fonts_rs.barcode_ean13_demo";

const FONT_FAMILY: &str = "LibreBarcodeEAN13Text";
const FONT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/LibreBarcodeEAN13Text-Regular.ttf");

/// EAN13 set A/B parity patterns indexed by the first digit (0-9).
const PARITY_PATTERNS: &[[char; 6]] = &[
    ['A', 'A', 'A', 'A', 'A', 'A'], // 0
    ['A', 'A', 'B', 'A', 'B', 'B'], // 1
    ['A', 'A', 'B', 'B', 'A', 'B'], // 2
    ['A', 'A', 'B', 'B', 'B', 'A'], // 3
    ['A', 'B', 'A', 'A', 'B', 'B'], // 4
    ['A', 'B', 'B', 'A', 'A', 'B'], // 5
    ['A', 'B', 'B', 'B', 'A', 'A'], // 6
    ['A', 'B', 'A', 'B', 'A', 'B'], // 7
    ['A', 'B', 'A', 'B', 'B', 'A'], // 8
    ['A', 'B', 'B', 'A', 'B', 'A'], // 9
];

/// Computes the EAN13 check digit from the first 12 digits.
fn compute_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            let weight = if i % 2 == 0 { 1 } else { 3 };
            d as u32 * weight
        })
        .sum();
    (10 - (sum % 10)) as u8 % 10
}

/// Encodes a 12-digit EAN13 string (without check digit) or 13-digit string
/// (with check digit) into the character sequence the EAN13 font expects.
///
/// Returns the encoded string plus the check digit, or an error message.
fn encode_ean13(input: &str) -> Result<String, String> {
    let digits: Vec<u8> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    if digits.len() < 12 || digits.len() > 13 {
        return Err(format!("Need 12 or 13 digits, got {}", digits.len()));
    }

    let check_digit = if digits.len() == 13 { digits[12] } else { compute_check_digit(&digits) };

    let first = digits[0];
    if first > 9 {
        return Err("Invalid first digit".to_string());
    }

    let pattern = &PARITY_PATTERNS[first as usize];

    let mut encoded = String::new();

    // Start guard
    encoded.push(':');

    // Left 6 digits: digits[1..7] encoded with Set A or B based on parity pattern
    for (i, &d) in digits[1..7].iter().enumerate() {
        let ch = match pattern[i] {
            'A' => char::from(b'A' + d),
            'B' => char::from(b'K' + d),
            _ => unreachable!(),
        };
        encoded.push(ch);
    }

    // Centre guard
    encoded.push('*');

    // Right 6 digits: digits[7..12] encoded with Set C
    for &d in &digits[7..12] {
        encoded.push(char::from(b'a' + d));
    }

    // Check digit (Set C)
    encoded.push(char::from(b'a' + check_digit));

    // End guard
    encoded.push(':');

    Ok(encoded)
}

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    fonts_rs_barcode_ean13::register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

fn build_ui(app: &Application) {
    let css = format!(".barcode-display {{\n    font-family: '{FONT_FAMILY}';\n    font-size: 192px;\n}}\n");

    let provider = gtk4::CssProvider::new();
    provider.load_from_string(&css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Libre Barcode EAN13 Demo")
        .default_width(700)
        .default_height(350)
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
        .label("Libre Barcode EAN13")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Type 12 or 13 digits below. The barcode is encoded and rendered with the EAN13 font.\nThe check digit is calculated automatically if you enter 12 digits.")
        .halign(Align::Center)
        .wrap(true)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    let write_label = Label::builder().label("Decoded").css_classes(["heading"]).halign(Align::Start).build();
    main_box.append(&write_label);

    let entry = Entry::builder()
        .placeholder_text("Type 12 or 13 digits...")
        .text("123456789012")
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
        .label("")
        .halign(Align::Center)
        .valign(Align::Center)
        .vexpand(true)
        .css_classes(["barcode-display"])
        .build();
    main_box.append(&barcode_label);

    let status_label = Label::builder().label("").halign(Align::Center).css_classes(["dim-label"]).build();
    main_box.append(&status_label);

    let update = |text: &str, barcode_label: &Label, status_label: &Label| match encode_ean13(text) {
        Ok(encoded) => {
            let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            let check = if digits.len() == 12 {
                let ds: Vec<u8> = digits.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8)).collect();
                compute_check_digit(&ds)
            } else {
                digits.chars().last().and_then(|c| c.to_digit(10)).map(|d| d as u8).unwrap_or(0)
            };
            barcode_label.set_label(&encoded);
            status_label.set_label(&format!("Encoded: {encoded}  |  Check digit: {check}"));
        }
        Err(e) => {
            barcode_label.set_label("");
            status_label.set_label(&e);
        }
    };

    update("123456789012", &barcode_label, &status_label);

    entry.connect_changed(glib::clone!(
        #[weak]
        barcode_label,
        #[weak]
        status_label,
        move |e| {
            update(&e.text(), &barcode_label, &status_label);
        }
    ));

    window.set_child(Some(&main_box));
    window.present();
}
