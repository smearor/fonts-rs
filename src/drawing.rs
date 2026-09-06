//! Software rendering utilities for headless (non-GTK) pixel buffer rendering.
//!
//! Provides functions to draw Nerd Font icons, text labels, progress bars,
//! and icon grids onto raw RGBA pixel buffers using `ab_glyph`.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

use crate::fonts::label_font;
use crate::fonts::nerd_font;
use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::Glyph;
use ab_glyph::PxScale;
use ab_glyph::PxScaleFont;
use ab_glyph::ScaleFont;
use tracing::debug;

/// Fill the entire pixel buffer with a solid background color.
pub fn fill_background(pixels: &mut [u8], width: u32, height: u32, color: [u8; 4]) {
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            pixels[idx..idx + 4].copy_from_slice(&color);
        }
    }
}

/// Draw a Nerd Font icon centered on the image, occupying the upper portion.
///
/// The icon name is resolved to a Nerd Font codepoint via the caller-provided
/// resolver function. If the font or icon is unavailable, a circular
/// placeholder is drawn instead.
pub fn draw_nerd_font_icon(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    icon_name: &str,
    is_active: bool,
    resolve_codepoint: impl Fn(&str) -> Option<char>,
    icon_color: Option<[u8; 4]>,
) {
    let font = match nerd_font() {
        Some(f) => f,
        None => {
            debug!("nerd-fonts-gtk: Nerd Font not available, drawing placeholder");
            draw_icon_placeholder(pixels, width, height, is_active);
            return;
        }
    };

    let codepoint = match resolve_codepoint(icon_name) {
        Some(c) => c,
        None => {
            debug!("nerd-fonts-gtk: unknown icon name '{}', drawing placeholder", icon_name);
            draw_icon_placeholder(pixels, width, height, is_active);
            return;
        }
    };

    let color = icon_color.unwrap_or_else(|| default_text_color(is_active));

    let icon_size = width.min(height) as f32 * 0.6;
    let glyph_id = font.glyph_id(codepoint);
    let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
    let scale = PxScale::from(em_scale);

    // Use outline px_bounds for precise centering (glyph_bounds includes side bearings)
    let probe = glyph_id.with_scale(scale);
    if let Some(outline) = font.outline_glyph(probe.clone()) {
        let pb = outline.px_bounds();
        let glyph_w = pb.width();
        let glyph_h = pb.height();
        let pos_x = (width as f32 - glyph_w) / 2.0 - pb.min.x;
        let pos_y = (height as f32 * 0.4) - glyph_h / 2.0 - pb.min.y;
        let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pos_x, pos_y));
        draw_glyph(pixels, width, height, font, &glyph, color);
    } else {
        // Fallback: use glyph_bounds
        let bounds = font.glyph_bounds(&probe);
        let glyph_w = bounds.width();
        let glyph_h = bounds.height();
        let pos_x = (width as f32 - glyph_w) / 2.0 - bounds.min.x;
        let pos_y = (height as f32 * 0.4) - glyph_h / 2.0 - bounds.min.y;
        let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pos_x, pos_y));
        draw_glyph(pixels, width, height, font, &glyph, color);
    }
}

/// Draw a simple circular placeholder when the font or icon is unavailable.
pub fn draw_icon_placeholder(pixels: &mut [u8], width: u32, height: u32, is_active: bool) {
    let center_x = width / 2;
    let center_y = width / 2;
    let radius = (width.min(height) / 3).max(8);
    let color = default_text_color(is_active);

    let cx = center_x as i32;
    let cy = (center_y as i32) - (height as i32 / 6);
    let r = radius as i32;

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= r * r {
                let idx = ((y as u32 * width + x as u32) * 4) as usize;
                pixels[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}

/// Draw a text label at the bottom of the image using the label font.
///
/// Falls back to the Nerd Font if the label font is unavailable, and to
/// a bitmap renderer if no font is available.
pub fn draw_label_text(pixels: &mut [u8], width: u32, height: u32, text: &str, is_active: bool, text_color_override: Option<[u8; 4]>) {
    let font = match label_font() {
        Some(f) => f,
        None => {
            debug!("nerd-fonts-gtk: label font not available, trying Nerd Font");
            match nerd_font() {
                Some(f) => f,
                None => {
                    debug!("nerd-fonts-gtk: no font available for label, using bitmap fallback");
                    draw_text_bitmap(pixels, width, height, text, is_active);
                    return;
                }
            }
        }
    };

    let color = text_color_override.unwrap_or_else(|| default_text_color(is_active));

    let font_size = (height as f32 * 0.22).max(8.0);
    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);

    let max_width = width as f32 * 0.9;
    let truncated = truncate_text_to_width(text, &scaled_font, max_width);
    if truncated.is_empty() {
        return;
    }

    let total_width: f32 = truncated.chars().map(|c| scaled_font.h_advance(font.glyph_id(c))).sum();
    let start_x = ((width as f32 - total_width) / 2.0).max(0.0);
    let baseline = height as f32 * 0.88;

    let mut pen_x = start_x;
    for ch in truncated.chars() {
        let glyph_id = font.glyph_id(ch);
        let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pen_x, baseline));
        draw_glyph(pixels, width, height, font, &glyph, color);
        pen_x += scaled_font.h_advance(glyph_id);
    }
}

/// Draw a Nerd Font icon by its Unicode codepoint at a custom position and size.
pub fn draw_nerd_font_codepoint(pixels: &mut [u8], width: u32, height: u32, codepoint: char, center_x: f32, center_y: f32, icon_size: f32, color: [u8; 4]) {
    let font = match nerd_font() {
        Some(f) => f,
        None => return,
    };

    let glyph_id = font.glyph_id(codepoint);
    let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
    let scale = PxScale::from(em_scale);

    // Center the glyph at (center_x, center_y) using outline px_bounds
    let probe = glyph_id.with_scale(scale);
    if let Some(outline) = font.outline_glyph(probe.clone()) {
        let pb = outline.px_bounds();
        let pos_x = center_x - pb.width() / 2.0 - pb.min.x;
        let pos_y = center_y - pb.height() / 2.0 - pb.min.y;
        let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pos_x, pos_y));
        draw_glyph(pixels, width, height, font, &glyph, color);
    } else {
        let bounds = font.glyph_bounds(&probe);
        let pos_x = center_x - bounds.width() / 2.0 - bounds.min.x;
        let pos_y = center_y - bounds.height() / 2.0 - bounds.min.y;
        let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pos_x, pos_y));
        draw_glyph(pixels, width, height, font, &glyph, color);
    }
}

/// Draw text centered horizontally at a given y-baseline with a configurable font size.
pub fn draw_text_centered(pixels: &mut [u8], width: u32, height: u32, text: &str, baseline_y: f32, font_size: f32, color: [u8; 4]) {
    let font = match label_font().or_else(nerd_font) {
        Some(f) => f,
        None => {
            draw_text_bitmap(pixels, width, height, text, false);
            return;
        }
    };

    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);
    let max_width = width as f32 * 0.9;
    let truncated = truncate_text_to_width(text, &scaled_font, max_width);
    if truncated.is_empty() {
        return;
    }

    let total_width: f32 = truncated.chars().map(|c| scaled_font.h_advance(font.glyph_id(c))).sum();
    let start_x = ((width as f32 - total_width) / 2.0).max(0.0);

    let mut pen_x = start_x;
    for ch in truncated.chars() {
        let glyph_id = font.glyph_id(ch);
        let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pen_x, baseline_y));
        draw_glyph(pixels, width, height, font, &glyph, color);
        pen_x += scaled_font.h_advance(glyph_id);
    }
}

/// Draw a horizontal progress bar at the bottom of the image.
pub fn draw_progress_bar(pixels: &mut [u8], width: u32, height: u32, value: f32, color: [u8; 4]) {
    let bar_height = 4u32;
    let bar_y = height.saturating_sub(bar_height + 2);
    let fill_width = ((width as f32 * value.clamp(0.0, 1.0)) as u32).min(width);

    for y in bar_y..bar_y + bar_height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if x < fill_width {
                pixels[idx..idx + 4].copy_from_slice(&color);
            } else {
                pixels[idx..idx + 4].copy_from_slice(&[color[0] / 3, color[1] / 3, color[2] / 3, 255]);
            }
        }
    }
}

/// Draw a 2x2 or 3x3 icon grid within the image area.
pub fn draw_icon_grid(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    icons: &[&str],
    grid_cols: u32,
    is_active: bool,
    resolve_codepoint: impl Fn(&str) -> Option<char>,
) {
    let font = match nerd_font() {
        Some(f) => f,
        None => return,
    };

    let rows = (icons.len() as u32).div_ceil(grid_cols).max(1);
    let cell_w = width / grid_cols;
    let cell_h = height / rows;
    let icon_size = cell_w.min(cell_h) as f32 * 0.6;
    let color = default_text_color(is_active);

    for (i, icon_name) in icons.iter().enumerate() {
        let codepoint = match resolve_codepoint(icon_name) {
            Some(c) => c,
            None => continue,
        };

        let col = (i as u32) % grid_cols;
        let row = (i as u32) / grid_cols;
        let cx = col * cell_w + cell_w / 2;
        let cy = row * cell_h + cell_h / 2;

        let glyph_id = font.glyph_id(codepoint);
        let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
        let scale = PxScale::from(em_scale);

        // Center the glyph in its cell using outline px_bounds
        let probe = glyph_id.with_scale(scale);
        if let Some(outline) = font.outline_glyph(probe.clone()) {
            let pb = outline.px_bounds();
            let pos_x = cx as f32 - pb.width() / 2.0 - pb.min.x;
            let pos_y = cy as f32 - pb.height() / 2.0 - pb.min.y;
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pos_x, pos_y));
            draw_glyph(pixels, width, height, font, &glyph, color);
        } else {
            let bounds = font.glyph_bounds(&probe);
            let pos_x = cx as f32 - bounds.width() / 2.0 - bounds.min.x;
            let pos_y = cy as f32 - bounds.height() / 2.0 - bounds.min.y;
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(pos_x, pos_y));
            draw_glyph(pixels, width, height, font, &glyph, color);
        }
    }
}

/// Default text color based on active state.
fn default_text_color(is_active: bool) -> [u8; 4] {
    if is_active { [100, 180, 255, 255] } else { [240, 240, 240, 255] }
}

/// Compute the em-scale needed so that a glyph's largest dimension (width or
/// height) equals `target_px` pixels.
///
/// `PxScale::from(x)` sets the **em** size, not the glyph size. Nerd Font
/// symbols typically occupy only a small fraction of the em square, so a
/// 76px em can produce a ~12px glyph. This function probes the glyph at 1px
/// em, measures its design-space bounds, and returns the em-scale that makes
/// the glyph's largest dimension match `target_px`.
fn glyph_fit_scale(font: &FontVec, glyph_id: ab_glyph::GlyphId, target_px: f32) -> f32 {
    // Probe at a large enough scale to get accurate pixel measurements.
    // outline_glyph rounds to integer pixels, so scale=1.0 gives 0 or 1 px
    // which is too imprecise. Use 100.0 em and scale the result.
    let probe_scale = 100.0_f32;
    let probe = glyph_id.with_scale(probe_scale);
    if let Some(outline) = font.outline_glyph(probe) {
        let pb = outline.px_bounds();
        let gw = pb.width().max(0.0);
        let gh = pb.height().max(0.0);
        let max_g = gw.max(gh);
        if max_g > 0.01 {
            // max_g is the pixel size at probe_scale em.
            // To get target_px pixels: em_scale = target_px * probe_scale / max_g
            return target_px * probe_scale / max_g;
        }
    }
    // Fallback: use glyph_bounds (design-space, not pixel-accurate)
    let probe = glyph_id.with_scale(1.0);
    let bounds = font.glyph_bounds(&probe);
    let gw = bounds.width().max(0.0);
    let gh = bounds.height().max(0.0);
    let max_g = gw.max(gh);
    if max_g > 0.01 { target_px / max_g } else { target_px }
}

/// Truncate text to fit within a maximum pixel width.
fn truncate_text_to_width(text: &str, scaled_font: &PxScaleFont<&FontVec>, max_width: f32) -> String {
    let mut result = String::new();
    for ch in text.chars() {
        let glyph_id = scaled_font.glyph_id(ch);
        let char_width = scaled_font.h_advance(glyph_id);
        let current_width: f32 = result.chars().map(|c| scaled_font.h_advance(scaled_font.glyph_id(c))).sum();
        if current_width + char_width > max_width {
            break;
        }
        result.push(ch);
    }
    result
}

/// Draw a single glyph onto the pixel buffer with alpha blending.
fn draw_glyph(pixels: &mut [u8], width: u32, height: u32, font: &FontVec, glyph: &Glyph, color: [u8; 4]) {
    if let Some(outline) = font.outline_glyph(glyph.clone()) {
        let bounds = outline.px_bounds();
        let px = bounds.min.x.round() as i32;
        let py = bounds.min.y.round() as i32;

        outline.draw(|gx, gy, coverage| {
            let x = px + gx as i32;
            let y = py + gy as i32;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let idx = ((y as u32 * width + x as u32) * 4) as usize;
                let alpha = (coverage * color[3] as f32) as u8;
                if alpha > 0 {
                    let blended = blend_pixel(&pixels[idx..idx + 4], &color, alpha);
                    pixels[idx..idx + 4].copy_from_slice(&blended);
                }
            }
        });
    }
}

/// Alpha-blend a source color over an existing pixel.
fn blend_pixel(dst: &[u8], src: &[u8; 4], alpha: u8) -> [u8; 4] {
    let src_alpha = alpha as f32 / 255.0;
    let dst_alpha = dst[3] as f32 / 255.0;
    let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);
    if out_alpha < 0.001 {
        return [0, 0, 0, 0];
    }
    let r = (src[0] as f32 * src_alpha + dst[0] as f32 * dst_alpha * (1.0 - src_alpha)) / out_alpha;
    let g = (src[1] as f32 * src_alpha + dst[1] as f32 * dst_alpha * (1.0 - src_alpha)) / out_alpha;
    let b = (src[2] as f32 * src_alpha + dst[2] as f32 * dst_alpha * (1.0 - src_alpha)) / out_alpha;
    [r.round() as u8, g.round() as u8, b.round() as u8, (out_alpha * 255.0).round() as u8]
}

/// Simple bitmap text renderer used when no font is available.
fn draw_text_bitmap(pixels: &mut [u8], width: u32, height: u32, text: &str, _is_active: bool) {
    let color = default_text_color(false);
    let char_w = 6u32;
    let char_h = 8u32;
    let total_w = text.len() as u32 * char_w;
    let start_x = width.saturating_sub(total_w) / 2;
    let start_y = height.saturating_sub(char_h) / 2;

    for (i, _ch) in text.chars().enumerate() {
        let x = start_x + i as u32 * char_w;
        for dy in 0..char_h {
            for dx in 0..char_w {
                if x + dx < width && start_y + dy < height {
                    let idx = (((start_y + dy) * width + x + dx) * 4) as usize;
                    pixels[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }
}
