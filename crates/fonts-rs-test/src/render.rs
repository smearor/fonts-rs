//! ab_glyph software rendering example.
//!
//! Provides [`run_embed_fonts_render`] for rendering text with `ab_glyph`
//! and displaying the result as a GDK memory texture in GTK.

use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::GlyphId;
use ab_glyph::GlyphImageFormat;
use ab_glyph::ScaleFont;
use ab_glyph::v2::GlyphImage;
use fonts_rs_model::SoftwareRender;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Picture;
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::glib;
use gtk4::prelude::*;
use miette::Result;

use crate::app::run_gtk_app;

/// Runs a GTK app rendering text with `ab_glyph` software rendering.
///
/// Resolves the family's `ab_glyph` font via `F::font()`, renders the
/// given text into an RGBA pixel buffer, wraps it as a GDK memory
/// texture, and displays it in a `Picture` widget.
///
/// # Type Parameters
///
/// - `F`: Font family implementing [`SoftwareRender`].
///
/// # Parameters
///
/// - `app_id`: Unique GTK application ID.
/// - `window_title`: Window title text.
/// - `text`: Text to render.
/// - `font_size`: Font size in pixels.
///
/// # Errors
///
/// Returns an error if GTK init fails or the font is not available.
pub fn run_embed_fonts_render<F: SoftwareRender>(app_id: &str, window_title: &str, text: &str, font_size: f32) -> Result<glib::ExitCode> {
    let font = F::font().ok_or_else(|| miette::miette!("{}: no ab_glyph font available", F::FAMILY_DISPLAY_NAME))?;

    let text = text.to_string();
    let window_title = window_title.to_string();

    run_gtk_app(app_id, move |app| {
        build_ui(app, &window_title, font, &text, font_size);
    })
}

fn build_ui(app: &Application, window_title: &str, font: &'static FontVec, text: &str, font_size: f32) {
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
        .label("Rendered with ab_glyph (embed-fonts) - no Pango font map")
        .css_classes(["title-2"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let (width, height, pixels) = render_text(font, text, font_size);
    let non_black = pixels.chunks_exact(4).filter(|px| px[0] > 0 || px[1] > 0 || px[2] > 0).count();
    eprintln!("render_text: {width}x{height}, non_black={non_black}");

    let bytes = glib::Bytes::from_owned(pixels);
    let texture = gtk4::gdk::MemoryTexture::new(width, height, gtk4::gdk::MemoryFormat::R8g8b8a8Premultiplied, &bytes, (width * 4) as usize);

    let picture = Picture::for_paintable(&texture);
    picture.set_halign(Align::Center);
    picture.set_valign(Align::Center);
    main_box.append(&picture);

    window.set_child(Some(&main_box));
    window.present();
}

/// Renders text into an RGBA pixel buffer using `ab_glyph`.
///
/// Outline glyphs are rasterized as white coverage on opaque black.
/// Bitmap strikes (CBDT/CBLC color fonts like Noto Color Emoji, which
/// carry no outlines) are blitted via `glyph_raster_image2`.
///
/// Returns `(width, height, pixels)` where `pixels` is RGBA8.
pub fn render_text(font: &FontVec, text: &str, scale: f32) -> (i32, i32, Vec<u8>) {
    let scaled = font.as_scaled(scale);

    // Pen advances come from hmtx - available for outline and bitmap
    // glyphs alike (outline bounds are not a reliable advance).
    let mut glyphs: Vec<(GlyphId, f32)> = Vec::new();
    let mut x = 0.0_f32;
    let ascent = scaled.ascent();
    let descent = scaled.descent();
    let line_height = ascent - descent;

    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);
        glyphs.push((glyph_id, x));
        x += scaled.h_advance(glyph_id);
    }

    let total_width = x.ceil().max(1.0) as u32;
    let total_height = line_height.ceil().max(1.0) as u32;

    let mut alpha = vec![0.0_f32; (total_width * total_height) as usize];
    let mut pixels = vec![0u8; (total_width * total_height * 4) as usize];
    for px in pixels.chunks_exact_mut(4) {
        px[3] = 255;
    }

    for (glyph_id, pen_x) in &glyphs {
        let glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(*pen_x, ascent));
        if let Some(outlined) = font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            outlined.draw(|gx, gy, coverage| {
                let px = bounds.min.x as i32 + gx as i32;
                let py = bounds.min.y as i32 + gy as i32;
                if px >= 0 && px < total_width as i32 && py >= 0 && py < total_height as i32 {
                    let idx = (py as u32 * total_width + px as u32) as usize;
                    alpha[idx] = alpha[idx].max(coverage);
                }
            });
        } else if let Some(image) = font.glyph_raster_image2(*glyph_id, scale as u16) {
            blit_raster_image(&image, *pen_x, ascent, scale, &mut pixels, total_width, total_height);
        }
    }

    for (idx, &a) in alpha.iter().enumerate() {
        let v = (a * 255.0).round() as u8;
        let px = &mut pixels[idx * 4..idx * 4 + 4];
        px[0] = px[0].max(v);
        px[1] = px[1].max(v);
        px[2] = px[2].max(v);
    }

    (total_width as i32, total_height as i32, pixels)
}

/// Blits a bitmap strike into the RGBA buffer.
///
/// `glyph_raster_image2` returns the best matching strike, whose
/// `pixels_per_em` may differ from the requested size - the image is
/// rescaled by `scale / pixels_per_em` with nearest-neighbor sampling
/// and positioned by its bearings (`origin`).
fn blit_raster_image(image: &GlyphImage, pen_x: f32, baseline: f32, scale: f32, pixels: &mut [u8], width: u32, height: u32) {
    if image.pixels_per_em == 0 {
        return;
    }
    let Some((src_w, src_h, src)) = raster_image_rgba(image) else {
        return;
    };
    let factor = scale / f32::from(image.pixels_per_em);
    let dest_w = (src_w as f32 * factor).round().max(1.0) as i32;
    let dest_h = (src_h as f32 * factor).round().max(1.0) as i32;
    let left = (pen_x + image.origin.x * factor).round() as i32;
    // `origin.y` is the bottom bearing: signed distance from the
    // baseline to the bitmap's bottom edge (negative = below baseline).
    // The top edge sits `height` strike pixels above it.
    let top = (baseline - (image.origin.y + src_h as f32) * factor).round() as i32;

    for dy in 0..dest_h {
        for dx in 0..dest_w {
            let sx = (dx as f32 / factor) as u32;
            let sy = (dy as f32 / factor) as u32;
            if sx >= src_w || sy >= src_h {
                continue;
            }
            let s = ((sy * src_w + sx) * 4) as usize;
            let px = left + dx;
            let py = top + dy;
            if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                continue;
            }
            let dst = ((py as u32 * width + px as u32) * 4) as usize;
            // premultiplied RGBA on the opaque black background
            pixels[dst] = src[s];
            pixels[dst + 1] = src[s + 1];
            pixels[dst + 2] = src[s + 2];
        }
    }
}

/// Decodes a raster strike into premultiplied RGBA8.
///
/// Returns `(width, height, pixels)`. Supported formats:
/// `BitmapPremulBgra32` (raw CBDT bitmaps) and `Png` (Noto Color Emoji
/// stores PNG blobs in CBDT format 17); other formats return `None`.
fn raster_image_rgba(image: &GlyphImage) -> Option<(u32, u32, Vec<u8>)> {
    match image.format {
        GlyphImageFormat::BitmapPremulBgra32 => {
            let mut rgba = Vec::with_capacity(image.data.len());
            for bgra in image.data.chunks_exact(4) {
                rgba.extend_from_slice(&[bgra[2], bgra[1], bgra[0], bgra[3]]);
            }
            Some((u32::from(image.width), u32::from(image.height), rgba))
        }
        GlyphImageFormat::Png => decode_png_rgba(image.data),
        _ => None,
    }
}

/// Decodes PNG data into premultiplied RGBA8 via gdk-pixbuf.
fn decode_png_rgba(data: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let pixbuf = Pixbuf::from_read(std::io::Cursor::new(data.to_vec())).ok()?;
    let width = pixbuf.width() as u32;
    let height = pixbuf.height() as u32;
    let channels = pixbuf.n_channels() as usize;
    let rowstride = pixbuf.rowstride() as usize;
    let bytes = pixbuf.read_pixel_bytes();
    let raw: &[u8] = bytes.as_ref();

    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let i = y * rowstride + x * channels;
            let a = if channels == 4 { raw[i + 3] } else { 255 };
            // premultiply onto the opaque black background
            rgba.extend_from_slice(&[
                (u16::from(raw[i]) * u16::from(a) / 255) as u8,
                (u16::from(raw[i + 1]) * u16::from(a) / 255) as u8,
                (u16::from(raw[i + 2]) * u16::from(a) / 255) as u8,
                a,
            ]);
        }
    }
    Some((width, height, rgba))
}
