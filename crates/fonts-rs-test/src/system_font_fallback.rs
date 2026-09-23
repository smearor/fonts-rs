//! System font fallback example.
//!
//! Provides [`run_system_font_fallback`] for demonstrating what happens when
//! the font is NOT loaded — GTK falls back to the system font.

use fonts_rs_model::FontFamily;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app showing system font fallback.
///
/// Displays text with a CSS `font-family` referencing `F::CSS_FONT_FAMILY`
/// while the font is deliberately NOT registered, so GTK falls back to
/// the system default font.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`FontFamily`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `text`: Text to display.
/// - `hint`: Explanation text shown below the rendered text.
///
/// # Errors
///
/// Returns an error if GTK init fails.
pub fn run_system_font_fallback<F: FontFamily>(app_id: &str, window_title: &str, text: &str, hint: &str) -> Result<glib::ExitCode> {
    let text = text.to_string();
    let hint = hint.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<F>(app, &window_title, &text, &hint);
    })
}

fn build_ui<F: FontFamily>(app: &Application, window_title: &str, text: &str, hint: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(600)
        .default_height(200)
        .build();

    let font_family = F::CSS_FONT_FAMILY;
    let css = format!(".font-text {{ font-family: '{font_family}'; font-size: 192px; }}");
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(&css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let label = Label::builder()
        .label(text)
        .css_classes(["font-text"])
        .halign(Align::Center)
        .valign(Align::Center)
        .vexpand(true)
        .build();

    let hint_label = Label::builder().label(hint).css_classes(["dim-label"]).halign(Align::Center).build();

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(24)
        .margin_bottom(24)
        .build();
    main_box.append(&label);
    main_box.append(&hint_label);

    window.set_child(Some(&main_box));
    window.present();
}
