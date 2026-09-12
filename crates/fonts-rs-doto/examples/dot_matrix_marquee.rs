//! Dot Matrix Marquee — a scrolling LED-style display using Doto glyphs.
//!
//! Type text, watch it scroll across the dot-matrix display like an old
//! airport departure board. The display glows, the dots pulse, and you
//! can swap between all 9 variants of the 3x3 weight×roundness matrix
//! to see how the font changes character.
//!
//! Run with:
//! ```sh
//! cargo run --example dot_matrix_marquee --features gtk,embed-fonts
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use fonts_rs_doto::register_glyphs;
use fonts_rs_doto::variant;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::ColorDialogButton;
use gtk4::Entry;
use gtk4::Image;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Overflow;
use gtk4::glib;
use gtk4::prelude::*;

use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.fonts_rs.doto_marquee";

/// Default spacing between glyph slots in the marquee (px).
const DEFAULT_GLYPH_SPACING: i32 = 2;

/// CSS padding of the marquee background (px, each side).
const MARQUEE_PADDING: i32 = 24;

/// Program start time for debug timestamps.
static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

fn ts() -> String {
    let start = START.get_or_init(Instant::now);
    format!("{:.3}s", start.elapsed().as_secs_f64())
}

/// All 25 variants in the 5x5 matrix: (feature_name, label, wght, rond).
const VARIANTS: &[(&str, &str, f32, f32)] = &[
    ("ultra-light-square", "Ultra-Light Square", 100.0, 0.0),
    ("ultra-light-soft-square", "Ultra-Light Soft Square", 100.0, 25.0),
    ("ultra-light-medium", "Ultra-Light Medium", 100.0, 50.0),
    ("ultra-light-soft-dot", "Ultra-Light Soft Dot", 100.0, 75.0),
    ("ultra-light-dot", "Ultra-Light Dot", 100.0, 100.0),
    ("light-square", "Light Square", 300.0, 0.0),
    ("light-soft-square", "Light Soft Square", 300.0, 25.0),
    ("light-medium", "Light Medium", 300.0, 50.0),
    ("light-soft-dot", "Light Soft Dot", 300.0, 75.0),
    ("light-dot", "Light Dot", 300.0, 100.0),
    ("regular-square", "Regular Square", 500.0, 0.0),
    ("regular-soft-square", "Regular Soft Square", 500.0, 25.0),
    ("regular-medium", "Regular Medium", 500.0, 50.0),
    ("regular-soft-dot", "Regular Soft Dot", 500.0, 75.0),
    ("regular-dot", "Regular Dot", 500.0, 100.0),
    ("bold-square", "Bold Square", 700.0, 0.0),
    ("bold-soft-square", "Bold Soft Square", 700.0, 25.0),
    ("bold-medium", "Bold Medium", 700.0, 50.0),
    ("bold-soft-dot", "Bold Soft Dot", 700.0, 75.0),
    ("bold-dot", "Bold Dot", 700.0, 100.0),
    ("extra-bold-square", "Extra-Bold Square", 900.0, 0.0),
    ("extra-bold-soft-square", "Extra-Bold Soft Square", 900.0, 25.0),
    ("extra-bold-medium", "Extra-Bold Medium", 900.0, 50.0),
    ("extra-bold-soft-dot", "Extra-Bold Soft Dot", 900.0, 75.0),
    ("extra-bold-dot", "Extra-Bold Dot", 900.0, 100.0),
];

/// Map an ASCII character to its Doto glyph base name.
fn char_to_glyph_base(ch: char) -> Option<&'static str> {
    let names = [
        (' ', "space"),
        ('!', "exclam"),
        ('"', "quotedbl"),
        ('#', "numbersign"),
        ('$', "dollar"),
        ('%', "percent"),
        ('&', "ampersand"),
        ('\'', "quotesingle"),
        ('(', "parenleft"),
        (')', "parenright"),
        ('*', "asterisk"),
        ('+', "plus"),
        (',', "comma"),
        ('-', "hyphen"),
        ('.', "period"),
        ('/', "slash"),
        ('0', "zero"),
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
        (':', "colon"),
        (';', "semicolon"),
        ('<', "less"),
        ('=', "equal"),
        ('>', "greater"),
        ('?', "question"),
        ('@', "at"),
        ('A', "a"),
        ('B', "b"),
        ('C', "c"),
        ('D', "d"),
        ('E', "e"),
        ('F', "f"),
        ('G', "g"),
        ('H', "h"),
        ('I', "i"),
        ('J', "j"),
        ('K', "k"),
        ('L', "l"),
        ('M', "m"),
        ('N', "n"),
        ('O', "o"),
        ('P', "p"),
        ('Q', "q"),
        ('R', "r"),
        ('S', "s"),
        ('T', "t"),
        ('U', "u"),
        ('V', "v"),
        ('W', "w"),
        ('X', "x"),
        ('Y', "y"),
        ('Z', "z"),
        ('[', "bracketleft"),
        ('\\', "backslash"),
        (']', "bracketright"),
        ('^', "asciicircum"),
        ('`', "grave"),
        ('{', "braceleft"),
        ('|', "bar"),
        ('}', "braceright"),
        ('~', "asciitilde"),
    ];
    names.iter().find(|(c, _)| *c == ch).map(|(_, name)| *name)
}

/// Convert a `gdk::RGBA` to a `#rrggbb` hex string (alpha ignored for glyph coloring).
fn rgba_to_hex(rgba: &gtk4::gdk::RGBA) -> String {
    format!("#{:02x}{:02x}{:02x}", (rgba.red() * 255.0) as u8, (rgba.green() * 255.0) as u8, (rgba.blue() * 255.0) as u8,)
}

/// Update the dynamic CSS provider with current colors and transparency.
fn update_dynamic_css(state: &Rc<RefCell<MarqueeState>>) {
    let s = state.borrow();
    let bg = &s.bg_color;
    let alpha = if s.fullscreen { s.transparency } else { 1.0 };
    let bg_css = format!(
        "rgba({}, {}, {}, {:.3})",
        (bg.red() * 255.0) as u8,
        (bg.green() * 255.0) as u8,
        (bg.blue() * 255.0) as u8,
        alpha,
    );
    let window_bg = if s.fullscreen {
        "transparent".to_string()
    } else {
        format!("rgb({}, {}, {})", (bg.red() * 255.0) as u8, (bg.green() * 255.0) as u8, (bg.blue() * 255.0) as u8,)
    };
    let css = format!(
        "window {{ background-color: {window_bg}; }}\n\
         .marquee-bg {{ background-color: {bg_css}; }}\n"
    );
    s.css_provider.load_from_string(&css);
    // Tell the compositor the window surface is not opaque in fullscreen,
    // so the transparent CSS background actually shows through to the desktop.
    if s.fullscreen {
        if let Some(surface) = s.window.surface() {
            surface.set_opaque_region(None);
        }
    }
}

/// Load an SVG from a GResource, replace `currentColor` with the given hex color,
/// and return a `gdk::Texture`.
///
/// `Image::from_resource` renders SVGs without CSS context, so `currentColor`
/// always resolves to black. This workaround recolors the SVG bytes before
/// creating the texture.
fn colorized_texture(resource_path: &str, hex_color: &str) -> Option<gtk4::gdk::Texture> {
    let bytes = gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE).ok()?;
    let svg = std::str::from_utf8(bytes.as_ref()).ok()?;
    let colored = svg.replace("currentColor", hex_color);
    let bytes = glib::Bytes::from(colored.as_bytes());
    gtk4::gdk::Texture::from_bytes(&bytes).ok()
}

/// State for the scrolling marquee.
struct MarqueeState {
    text: String,
    offset: usize,
    images: Vec<Image>,
    marquee: Box,
    spacer_right: Box,
    pixel_size: i32,
    glyph_spacing: i32,
    scroll_interval_ms: u64,
    last_width: i32,
    win_width: i32,
    stable_count: u32,
    scrolling: bool,
    fullscreen: bool,
    saved_width: i32,
    saved_slot_count: usize,
    unfullscreening: bool,
    fg_color: gtk4::gdk::RGBA,
    glow_color: gtk4::gdk::RGBA,
    dim_color: gtk4::gdk::RGBA,
    bg_color: gtk4::gdk::RGBA,
    transparency: f64,
    css_provider: gtk4::CssProvider,
    window: ApplicationWindow,
}

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    Ok(app.run())
}

fn build_ui(app: &Application) {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(
        r#"
        window {
            background-color: transparent;
        }
        .marquee-bg {
            border-radius: 12px;
            padding: 24px;
        }
        .marquee-dot {
            color: #ff6600;
        }
        .marquee-dot-glow {
            color: #ffaa00;
            filter: brightness(1.3);
        }
        .marquee-dot-dim {
            color: #331a00;
        }
        .control-panel {
            background-color: #1a1a1a;
            border-radius: 8px;
            padding: 12px;
        }
        .variant-label {
            color: #ff9933;
            font-size: 11pt;
            font-weight: bold;
        }
        .axis-scale trough {
            background-color: #333;
            min-height: 6px;
        }
        .axis-scale highlight {
            background-color: #ff6600;
            min-height: 6px;
        }
        .axis-scale slider {
            background-color: #ff6600;
            min-height: 16px;
            min-width: 16px;
        }
        .hint {
            color: #888;
        }
        "#,
    );
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Doto Dot-Matrix Marquee")
        .default_width(800)
        .build();

    window.connect_close_request({
        let app = app.clone();
        move |_| {
            app.quit();
            glib::Propagation::Proceed
        }
    });

    // Lock height: window should only be resizable horizontally
    // The height is determined by the marquee + controls, not by the user

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();
    main_box.set_size_request(0, -1);

    // --- Marquee display ---
    let marquee = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(DEFAULT_GLYPH_SPACING)
        .halign(Align::Fill)
        .hexpand(true)
        .css_classes(["marquee-bg"])
        .build();
    // Allow shrinking below content's natural minimum width.
    // The polling loop will remove slots when the width decreases.
    marquee.set_size_request(0, -1);
    marquee.set_overflow(Overflow::Hidden);

    // Spacer widgets on each side to center the slots.
    // They expand to fill leftover space equally, and shrink to zero when window is small.
    let spacer_left = Box::builder().hexpand(true).halign(Align::Fill).build();
    let spacer_right = Box::builder().hexpand(true).halign(Align::Fill).build();

    marquee.append(&spacer_left);
    marquee.append(&spacer_right);

    main_box.append(&marquee);

    // --- Control panel ---
    let controls = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .css_classes(["control-panel"])
        .build();

    let input_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let entry = Entry::builder().placeholder_text("Type something...").text("HELLO DOTO").hexpand(true).build();
    input_row.append(&entry);

    let scroll_button = Button::with_label("⏸ Pause");
    input_row.append(&scroll_button);

    controls.append(&input_row);

    // --- Size slider ---
    let size_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let size_label = Label::builder().label("Size:").css_classes(["variant-label"]).build();
    size_row.append(&size_label);

    let size_scale = gtk4::Scale::with_range(Orientation::Horizontal, 16.0, 512.0, 16.0);
    size_scale.set_value(96.0);
    size_scale.set_draw_value(false);
    size_scale.set_hexpand(true);
    for mark in [16, 32, 64, 96, 128, 160, 192, 256, 384, 512] {
        size_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    size_row.append(&size_scale);
    controls.append(&size_row);

    // --- Speed slider ---
    let speed_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let speed_label = Label::builder().label("Speed:").css_classes(["variant-label"]).build();
    speed_row.append(&speed_label);

    let speed_scale = gtk4::Scale::with_range(Orientation::Horizontal, 50.0, 1000.0, 50.0);
    speed_scale.set_value(400.0);
    speed_scale.set_draw_value(false);
    speed_scale.set_hexpand(true);
    speed_scale.set_inverted(true);
    for mark in [50, 200, 400, 600, 800, 1000] {
        let label = if mark <= 200 {
            "fast"
        } else if mark <= 600 {
            "medium"
        } else {
            "slow"
        };
        speed_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(label));
    }
    speed_row.append(&speed_scale);
    controls.append(&speed_row);

    // --- Spacing slider ---
    let spacing_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let spacing_label = Label::builder().label("Spacing:").css_classes(["variant-label"]).build();
    spacing_row.append(&spacing_label);

    let spacing_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 32.0, 1.0);
    spacing_scale.set_value(DEFAULT_GLYPH_SPACING as f64);
    spacing_scale.set_draw_value(false);
    spacing_scale.set_hexpand(true);
    for mark in [0, 2, 8, 16, 24, 32] {
        spacing_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    spacing_row.append(&spacing_scale);
    controls.append(&spacing_row);

    // --- Weight slider (read-only) ---
    let active_idx = VARIANTS
        .iter()
        .position(|(feat, _, _, _)| variant::GLYPH_PREFIX == format!("doto-{feat}"))
        .unwrap_or(12);
    let (_active_feat, _active_name, active_wght, active_rond) = VARIANTS[active_idx];

    let weight_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let weight_label = Label::builder().label("Weight:").css_classes(["variant-label"]).build();
    weight_row.append(&weight_label);

    let weight_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 1000.0, 100.0);
    weight_scale.set_value(active_wght as f64);
    weight_scale.set_draw_value(false);
    weight_scale.set_hexpand(true);
    weight_scale.set_sensitive(false);
    weight_scale.add_css_class("axis-scale");
    for mark in (0..=1000).step_by(100) {
        let label = if mark as f32 == active_wght {
            format!("<b><span foreground='#ff6600'>{mark}</span></b>")
        } else {
            mark.to_string()
        };
        weight_scale.add_mark(mark as f64, gtk4::PositionType::Top, Some(&label));
    }
    for wght in [100.0_f32, 300.0, 500.0, 700.0, 900.0] {
        let part = match wght as i32 {
            100 => "Ultra-Light",
            300 => "Light",
            500 => "Regular",
            700 => "Bold",
            900 => "Extra-Bold",
            _ => "",
        };
        let label = if wght == active_wght {
            format!("<b><span foreground='#ff6600'>{part}</span></b>")
        } else {
            part.to_string()
        };
        weight_scale.add_mark(wght as f64, gtk4::PositionType::Bottom, Some(&label));
    }
    weight_row.append(&weight_scale);
    controls.append(&weight_row);

    // --- Rond slider (read-only) ---
    let rond_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let rond_label = Label::builder().label("Rond:").css_classes(["variant-label"]).build();
    rond_row.append(&rond_label);

    let rond_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 10.0);
    rond_scale.set_value(active_rond as f64);
    rond_scale.set_draw_value(false);
    rond_scale.set_hexpand(true);
    rond_scale.set_sensitive(false);
    rond_scale.add_css_class("axis-scale");
    for mark in (0..=100).step_by(25) {
        let label = if mark as f32 == active_rond {
            format!("<b><span foreground='#ff6600'>{mark}%</span></b>")
        } else {
            format!("{mark}%")
        };
        rond_scale.add_mark(mark as f64, gtk4::PositionType::Top, Some(&label));
    }
    for rond in [0.0_f32, 25.0, 50.0, 75.0, 100.0] {
        let part = match rond as i32 {
            0 => "Square",
            25 => "Soft Square",
            50 => "Medium",
            75 => "Soft Dot",
            100 => "Dot",
            _ => "",
        };
        let label = if rond == active_rond {
            format!("<b><span foreground='#ff6600'>{part}</span></b>")
        } else {
            part.to_string()
        };
        rond_scale.add_mark(rond as f64, gtk4::PositionType::Bottom, Some(&label));
    }
    rond_row.append(&rond_scale);
    controls.append(&rond_row);

    // --- Color pickers + transparency slider (one row) ---
    let color_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let color_dialog = gtk4::ColorDialog::new();

    let fg_btn = ColorDialogButton::builder().tooltip_text("Vordergrund (Glyphen)").build();
    fg_btn.set_dialog(&color_dialog);
    fg_btn.set_rgba(&gtk4::gdk::RGBA::new(1.0, 0.4, 0.0, 1.0));
    color_row.append(&fg_btn);

    let glow_btn = ColorDialogButton::builder().tooltip_text("Glüh-Effekt").build();
    glow_btn.set_dialog(&color_dialog);
    glow_btn.set_rgba(&gtk4::gdk::RGBA::new(1.0, 0.67, 0.0, 1.0));
    color_row.append(&glow_btn);

    let dim_btn = ColorDialogButton::builder().tooltip_text("Dim (inaktive Glyphen)").build();
    dim_btn.set_dialog(&color_dialog);
    dim_btn.set_rgba(&gtk4::gdk::RGBA::new(0.2, 0.1, 0.0, 1.0));
    color_row.append(&dim_btn);

    let bg_btn = ColorDialogButton::builder().tooltip_text("Hintergrund").build();
    bg_btn.set_dialog(&color_dialog);
    bg_btn.set_rgba(&gtk4::gdk::RGBA::new(0.04, 0.04, 0.04, 1.0));
    color_row.append(&bg_btn);

    let opacity_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 5.0);
    opacity_scale.set_value(100.0);
    opacity_scale.set_draw_value(false);
    opacity_scale.set_hexpand(true);
    opacity_scale.set_tooltip_text(Some("Transparenz (nur Fullscreen)"));
    opacity_scale.add_mark(0.0, gtk4::PositionType::Bottom, Some("0%"));
    opacity_scale.add_mark(50.0, gtk4::PositionType::Bottom, Some("50%"));
    opacity_scale.add_mark(100.0, gtk4::PositionType::Bottom, Some("100%"));
    color_row.append(&opacity_scale);

    controls.append(&color_row);

    // Wrap controls in a ScrolledWindow so they don't block window shrinking.
    // The controls scroll horizontally if the window is narrower than their natural width.
    let controls_scroll = gtk4::ScrolledWindow::builder()
        .child(&controls)
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Never)
        .build();
    controls_scroll.set_propagate_natural_height(true);
    controls_scroll.set_size_request(0, -1);

    main_box.append(&controls_scroll);

    // Prevent vertical expansion so the window height matches content
    controls.set_vexpand(false);
    controls_scroll.set_vexpand(false);
    main_box.set_vexpand(false);

    window.set_child(Some(&main_box));

    window.present();

    // --- Marquee logic ---
    let dynamic_provider = gtk4::CssProvider::new();
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &dynamic_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    let state = Rc::new(RefCell::new(MarqueeState {
        text: "HELLO DOTO".to_string(),
        offset: 0,
        images: Vec::new(),
        marquee: marquee.clone(),
        spacer_right: spacer_right.clone(),
        pixel_size: 96,
        glyph_spacing: DEFAULT_GLYPH_SPACING,
        scroll_interval_ms: 400,
        last_width: 0,
        win_width: 0,
        stable_count: 0,
        scrolling: true,
        fullscreen: false,
        saved_width: 800,
        saved_slot_count: 0,
        unfullscreening: false,
        fg_color: gtk4::gdk::RGBA::new(1.0, 0.4, 0.0, 1.0),
        glow_color: gtk4::gdk::RGBA::new(1.0, 0.67, 0.0, 1.0),
        dim_color: gtk4::gdk::RGBA::new(0.2, 0.1, 0.0, 1.0),
        bg_color: gtk4::gdk::RGBA::new(0.04, 0.04, 0.04, 1.0),
        transparency: 1.0,
        css_provider: dynamic_provider,
        window: window.clone(),
    }));

    update_dynamic_css(&state);

    // Fullscreen toggle via F5 / ESC
    {
        let state = state.clone();
        let window = window.clone();
        let controls = controls_scroll.clone();
        let marquee = marquee.clone();
        let window_for_controller = window.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            let mut s = state.borrow_mut();
            match keyval {
                gtk4::gdk::Key::F5 => {
                    if s.fullscreen {
                        s.fullscreen = false;
                        s.unfullscreening = true;
                        s.last_width = 0;
                        s.stable_count = 0;
                        let saved = s.saved_width;
                        let target_slots = s.saved_slot_count;
                        drop(s);
                        // Restore slot count immediately so content doesn't
                        // force the window to stay fullscreen-wide
                        {
                            let mut s = state.borrow_mut();
                            let current = s.images.len();
                            if target_slots < current {
                                for _ in target_slots..current {
                                    if let Some(img) = s.images.pop() {
                                        s.marquee.remove(&img);
                                    }
                                }
                            }
                        }
                        update_marquee(&state);
                        eprintln!(
                            "[{}] [FS-EXIT] saved_width={}, target_slots={}, current_slots_after_trim={}, window.width={}",
                            ts(),
                            saved,
                            target_slots,
                            state.borrow().images.len(),
                            window.width()
                        );
                        // 1. Temp constraints zurücksetzen
                        marquee.set_hexpand(false);
                        window.set_size_request(-1, -1);
                        // 2. Unmaximize mit Zielgröße
                        window.set_default_size(saved, -1);
                        window.set_decorated(true);
                        window.unmaximize();
                        // 3. WM zwingen auf default_size zu schrumpfen (Fixed-Size Trick)
                        window.set_resizable(false);
                        controls.set_visible(true);
                        marquee.set_margin_top(16);
                        marquee.set_margin_bottom(16);
                        marquee.set_margin_start(16);
                        marquee.set_margin_end(16);
                        marquee.set_valign(gtk4::Align::Fill);
                        marquee.set_vexpand(false);
                        update_dynamic_css(&state);
                        eprintln!("[{}] [FS-EXIT-AFTER] window.width={}, default_width={}", ts(), window.width(), window.default_width());
                    } else {
                        // Don't overwrite saved_width if still unfullscreening from a previous exit
                        if !s.unfullscreening {
                            s.saved_width = window.width();
                            s.saved_slot_count = s.images.len();
                        }
                        s.fullscreen = true;
                        s.unfullscreening = false;
                        eprintln!(
                            "[{}] [FS-ENTER] saved_width={}, saved_slots={}, window.width={}, unfullscreening={}",
                            ts(),
                            s.saved_width,
                            s.saved_slot_count,
                            window.width(),
                            s.unfullscreening
                        );
                        drop(s);
                        marquee.set_hexpand(true);
                        window.set_decorated(false);
                        window.maximize();
                        controls.set_visible(false);
                        marquee.set_margin_top(0);
                        marquee.set_margin_bottom(0);
                        marquee.set_margin_start(0);
                        marquee.set_margin_end(0);
                        marquee.set_valign(gtk4::Align::Center);
                        marquee.set_vexpand(true);
                        update_dynamic_css(&state);
                    }
                    glib::Propagation::Stop
                }
                gtk4::gdk::Key::Escape => {
                    if s.fullscreen {
                        s.fullscreen = false;
                        s.unfullscreening = true;
                        s.last_width = 0;
                        s.stable_count = 0;
                        let saved = s.saved_width;
                        let target_slots = s.saved_slot_count;
                        drop(s);
                        // Restore slot count immediately so content doesn't
                        // force the window to stay fullscreen-wide
                        {
                            let mut s = state.borrow_mut();
                            let current = s.images.len();
                            if target_slots < current {
                                for _ in target_slots..current {
                                    if let Some(img) = s.images.pop() {
                                        s.marquee.remove(&img);
                                    }
                                }
                            }
                        }
                        update_marquee(&state);
                        eprintln!(
                            "[{}] [ESC-EXIT] saved_width={}, target_slots={}, current_slots_after_trim={}, window.width={}",
                            ts(),
                            saved,
                            target_slots,
                            state.borrow().images.len(),
                            window.width()
                        );
                        // 1. Temp constraints zurücksetzen
                        marquee.set_hexpand(false);
                        window.set_size_request(-1, -1);
                        // 2. Unmaximize mit Zielgröße
                        window.set_default_size(saved, -1);
                        window.set_decorated(true);
                        window.unmaximize();
                        // 3. WM zwingen auf default_size zu schrumpfen (Fixed-Size Trick)
                        window.set_resizable(false);
                        controls.set_visible(true);
                        marquee.set_margin_top(16);
                        marquee.set_margin_bottom(16);
                        marquee.set_margin_start(16);
                        marquee.set_margin_end(16);
                        marquee.set_valign(gtk4::Align::Fill);
                        marquee.set_vexpand(false);
                        update_dynamic_css(&state);
                        eprintln!("[{}] [ESC-EXIT-AFTER] window.width={}, default_width={}", ts(), window.width(), window.default_width());
                    }
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
        window_for_controller.add_controller(key_controller);
    }

    // Initial slot count + render
    adjust_marquee_slots(&state);
    update_marquee(&state);

    // Poll marquee width to add/remove slots dynamically (debounced)
    {
        let state = state.clone();
        let marquee = marquee.clone();
        let window = window.clone();
        glib::source::timeout_add_local(std::time::Duration::from_millis(5), move || {
            let width = window.width();
            if width > 0 {
                let mut s = state.borrow_mut();
                if width != s.last_width {
                    s.last_width = width;
                    s.stable_count = 0;
                } else {
                    s.stable_count += 1;
                }
                let stable = s.stable_count;
                s.win_width = width;

                // Log every tick while unfullscreening
                if s.unfullscreening {
                    eprintln!(
                        "[{}] [POLL-UNFS] width={}, saved_width={}, slots={}, saved_slots={}, stable={}",
                        ts(),
                        width,
                        s.saved_width,
                        s.images.len(),
                        s.saved_slot_count,
                        stable
                    );
                }
                // Clear unfullscreening guard once window has shrunk to <= saved_width
                // and been stable for a few ticks
                if s.unfullscreening && width <= s.saved_width && stable >= 3 {
                    eprintln!(
                        "[{}] [POLL] unfullscreening cleared: width={} <= saved_width={}, stable={}",
                        ts(),
                        width,
                        s.saved_width,
                        stable
                    );
                    s.unfullscreening = false;
                    // Restore hexpand and re-enable resizing now that window has shrunk
                    eprintln!("[{}] [POLL] restoring hexpand=true, set_resizable(true)", ts());
                    marquee.set_hexpand(true);
                    window.set_resizable(true);
                }
                let unfullscreening = s.unfullscreening;
                let saved_slot_count = s.saved_slot_count;
                drop(s);
                let s = state.borrow();
                let pixel_size = s.pixel_size;
                let slot_width = pixel_size + s.glyph_spacing;
                let available = if width > 2 * MARQUEE_PADDING {
                    width - 2 * MARQUEE_PADDING
                } else {
                    pixel_size
                };
                // Floor division: never add more slots than fit.
                // Remaining space is split by the two spacers (left/right).
                let needed = ((((available + s.glyph_spacing) / slot_width).max(1) - 1).max(1)) as usize;
                let current = s.images.len();
                drop(s);
                if unfullscreening {
                    // Trim excess slots to saved_slot_count, never grow
                    if current > saved_slot_count {
                        eprintln!("[{}] [POLL-UNFS] trim to saved_slot_count={}: current={}", ts(), saved_slot_count, current);
                        let mut s = state.borrow_mut();
                        for _ in saved_slot_count..current {
                            if let Some(img) = s.images.pop() {
                                s.marquee.remove(&img);
                            }
                        }
                        drop(s);
                    }
                } else if needed < current {
                    eprintln!("[{}] [POLL] shrink: needed={}, current={}, width={}, stable={}", ts(), needed, current, width, stable);
                    adjust_marquee_slots(&state);
                } else if needed > current && stable >= 2 {
                    eprintln!("[{}] [POLL] grow: needed={}, current={}, width={}, stable={}", ts(), needed, current, width, stable);
                    adjust_marquee_slots(&state);
                    update_marquee(&state);
                }
            }
            glib::ControlFlow::Continue
        });
    }

    // Scroll/Pause button: toggle auto-scrolling
    {
        let state = state.clone();
        scroll_button.connect_clicked(glib::clone!(
            #[weak]
            scroll_button,
            move |_| {
                let mut s = state.borrow_mut();
                s.scrolling = !s.scrolling;
                let scrolling = s.scrolling;
                drop(s);
                scroll_button.set_label(if scrolling { "⏸ Pause" } else { "▶ Scroll" });
            }
        ));
    }

    // Entry: update text and reset offset
    {
        let state = state.clone();
        entry.connect_changed(move |e| {
            let mut s = state.borrow_mut();
            s.text = e.text().to_uppercase();
            s.offset = 0;
            drop(s);
            update_marquee(&state);
        });
    }

    // Auto-scroll with a recursive timer (so interval can change dynamically)
    {
        let state = state.clone();
        fn schedule_next(state: Rc<RefCell<MarqueeState>>) {
            let interval = state.borrow().scroll_interval_ms;
            glib::source::timeout_add_local(std::time::Duration::from_millis(interval), move || {
                let mut s = state.borrow_mut();
                if s.scrolling {
                    s.offset = s.offset.wrapping_add(1);
                }
                drop(s);
                update_marquee(&state);
                let next = state.clone();
                schedule_next(next);
                glib::ControlFlow::Break
            });
        }
        schedule_next(state);
    }

    // Speed slider: update scroll interval
    {
        let state = state.clone();
        speed_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.scroll_interval_ms = scale.value() as u64;
        });
    }

    // Spacing slider: update glyph spacing and re-check slot count
    {
        let state = state.clone();
        let marquee = marquee.clone();
        spacing_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.glyph_spacing = scale.value() as i32;
            let spacing = s.glyph_spacing;
            drop(s);
            marquee.set_spacing(spacing);
            adjust_marquee_slots(&state);
            update_marquee(&state);
        });
    }

    // Size slider: update pixel size and re-check slot count
    {
        let state = state.clone();
        size_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.pixel_size = scale.value() as i32;
            drop(s);
            adjust_marquee_slots(&state);
            update_marquee(&state);
        });
    }

    // Color picker: foreground
    {
        let state = state.clone();
        fg_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.fg_color = btn.rgba();
            drop(s);
            update_marquee(&state);
        });
    }

    // Color picker: glow
    {
        let state = state.clone();
        glow_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.glow_color = btn.rgba();
            drop(s);
            update_marquee(&state);
        });
    }

    // Color picker: dim
    {
        let state = state.clone();
        dim_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.dim_color = btn.rgba();
            drop(s);
            update_marquee(&state);
        });
    }

    // Color picker: background
    {
        let state = state.clone();
        bg_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.bg_color = btn.rgba();
            drop(s);
            update_dynamic_css(&state);
        });
    }

    // Transparency slider
    {
        let state = state.clone();
        opacity_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.transparency = scale.value() / 100.0;
            drop(s);
            update_dynamic_css(&state);
        });
    }
}

/// Add or remove Image widgets so the marquee has the right number of slots
/// for the current pixel size and available width.
fn adjust_marquee_slots(state: &Rc<RefCell<MarqueeState>>) {
    let s = state.borrow();
    let width = s.win_width;
    let pixel_size = s.pixel_size;
    let slot_width = pixel_size + s.glyph_spacing;
    let available = if width > 2 * MARQUEE_PADDING {
        width - 2 * MARQUEE_PADDING
    } else {
        pixel_size
    };
    // Floor division: never add more slots than fit.
    let needed = ((((available + s.glyph_spacing) / slot_width).max(1) - 1).max(1)) as usize;
    let current = s.images.len();
    drop(s);

    let mut s = state.borrow_mut();
    if needed > current {
        for _ in current..needed {
            let img = Image::builder().css_classes(["marquee-dot-dim"]).halign(Align::Center).build();
            img.set_pixel_size(s.pixel_size);
            img.set_size_request(0, -1);
            img.insert_before(&s.marquee, Some(&s.spacer_right));
            s.images.push(img);
        }
    } else if needed < current {
        for _ in needed..current {
            if let Some(img) = s.images.pop() {
                s.marquee.remove(&img);
            }
        }
    }
}

fn update_marquee(state: &Rc<RefCell<MarqueeState>>) {
    let s = state.borrow();
    let text = &s.text;
    let num_slots = s.images.len();
    if num_slots == 0 {
        return;
    }
    if text.is_empty() {
        for img in &s.images {
            img.set_css_classes(&["marquee-dot-dim"]);
        }
        return;
    }

    // Build a padded text: spaces before and after so it scrolls in and out
    let padded_len = text.len() + num_slots;
    let padded: String = std::iter::repeat_n(' ', num_slots)
        .chain(text.chars())
        .chain(std::iter::repeat_n(' ', num_slots))
        .take(padded_len)
        .collect();

    for (i, img) in s.images.iter().enumerate() {
        img.set_pixel_size(s.pixel_size);
        let pos = (s.offset + i) % padded.len().max(1);
        let ch = padded.chars().nth(pos).unwrap_or(' ');

        match char_to_glyph_base(ch) {
            Some(base) => {
                let full_name = format!("{}-{}", variant::GLYPH_PREFIX, base);
                let resource_path = format!("{}/scalable/glyphs/{}.svg", variant::GRESOURCE_PREFIX, full_name,);
                // Alternate glow for a subtle pulse effect
                let hex_color = if (s.offset + i).is_multiple_of(3) {
                    rgba_to_hex(&s.glow_color)
                } else {
                    rgba_to_hex(&s.fg_color)
                };
                if let Some(texture) = colorized_texture(&resource_path, &hex_color) {
                    img.set_paintable(Some(&texture));
                    img.set_css_classes(&["marquee-dot"]);
                } else {
                    img.set_css_classes(&["marquee-dot-dim"]);
                }
            }
            None => {
                img.set_paintable(gtk4::gdk::Paintable::NONE);
                img.set_css_classes(&["marquee-dot-dim"]);
            }
        }
    }
}
