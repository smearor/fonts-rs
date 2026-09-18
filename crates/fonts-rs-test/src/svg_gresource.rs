//! SVG GResource example.
//!
//! Provides [`run_svg_gresource`] for loading individual SVG glyphs from
//! GResource and displaying them as GDK textures in a GTK grid.

use fonts_rs_model::FontFamilyConfig;
use fonts_rs_model::GtkFont;
use fonts_rs_model::SCALABLE_DIR;
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

/// Runs a GTK app displaying SVG glyphs from GResource as textures.
///
/// Registers the font's SVG GResource and displays the first few glyphs
/// as `Picture` widgets loaded from GResource.
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
/// - `glyphs`: Slice of `(codepoint, glyph_name)` pairs to display.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_svg_gresource<T, C>(app_id: &str, window_title: &str, gresource_prefix: &str, glyphs: &[(char, &str)]) -> Result<glib::ExitCode>
where
    T: GtkFont,
    C: FontFamilyConfig,
{
    T::register_glyphs().into_diagnostic()?;

    let glyphs: Vec<(char, String)> = glyphs.iter().map(|(ch, name)| (*ch, name.to_string())).collect();

    let gresource_prefix = gresource_prefix.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<C>(app, &window_title, &gresource_prefix, &glyphs);
    })
}

fn build_ui<C: FontFamilyConfig>(app: &Application, window_title: &str, gresource_prefix: &str, glyphs: &[(char, String)]) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title)
        .default_width(600)
        .default_height(400)
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
        .label("SVG Glyphs from GResource")
        .css_classes(["title-2"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Each glyph is loaded as SVG from the GResource bundle and rendered as a texture.")
        .halign(Align::Center)
        .wrap(true)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    let grid = Box::builder().orientation(Orientation::Horizontal).spacing(12).halign(Align::Center).build();

    let prefix = gresource_prefix;

    for (_codepoint, name) in glyphs.iter().take(4) {
        let resource_path = format!("{}/{}/{}/{}.svg", prefix, SCALABLE_DIR, C::ICONS_CONTEXT, name);

        match gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE) {
            Ok(bytes) => match gtk4::gdk::Texture::from_bytes(&glib::Bytes::from(bytes.as_ref())) {
                Ok(texture) => {
                    let picture = Picture::for_paintable(&texture);
                    picture.set_halign(Align::Center);
                    picture.set_valign(Align::Center);
                    picture.set_size_request(64, 64);

                    let vbox = Box::builder().orientation(Orientation::Vertical).spacing(4).halign(Align::Center).build();
                    vbox.append(&picture);

                    let caption = Label::builder().label(name).css_classes(["caption", "dim-label"]).halign(Align::Center).build();
                    vbox.append(&caption);

                    grid.append(&vbox);
                }
                Err(e) => {
                    let label = Label::builder()
                        .label(format!("{name}\nTexture error: {e}"))
                        .css_classes(["caption", "error"])
                        .halign(Align::Center)
                        .build();
                    grid.append(&label);
                }
            },
            Err(e) => {
                let label = Label::builder()
                    .label(format!("{name}\nLookup error: {e}"))
                    .css_classes(["caption", "error"])
                    .halign(Align::Center)
                    .build();
                grid.append(&label);
            }
        }
    }

    main_box.append(&grid);
    window.set_child(Some(&main_box));
    window.present();
}
