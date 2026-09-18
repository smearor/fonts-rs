//! Composed SVG text example.
//!
//! Provides [`run_composed_svg_text`] for composing multiple SVG glyph
//! outlines into a single SVG document and displaying it as a GTK texture.

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

/// Runs a GTK app displaying composed SVG text from GResource glyphs.
///
/// Registers the font's SVG GResource, composes the given text into a
/// single SVG document by looking up each character's glyph SVG, and
/// displays the result as a `Picture` widget.
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
/// - `gresource_prefix`: GResource prefix for the font family.
/// - `text`: Text to compose (each char is looked up in `char_to_name`).
/// - `content_label`: Content type label (e.g. `"Emoji"`, `"EAN-Barcode"`).
/// - `char_to_name`: Mapping from characters to glyph names.
/// - `glyph_height`: Target height for each glyph in SVG user units.
///
/// # Errors
///
/// Returns an error if GTK init or GResource registration fails.
pub fn run_composed_svg_text<T, C>(
    app_id: &str,
    window_title: &str,
    gresource_prefix: &str,
    text: &str,
    content_label: &str,
    char_to_name: &[(char, &str)],
    glyph_height: f64,
) -> Result<glib::ExitCode>
where
    T: GtkFont,
    C: FontFamilyConfig,
{
    T::register_glyphs().into_diagnostic()?;

    let char_to_name: std::collections::HashMap<char, String> = char_to_name
        .iter()
        .map(|(ch, name)| (*ch, name.to_string()))
        .collect();

    let gresource_prefix = gresource_prefix.to_string();
    let text = text.to_string();
    let content_label = content_label.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui::<C>(app, &window_title, &gresource_prefix, &text, &content_label, &char_to_name, glyph_height);
    })
}

fn build_ui<C: FontFamilyConfig>(
    app: &Application,
    window_title: &str,
    gresource_prefix: &str,
    text: &str,
    content_label: &str,
    char_to_name: &std::collections::HashMap<char, String>,
    glyph_height: f64,
) {
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

    let svg = compose_text_svg::<C>(gresource_prefix, text, char_to_name, glyph_height);

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

/// Composes a text string into a single SVG with all glyphs side by side.
///
/// Each glyph SVG is loaded from GResource, its `viewBox` and inner
/// content extracted, and placed as a nested `<svg>` element at the
/// correct x offset.
fn compose_text_svg<C: FontFamilyConfig>(
    gresource_prefix: &str,
    text: &str,
    char_to_name: &std::collections::HashMap<char, String>,
    glyph_height: f64,
) -> String {
    let prefix = gresource_prefix;

    let mut segments: Vec<(f64, String, String)> = Vec::new();

    for ch in text.chars() {
        let Some(name) = char_to_name.get(&ch) else {
            continue;
        };

        let resource_path = format!(
            "{}/{}/{}/{}.svg",
            prefix,
            SCALABLE_DIR,
            C::ICONS_CONTEXT,
            name
        );
        let Ok(bytes) = gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE)
        else {
            continue;
        };
        let Ok(svg) = std::str::from_utf8(bytes.as_ref()) else {
            continue;
        };

        let viewbox = extract_svg_viewbox(svg).to_string();
        let content = extract_svg_inner(svg).to_string();

        let parts: Vec<f64> = viewbox
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let scaled_width = if parts.len() >= 4 && parts[3] > 0.0 {
            glyph_height * parts[2] / parts[3]
        } else {
            glyph_height * 0.3
        };

        segments.push((scaled_width, viewbox, content));
    }

    let total_width: f64 = segments.iter().map(|(w, _, _)| *w).sum();
    let total_width = total_width.ceil().max(1.0) as u32;
    let total_height = glyph_height.ceil() as u32;

    let mut inner = String::new();
    let mut x = 0.0_f64;
    for (scaled_width, viewbox, content) in &segments {
        let colored = content.replace("currentColor", "#000");
        let w = scaled_width.ceil();
        inner.push_str(&format!(
            "<svg x=\"{x:.0}\" y=\"0\" width=\"{w}\" height=\"{glyph_height:.0}\" viewBox=\"{viewbox}\">{colored}</svg>"
        ));
        x += scaled_width;
    }

    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{total_width}\" height=\"{total_height}\"><rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>{inner}</svg>")
}

/// Extracts the `viewBox` attribute value from an SVG string.
fn extract_svg_viewbox(svg: &str) -> &str {
    let pattern = "viewBox=\"";
    if let Some(start) = svg.find(pattern) {
        let start = start + pattern.len();
        if let Some(end) = svg[start..].find('"') {
            return &svg[start..start + end];
        }
    }
    "0 0 400 1000"
}

/// Extracts the inner content of an SVG (between the opening tag and `</svg>`).
fn extract_svg_inner(svg: &str) -> &str {
    if let Some(svg_start) = svg.find("<svg")
        && let Some(tag_end) = svg[svg_start..].find('>')
    {
        let content_start = svg_start + tag_end + 1;
        if let Some(close) = svg.rfind("</svg>") {
            return &svg[content_start..close];
        }
    }
    ""
}
