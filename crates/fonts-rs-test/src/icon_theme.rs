//! GtkIconTheme example.
//!
//! Provides [`run_gtk_icon_theme`] for displaying font glyphs as GTK icons
//! resolved by `GtkIconTheme` via their kebab-case icon name constants.

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GtkFont;
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
/// Registers the font's SVG GResource, adds the icons resource path to
/// `GtkIconTheme`, and displays each glyph as a `GtkImage` resolved by
/// its icon name.
///
/// # Type Parameters
///
/// - `T`: Font marker type implementing [`GtkFont`].
/// - `C`: Font config type implementing [`FontFamilyConfig`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `icons`: Slice of `(icon_name, label)` pairs to display.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_gtk_icon_theme<T, C>(app_id: &str, window_title: &str, gresource_prefix: &str, icons: &[(&str, &str)]) -> Result<glib::ExitCode>
where
    T: GtkFont,
    C: FontFamilyConfig,
{
    T::register_glyphs().into_diagnostic()?;

    let icons: Vec<(String, String)> = icons.iter().map(|(name, label)| (name.to_string(), label.to_string())).collect();

    let gresource_prefix = gresource_prefix.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<C>(app, &window_title, &gresource_prefix, &icons);
    })
}

fn build_ui<C: FontFamilyConfig>(app: &Application, window_title: &str, gresource_prefix: &str, icons: &[(String, String)]) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(600)
        .default_height(400)
        .build();

    let display = gtk4::prelude::RootExt::display(&window);
    let icon_theme = gtk4::IconTheme::for_display(&display);
    let glyphs_resource_path = format!("{}/{}/{}", gresource_prefix, fonts_rs_model::SCALABLE_DIR, C::ICONS_CONTEXT);
    icon_theme.add_resource_path(&glyphs_resource_path);

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
