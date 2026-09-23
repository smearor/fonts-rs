//! GtkIconTheme example.
//!
//! Provides [`run_gtk_icon_theme`] for displaying font glyphs as GTK icons
//! resolved by `GtkIconTheme` via their kebab-case icon name constants.

use fonts_rs_model::IconThemeFont;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Image;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app displaying font glyphs as `GtkIconTheme` icons.
///
/// Registers the font's GResource bundles via `F::ensure_registered`,
/// adds the family's icon resource path to `GtkIconTheme` via
/// `F::install_icon_theme`, and displays each glyph as a `GtkImage`
/// resolved by its icon name.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`IconThemeFont`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `icons`: Slice of `(icon_name, label)` pairs to display. Both
///   accept `&str`, `String`, and the typed `F::Name` (`GlyphName<F>` /
///   `IconName`) via `AsRef<str>`.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_gtk_icon_theme<F: IconThemeFont>(app_id: &str, window_title: &str, icons: &[(impl AsRef<str>, impl AsRef<str>)]) -> Result<glib::ExitCode> {
    F::ensure_registered().into_diagnostic()?;

    let icons: Vec<(String, String)> = icons.iter().map(|(name, label)| (name.as_ref().to_string(), label.as_ref().to_string())).collect();

    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<F>(app, &window_title, &icons);
    })
}

fn build_ui<F: IconThemeFont>(app: &Application, window_title: &str, icons: &[(String, String)]) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(600)
        .default_height(400)
        .build();

    let display = gtk4::prelude::RootExt::display(&window);
    if let Err(e) = F::install_icon_theme(&display) {
        eprintln!("Failed to install icon theme for {}: {e}", F::FAMILY_DISPLAY_NAME);
    }

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let title = Label::builder()
        .label("GtkIconTheme Lookup")
        .css_classes(["title-2"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Glyphs are resolved by GtkIconTheme using their kebab-case icon name constants.")
        .halign(Align::Center)
        .wrap(true)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    let grid = Box::builder().orientation(Orientation::Horizontal).spacing(12).halign(Align::Center).build();

    for (icon_name, label) in icons {
        let image = Image::from_icon_name(icon_name);
        image.set_pixel_size(64);
        image.set_halign(Align::Center);
        image.set_valign(Align::Center);

        let vbox = Box::builder().orientation(Orientation::Vertical).spacing(4).halign(Align::Center).build();
        vbox.append(&image);

        let caption = Label::builder()
            .label(label)
            .css_classes(["caption", "dim-label"])
            .halign(Align::Center)
            .build();
        vbox.append(&caption);

        grid.append(&vbox);
    }

    main_box.append(&grid);
    window.set_child(Some(&main_box));
    window.present();
}
