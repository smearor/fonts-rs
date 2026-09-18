//! Composed SVG text example.
//!
//! Provides [`run_composed_svg_text`] for composing multiple SVG glyph
//! outlines into a single SVG document and displaying it as a GTK texture.

use fonts_rs_model::IconThemeFont;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Picture;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app displaying composed SVG text from GResource glyphs.
///
/// Registers the font's GResource bundles via `F::ensure_registered`,
/// composes the given text into a single SVG document via
/// `F::compose_svg`, and displays the result as a `Picture` widget.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`IconThemeFont`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `text`: Text to compose (each char is resolved via `F::glyph_name`).
/// - `content_label`: Content type label (e.g. `"Emoji"`, `"EAN-Barcode"`).
/// - `glyph_height`: Target height for each glyph in SVG user units.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_composed_svg_text<F: IconThemeFont>(app_id: &str, window_title: &str, text: &str, content_label: &str, glyph_height: f64) -> Result<glib::ExitCode> {
    F::ensure_registered().into_diagnostic()?;

    let text = text.to_string();
    let content_label = content_label.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<F>(app, &window_title, &text, &content_label, glyph_height);
    })
}

fn build_ui<F: IconThemeFont>(app: &Application, window_title: &str, text: &str, content_label: &str, glyph_height: f64) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(800)
        .default_height(300)
        .build();

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let title = Label::builder()
        .label(format!("Composed SVG — glyphs assembled into one {content_label}"))
        .css_classes(["title-2"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let svg = F::compose_svg(text, glyph_height);

    let bytes = glib::Bytes::from_owned(svg.into_bytes());
    match gtk4::gdk::Texture::from_bytes(&bytes) {
        Ok(texture) => {
            let picture = Picture::for_paintable(&texture);
            picture.set_halign(Align::Center);
            picture.set_valign(Align::Center);
            main_box.append(&picture);
        }
        Err(e) => {
            let label = Label::builder()
                .label(format!("Texture error: {e}"))
                .css_classes(["error"])
                .halign(Align::Center)
                .build();
            main_box.append(&label);
        }
    }

    window.set_child(Some(&main_box));
    window.present();
}
