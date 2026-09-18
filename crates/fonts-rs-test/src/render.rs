//! ab_glyph software rendering example.
//!
//! Provides [`run_embed_fonts_render`] for rendering text with `ab_glyph`
//! and displaying the result as a GDK memory texture in GTK.

use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::Glyph;
use ab_glyph::ScaleFont;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Picture;
use gtk4::glib;
use gtk4::prelude::*;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app rendering text with `ab_glyph` software rendering.
///
/// Renders the given text into an RGBA pixel buffer using `ab_glyph`,
/// wraps it as a GDK memory texture, and displays it in a `Picture` widget.
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `font`: The `ab_glyph` font to render with.
/// - `text`: Text to render.
/// - `font_size`: Font size in pixels.
///
/// # Errors
///
/// Returns an error if GTK init fails.
pub fn run_embed_fonts_render(
    app_id: &str,
    window_title: &str,
    font: &'static FontVec,
    text: &str,
    font_size: f32,
) -> Result<glib::ExitCode> {
    let text = text.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui(app, &window_title, font, &text, font_size);
    })
}

fn build_ui(
    app: &Application,
    window_title: &str,
    font: &'static FontVec,
    text: &str,
    font_size: f32,
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
        .label("Rendered with ab_glyph (embed-fonts) — no Pango font map")
        .css_classes(["title-2"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let (width, height, pixels) = render_text(font, text, font_size);

    let bytes = glib::Bytes::from_owned(pixels);
    let texture = gtk4::gdk::MemoryTexture::new(
        width,
        height,
        gtk4::gdk::MemoryFormat::R8g8b8a8Premultiplied,
        &bytes,
        (width * 4) as usize,
    );

    let picture = Picture::for_paintable(&texture);
    picture.set_halign(Align::Center);
    picture.set_valign(Align::Center);
    main_box.append(&picture);

    window.set_child(Some(&main_box));
    window.present();
}

/// Renders text into an RGBA pixel buffer using `ab_glyph`.
///
/// Returns `(width, height, pixels)` where `pixels` is RGBA8.
fn render_text(font: &FontVec, text: &str, scale: f32) -> (i32, i32, Vec<u8>) {
    let scaled = font.as_scaled(scale);

    let mut glyphs: Vec<Glyph> = Vec::new();
    let mut x = 0.0_f32;
    let ascent = scaled.ascent();
    let descent = scaled.descent();
    let line_height = ascent - descent;

    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);
        let glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(x, ascent));
        if let Some(outlined) = font.outline_glyph(glyph.clone()) {
            let bounds = outlined.px_bounds();
            x += bounds.width().max(0.0);
        }
        glyphs.push(glyph);
    }

    let total_width = x.ceil().max(1.0) as u32;
    let total_height = line_height.ceil().max(1.0) as u32;

    let mut alpha = vec![0.0_f32; (total_width * total_height) as usize];

    for glyph in &glyphs {
        if let Some(outlined) = font.outline_glyph(glyph.clone()) {
            let bounds = outlined.px_bounds();
            outlined.draw(|gx, gy, coverage| {
                let px = bounds.min.x as i32 + gx as i32;
                let py = bounds.min.y as i32 + gy as i32;
                if px >= 0 && px < total_width as i32 && py >= 0 && py < total_height as i32 {
                    let idx = (py as u32 * total_width + px as u32) as usize;
                    alpha[idx] = alpha[idx].max(coverage);
                }
            });
        }
    }

    let mut pixels = Vec::with_capacity((total_width * total_height * 4) as usize);
    for &a in &alpha {
        let v = (a * 255.0).round() as u8;
        pixels.push(v);
        pixels.push(v);
        pixels.push(v);
        pixels.push(255);
    }

    (total_width as i32, total_height as i32, pixels)
}
