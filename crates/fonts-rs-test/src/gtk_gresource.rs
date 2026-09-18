//! GTK GResource font loading example.
//!
//! Provides [`run_gtk_gresource`] for loading a TTF font from GResource
//! into the Pango font map and rendering text with CSS `font-family`.

use fonts_rs_model::PangoFont;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app loading a TTF font from GResource and rendering text.
///
/// Registers the font's GResource bundles via `F::ensure_registered`,
/// loads the TTF into the Pango font map via `F::install_pango`, and
/// displays the text with a CSS `font-family` rule using
/// `F::CSS_FONT_FAMILY`.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`PangoFont`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `text`: Text to render.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_gtk_gresource<F: PangoFont>(app_id: &str, window_title: &str, text: &str) -> Result<glib::ExitCode> {
    F::ensure_registered().into_diagnostic()?;

    let text = text.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<F>(app, &window_title, &text);
    })
}

fn build_ui<F: PangoFont>(app: &Application, window_title: &str, text: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(600)
        .default_height(200)
        .build();

    if let Some(font_map) = window.pango_context().font_map()
        && let Err(e) = F::install_pango(&font_map)
    {
        eprintln!("Failed to install {} into Pango: {e}", F::FAMILY_DISPLAY_NAME);
    }

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

    let main_box = Box::builder().orientation(Orientation::Vertical).margin_top(24).margin_bottom(24).build();
    main_box.append(&label);

    window.set_child(Some(&main_box));
    window.present();
}
