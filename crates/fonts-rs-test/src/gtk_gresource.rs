//! GTK GResource font loading example.
//!
//! Provides [`run_gtk_gresource`] for loading a TTF font from GResource
//! into the Pango font map and rendering text with CSS `font-family`.

use fonts_rs_model::GtkFont;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use pango::prelude::FontMapExt;
use miette::IntoDiagnostic;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app loading a TTF font from GResource and rendering text.
///
/// Registers the font's SVG and TTF GResource, loads the TTF into the
/// Pango font map, and displays the text with a CSS `font-family` rule.
///
/// # Type Parameters
///
/// - `T`: Font marker type implementing [`GtkFont`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `font_family`: CSS `font-family` name.
/// - `text`: Text to render.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_gtk_gresource<T: GtkFont>(
    app_id: &str,
    window_title: &str,
    font_family: &str,
    text: &str,
) -> Result<glib::ExitCode> {
    T::register_glyphs().into_diagnostic()?;
    T::register_font().into_diagnostic()?;

    let font_family = font_family.to_string();
    let text = text.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<T>(app, &window_title, &font_family, &text);
    })
}

fn build_ui<T: GtkFont>(
    app: &Application,
    window_title: &str,
    font_family: &str,
    text: &str,
) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(600)
        .default_height(200)
        .build();

    if let Some(font_map) = window.pango_context().font_map() {
        match T::load_font() {
            Ok(path) => {
                if let Err(e) = font_map.add_font_file(path) {
                    eprintln!("Failed to add font to Pango: {e}");
                }
            }
            Err(e) => eprintln!("Failed to load font from GResource: {e}"),
        }
    }

    let css = format!(".font-text {{ font-family: '{font_family}'; font-size: 192px; }}");
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(&css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let label = Label::builder()
        .label(text)
        .css_classes(["font-text"])
        .halign(Align::Center)
        .valign(Align::Center)
        .vexpand(true)
        .build();

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .build();
    main_box.append(&label);

    window.set_child(Some(&main_box));
    window.present();
}
