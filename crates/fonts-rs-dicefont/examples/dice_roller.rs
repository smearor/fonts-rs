//! Dice Roller — a tabletop RPG dice tray using Dicefont glyphs.
//!
//! Select your dice, roll them, and watch the die faces animate before
//! settling on the result. Features a roll history log, color customization,
//! and support for all standard TTRPG dice (D4, D6, D8, D10, D12, D20)
//! plus fate dice (F) and dot-d6 variants.
//!
//! Run with:
//! ```sh
//! cargo run --example dice_roller --features gtk,embed-fonts
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use fonts_rs_dicefont::register_glyphs;
use fonts_rs_dicefont::variant;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::ColorDialog;
use gtk4::ColorDialogButton;
use gtk4::Image;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;

use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.fonts_rs.dice_roller";

/// Die type specification: name, sides, and glyph name prefix.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DieType {
    name: &'static str,
    sides: u32,
    glyph_prefix: &'static str,
}

const DIE_TYPES: &[DieType] = &[
    DieType {
        name: "D4",
        sides: 4,
        glyph_prefix: "d4",
    },
    DieType {
        name: "D6",
        sides: 6,
        glyph_prefix: "d6",
    },
    DieType {
        name: "D8",
        sides: 8,
        glyph_prefix: "d8",
    },
    DieType {
        name: "D10",
        sides: 10,
        glyph_prefix: "d10",
    },
    DieType {
        name: "D12",
        sides: 12,
        glyph_prefix: "d12",
    },
    DieType {
        name: "D20",
        sides: 20,
        glyph_prefix: "d20",
    },
];

/// Fate die faces: minus, zero, plus.
const FATE_FACES: &[&str] = &["f-minus", "f-zero", "f-plus"];

/// Dot-d6 style for the "fancy" toggle.
const DOT_D6_FACES: &[&str] = &["dot-d6-1", "dot-d6-2", "dot-d6-3", "dot-d6-4", "dot-d6-5", "dot-d6-6"];

/// Number of animation frames before settling.
const ANIMATION_FRAMES: u32 = 12;

/// Animation interval in milliseconds.
const ANIMATION_INTERVAL_MS: u64 = 60;

/// Maximum dice per type.
const MAX_DICE: u32 = 8;

/// Convert a `gdk::RGBA` to a `#rrggbb` hex string.
fn rgba_to_hex(rgba: &gtk4::gdk::RGBA) -> String {
    format!("#{:02x}{:02x}{:02x}", (rgba.red() * 255.0) as u8, (rgba.green() * 255.0) as u8, (rgba.blue() * 255.0) as u8,)
}

/// Load an SVG from a GResource, replace `currentColor` with the given hex color,
/// and return a `gdk::Texture`.
fn colorized_texture(resource_path: &str, hex_color: &str) -> Option<gtk4::gdk::Texture> {
    let bytes = gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE).ok()?;
    let svg = std::str::from_utf8(bytes.as_ref()).ok()?;
    let colored = svg.replace("currentColor", hex_color);
    let bytes = glib::Bytes::from(colored.as_bytes());
    gtk4::gdk::Texture::from_bytes(&bytes).ok()
}

/// Build the GResource path for a glyph name.
fn glyph_resource_path(glyph_name: &str) -> String {
    format!("{}/scalable/glyphs/{}.svg", variant::GRESOURCE_PREFIX, glyph_name)
}

/// Build the full glyph name from prefix and face value.
fn face_glyph_name(die: &DieType, face: u32) -> String {
    format!("dicefont-{}-{}", die.glyph_prefix, face)
}

/// Pick a random face for a die type (1..=sides).
fn random_face(die: &DieType) -> u32 {
    let rng = simple_rng();
    (rng % die.sides) + 1
}

/// Simple LCG random number generator (no external dependency).
fn simple_rng() -> u32 {
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;
    static SEED: AtomicU64 = AtomicU64::new(0);
    let mut s = SEED.load(Ordering::Relaxed);
    if s == 0 {
        s = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    SEED.store(s, Ordering::Relaxed);
    (s >> 33) as u32
}

/// A single die widget with its current state.
struct DieWidget {
    image: Image,
    die: DieType,
    final_face: u32,
    final_face_name: String,
    is_fate: bool,
    is_dot_d6: bool,
    frame: u32,
}

/// Application state.
struct DiceState {
    dice_per_type: Vec<u32>,
    die_widgets: Vec<DieWidget>,
    rolling: bool,
    use_dot_d6: bool,
    use_fate: bool,
    pixel_size: i32,
    dice_color: gtk4::gdk::RGBA,
    bg_color: gtk4::gdk::RGBA,
    accent_color: gtk4::gdk::RGBA,
    history: Vec<String>,
    history_label: Label,
    total_label: Label,
    dice_tray: Box,
    css_provider: gtk4::CssProvider,
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
            background-color: #1a1a2e;
        }
        .dice-tray {
            background-color: #16213e;
            border-radius: 16px;
            padding: 24px;
        }
        .control-panel {
            background-color: #0f3460;
            border-radius: 12px;
            padding: 16px;
        }
        .die-button {
            background-color: #1a1a3e;
            border-radius: 8px;
            padding: 8px 16px;
            font-size: 14pt;
            font-weight: bold;
            color: #e94560;
        }
        .die-button:hover {
            background-color: #2a2a5e;
        }
        .die-button:active {
            background-color: #3a3a7e;
        }
        .roll-button {
            background-color: #e94560;
            border-radius: 12px;
            padding: 12px 32px;
            font-size: 16pt;
            font-weight: bold;
            color: white;
        }
        .roll-button:hover {
            background-color: #ff5570;
        }
        .roll-button:disabled {
            background-color: #555;
            color: #888;
        }
        .section-label {
            color: #e94560;
            font-size: 12pt;
            font-weight: bold;
        }
        .count-label {
            color: #aaa;
            font-size: 10pt;
        }
        .total-label {
            color: #e94560;
            font-size: 24pt;
            font-weight: bold;
        }
        .history-label {
            color: #888;
            font-size: 10pt;
            font-family: monospace;
        }
        .die-image {
            margin: 4px;
        }
        .toggle-button {
            background-color: #1a1a3e;
            border-radius: 6px;
            padding: 6px 12px;
            font-size: 11pt;
            color: #aaa;
        }
        .toggle-button:checked {
            background-color: #e94560;
            color: white;
        }
        "#,
    );
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dicefont Dice Roller")
        .default_width(900)
        .default_height(700)
        .build();

    window.connect_close_request({
        let app = app.clone();
        move |_| {
            app.quit();
            glib::Propagation::Proceed
        }
    });

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    // --- Dice tray ---
    let dice_tray = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .halign(Align::Fill)
        .hexpand(true)
        .css_classes(["dice-tray"])
        .build();

    let tray_header = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let tray_title = Label::builder().label("🎲 Dice Tray").css_classes(["section-label"]).build();
    tray_header.append(&tray_title);

    let total_label = Label::builder()
        .label("Total: —")
        .css_classes(["total-label"])
        .hexpand(true)
        .halign(Align::End)
        .build();
    tray_header.append(&total_label);

    dice_tray.append(&tray_header);

    // Grid area for dice display
    let dice_display = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    dice_display.set_size_request(0, 120);
    dice_tray.append(&dice_display);

    // History
    let history_label = Label::builder()
        .label("Roll history will appear here...")
        .css_classes(["history-label"])
        .halign(Align::Start)
        .hexpand(true)
        .wrap(true)
        .build();
    dice_tray.append(&history_label);

    main_box.append(&dice_tray);

    // --- Control panel ---
    let controls = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .css_classes(["control-panel"])
        .build();

    // Die count selectors
    let dice_row_label = Label::builder()
        .label("Select Your Dice")
        .css_classes(["section-label"])
        .halign(Align::Start)
        .build();
    controls.append(&dice_row_label);

    let dice_selectors = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Fill)
        .hexpand(true)
        .build();

    let mut count_labels: Vec<Label> = Vec::new();
    let mut minus_buttons: Vec<Button> = Vec::new();
    let mut plus_buttons: Vec<Button> = Vec::new();

    for die in DIE_TYPES {
        let die_col = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .hexpand(true)
            .halign(Align::Fill)
            .build();

        let die_name = Label::builder().label(die.name).css_classes(["section-label"]).build();
        die_col.append(&die_name);

        let count_row = Box::builder().orientation(Orientation::Horizontal).spacing(4).halign(Align::Center).build();

        let minus_btn = Button::with_label("−");
        minus_btn.add_css_class("die-button");
        minus_btn.set_size_request(36, 36);

        let count_label = Label::builder().label("0").css_classes(["count-label"]).build();
        count_label.set_size_request(24, 36);

        let plus_btn = Button::with_label("+");
        plus_btn.add_css_class("die-button");
        plus_btn.set_size_request(36, 36);

        count_row.append(&minus_btn);
        count_row.append(&count_label);
        count_row.append(&plus_btn);

        die_col.append(&count_row);
        dice_selectors.append(&die_col);

        count_labels.push(count_label);
        minus_buttons.push(minus_btn);
        plus_buttons.push(plus_btn);
    }

    controls.append(&dice_selectors);

    // Toggle row: fate dice + dot-d6 style
    let toggle_row = Box::builder().orientation(Orientation::Horizontal).spacing(12).halign(Align::Center).build();

    let fate_toggle = gtk4::ToggleButton::with_label("Fate Dice (F)");
    fate_toggle.add_css_class("toggle-button");
    fate_toggle.set_tooltip_text(Some("Replace D6 with Fate/Fudge dice (−, 0, +)"));
    toggle_row.append(&fate_toggle);

    let dot_toggle = gtk4::ToggleButton::with_label("Dot D6 Style");
    dot_toggle.add_css_class("toggle-button");
    dot_toggle.set_tooltip_text(Some("Use pip-style dot faces for D6 instead of numerals"));
    toggle_row.append(&dot_toggle);

    controls.append(&toggle_row);

    // Size slider
    let size_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let size_label = Label::builder().label("Size:").css_classes(["count-label"]).build();
    size_row.append(&size_label);

    let size_scale = gtk4::Scale::with_range(Orientation::Horizontal, 32.0, 128.0, 8.0);
    size_scale.set_value(64.0);
    size_scale.set_draw_value(false);
    size_scale.set_hexpand(true);
    for mark in [32, 48, 64, 96, 128] {
        size_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    size_row.append(&size_scale);
    controls.append(&size_row);

    // Color pickers
    let color_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let color_dialog = ColorDialog::new();

    let dice_btn = ColorDialogButton::builder().tooltip_text("Dice color").build();
    dice_btn.set_dialog(&color_dialog);
    dice_btn.set_rgba(&gtk4::gdk::RGBA::new(0.85, 0.15, 0.25, 1.0));
    color_row.append(&dice_btn);

    let bg_btn = ColorDialogButton::builder().tooltip_text("Tray background").build();
    bg_btn.set_dialog(&color_dialog);
    bg_btn.set_rgba(&gtk4::gdk::RGBA::new(0.09, 0.13, 0.24, 1.0));
    color_row.append(&bg_btn);

    let accent_btn = ColorDialogButton::builder().tooltip_text("Accent / text").build();
    accent_btn.set_dialog(&color_dialog);
    accent_btn.set_rgba(&gtk4::gdk::RGBA::new(0.91, 0.27, 0.38, 1.0));
    color_row.append(&accent_btn);

    controls.append(&color_row);

    // Roll button
    let roll_button = Button::with_label("🎲 ROLL!");
    roll_button.add_css_class("roll-button");
    roll_button.set_halign(Align::Center);
    roll_button.set_margin_top(8);
    controls.append(&roll_button);

    main_box.append(&controls);

    window.set_child(Some(&main_box));
    window.present();

    // --- State ---
    let dynamic_provider = gtk4::CssProvider::new();
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &dynamic_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    let state = Rc::new(RefCell::new(DiceState {
        dice_per_type: vec![0; DIE_TYPES.len()],
        die_widgets: Vec::new(),
        rolling: false,
        use_dot_d6: false,
        use_fate: false,
        pixel_size: 64,
        dice_color: gtk4::gdk::RGBA::new(0.85, 0.15, 0.25, 1.0),
        bg_color: gtk4::gdk::RGBA::new(0.09, 0.13, 0.24, 1.0),
        accent_color: gtk4::gdk::RGBA::new(0.91, 0.27, 0.38, 1.0),
        history: Vec::new(),
        history_label: history_label.clone(),
        total_label: total_label.clone(),
        dice_tray: dice_display.clone(),
        css_provider: dynamic_provider,
    }));

    update_dynamic_css(&state);

    // --- Wire up die count buttons ---
    for (i, die) in DIE_TYPES.iter().enumerate() {
        let state_minus = state.clone();
        let label_minus = count_labels[i].clone();
        let minus = minus_buttons[i].clone();

        minus.connect_clicked(move |_| {
            let mut s = state_minus.borrow_mut();
            if s.dice_per_type[i] > 0 {
                s.dice_per_type[i] -= 1;
                label_minus.set_label(&s.dice_per_type[i].to_string());
            }
        });

        let state_plus = state.clone();
        let label_plus = count_labels[i].clone();
        let plus = plus_buttons[i].clone();
        let _ = die;
        plus.connect_clicked(move |_| {
            let mut s = state_plus.borrow_mut();
            if s.dice_per_type[i] < MAX_DICE {
                s.dice_per_type[i] += 1;
                label_plus.set_label(&s.dice_per_type[i].to_string());
            }
        });
    }

    // Initialize count labels
    for (i, label) in count_labels.iter().enumerate() {
        label.set_label(&state.borrow().dice_per_type[i].to_string());
    }

    // --- Toggle buttons ---
    {
        let state = state.clone();
        fate_toggle.connect_toggled(move |btn| {
            let mut s = state.borrow_mut();
            s.use_fate = btn.is_active();
        });
    }

    {
        let state = state.clone();
        dot_toggle.connect_toggled(move |btn| {
            let mut s = state.borrow_mut();
            s.use_dot_d6 = btn.is_active();
        });
    }

    // --- Size slider ---
    {
        let state = state.clone();
        size_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.pixel_size = scale.value() as i32;
            for dw in &s.die_widgets {
                dw.image.set_pixel_size(s.pixel_size);
            }
        });
    }

    // --- Color pickers ---
    {
        let state = state.clone();
        dice_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.dice_color = btn.rgba();
            drop(s);
            recolor_dice(&state);
        });
    }

    {
        let state = state.clone();
        bg_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.bg_color = btn.rgba();
            drop(s);
            update_dynamic_css(&state);
        });
    }

    {
        let state = state.clone();
        accent_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.accent_color = btn.rgba();
            drop(s);
            update_dynamic_css(&state);
        });
    }

    // --- Roll button ---
    {
        let state = state.clone();
        roll_button.connect_clicked(move |_| {
            start_roll(&state);
        });
    }
}

/// Update the dynamic CSS with current colors.
fn update_dynamic_css(state: &Rc<RefCell<DiceState>>) {
    let s = state.borrow();
    let bg = &s.bg_color;
    let accent = &s.accent_color;
    let accent_hex = rgba_to_hex(accent);
    let bg_css = format!("rgba({}, {}, {}, 1.0)", (bg.red() * 255.0) as u8, (bg.green() * 255.0) as u8, (bg.blue() * 255.0) as u8,);
    let css = format!(
        ".dice-tray {{ background-color: {bg_css}; }}\n\
         .section-label {{ color: {accent_hex}; }}\n\
         .total-label {{ color: {accent_hex}; }}\n"
    );
    s.css_provider.load_from_string(&css);
}

/// Recolor all displayed dice with the current dice color.
fn recolor_dice(state: &Rc<RefCell<DiceState>>) {
    let s = state.borrow();
    let hex = rgba_to_hex(&s.dice_color);
    for dw in &s.die_widgets {
        let glyph = if dw.frame >= ANIMATION_FRAMES {
            format!("dicefont-{}", dw.final_face_name)
        } else {
            // During animation, show a random face
            let r = random_face(&dw.die);
            face_glyph_name(&dw.die, r)
        };
        let path = glyph_resource_path(&glyph);
        if let Some(texture) = colorized_texture(&path, &hex) {
            dw.image.set_paintable(Some(&texture));
        }
    }
}

/// Start a dice roll animation.
fn start_roll(state: &Rc<RefCell<DiceState>>) {
    let mut s = state.borrow_mut();
    if s.rolling {
        return;
    }

    // Collect dice to roll
    let has_dice = s.dice_per_type.iter().any(|&c| c > 0);

    if !has_dice {
        return;
    }

    s.rolling = true;

    // Clear existing dice
    let tray = s.dice_tray.clone();
    for dw in s.die_widgets.drain(..) {
        tray.remove(&dw.image);
    }

    // Create new die widgets
    let pixel_size = s.pixel_size;
    let hex_color = rgba_to_hex(&s.dice_color);

    let mut widgets = Vec::new();

    // Add standard dice
    for (i, die) in DIE_TYPES.iter().enumerate() {
        for _ in 0..s.dice_per_type[i] {
            let is_fate = die.name == "D6" && s.use_fate;
            let is_dot_d6 = die.name == "D6" && s.use_dot_d6;

            let img = Image::new();
            img.set_pixel_size(pixel_size);
            img.add_css_class("die-image");

            let initial_glyph = if is_fate {
                format!("dicefont-{}", FATE_FACES[0])
            } else if is_dot_d6 {
                format!("dicefont-{}", DOT_D6_FACES[0])
            } else {
                let face = random_face(die);
                face_glyph_name(die, face)
            };

            let path = glyph_resource_path(&initial_glyph);
            if let Some(texture) = colorized_texture(&path, &hex_color) {
                img.set_paintable(Some(&texture));
            }
            s.dice_tray.append(&img);
            widgets.push(DieWidget {
                image: img,
                die: *die,
                final_face: 1,
                final_face_name: initial_glyph,
                is_fate,
                is_dot_d6,
                frame: 0,
            });
        }
    }

    s.die_widgets = widgets;
    drop(s);

    // Start animation
    animate_roll(state);
}

/// Animate the dice roll, cycling random faces before settling.
fn animate_roll(state: &Rc<RefCell<DiceState>>) {
    let s = state.borrow();
    if !s.rolling {
        return;
    }

    let hex_color = rgba_to_hex(&s.dice_color);

    // Check if all dice have finished animating
    let all_done = s.die_widgets.iter().all(|dw| dw.frame >= ANIMATION_FRAMES);
    if all_done {
        drop(s);
        finish_roll(state);
        return;
    }

    // Advance each die's animation
    let mut updates: Vec<(usize, String, u32, String)> = Vec::new();

    for (idx, dw) in s.die_widgets.iter().enumerate() {
        if dw.frame >= ANIMATION_FRAMES {
            continue;
        }

        let new_frame = dw.frame + 1;

        let (glyph, final_face, final_name) = if new_frame >= ANIMATION_FRAMES {
            // Final face — settle on a result
            if dw.is_fate {
                let fate_idx = (simple_rng() % 3) as usize;
                let fate_name = FATE_FACES[fate_idx].to_string();
                let fate_val = if fate_idx == 0 {
                    0u32
                } else if fate_idx == 1 {
                    1u32
                } else {
                    2u32
                };
                (format!("dicefont-{}", fate_name), fate_val, fate_name)
            } else if dw.is_dot_d6 {
                let face = random_face(&dw.die);
                let dot_name = DOT_D6_FACES[(face - 1) as usize].to_string();
                (format!("dicefont-{}", dot_name), face, dot_name)
            } else {
                let face = random_face(&dw.die);
                let name = format!("{}-{}", dw.die.glyph_prefix, face);
                (format!("dicefont-{}", name), face, name)
            }
        } else {
            // Random face during animation
            let glyph = if dw.is_fate {
                let fate_idx = (simple_rng() % 3) as usize;
                format!("dicefont-{}", FATE_FACES[fate_idx])
            } else if dw.is_dot_d6 {
                let dot_idx = (simple_rng() % 6) as usize;
                format!("dicefont-{}", DOT_D6_FACES[dot_idx])
            } else {
                let face = random_face(&dw.die);
                face_glyph_name(&dw.die, face)
            };
            (glyph, dw.final_face, dw.final_face_name.clone())
        };

        updates.push((idx, glyph, final_face, final_name));
    }

    drop(s);

    // Apply updates
    {
        let mut s = state.borrow_mut();
        for (idx, glyph, final_face, final_name) in &updates {
            let dw = &mut s.die_widgets[*idx];
            dw.frame += 1;
            dw.final_face = *final_face;
            dw.final_face_name = final_name.clone();
            let path = glyph_resource_path(glyph);
            if let Some(texture) = colorized_texture(&path, &hex_color) {
                dw.image.set_paintable(Some(&texture));
            }
        }
    }

    // Schedule next frame (one-shot: Stop prevents duplicate timers)
    let state = state.clone();
    glib::source::timeout_add_local(std::time::Duration::from_millis(ANIMATION_INTERVAL_MS), move || {
        animate_roll(&state);
        glib::ControlFlow::Break
    });
}

/// Finish the roll: compute totals, update history.
fn finish_roll(state: &Rc<RefCell<DiceState>>) {
    let mut s = state.borrow_mut();
    s.rolling = false;

    // Compute total from stored final faces
    let mut total: i32 = 0;
    let mut roll_parts: Vec<String> = Vec::new();

    for dw in &s.die_widgets {
        if dw.is_fate {
            // Fate: final_face 0 = minus (-1), 1 = zero (0), 2 = plus (+1)
            let val = match dw.final_face {
                0 => -1,
                1 => 0,
                _ => 1,
            };
            total += val;
            let sign = if val < 0 {
                "−"
            } else if val > 0 {
                "+"
            } else {
                "0"
            };
            roll_parts.push(format!("F({})", sign));
        } else {
            total += dw.final_face as i32;
            let prefix = if dw.is_dot_d6 { "D6" } else { dw.die.name };
            roll_parts.push(format!("{}({})", prefix, dw.final_face));
        }
    }

    if roll_parts.is_empty() {
        s.total_label.set_label("Total: —");
    } else {
        s.total_label.set_label(&format!("Total: {}", total));
    }

    // Update history
    let history_entry = if roll_parts.is_empty() {
        "—".to_string()
    } else {
        format!("{} = {}", roll_parts.join(" + "), total)
    };
    s.history.push(history_entry);
    if s.history.len() > 20 {
        s.history.remove(0);
    }
    let history_text = s.history.iter().rev().cloned().collect::<Vec<_>>().join("\n");
    s.history_label.set_label(&history_text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die_types_are_correct() {
        assert_eq!(DIE_TYPES.len(), 6);
        assert_eq!(DIE_TYPES[0].name, "D4");
        assert_eq!(DIE_TYPES[5].name, "D20");
    }

    #[test]
    fn face_glyph_name_format() {
        let die = DIE_TYPES[5]; // D20
        assert_eq!(face_glyph_name(&die, 20), "dicefont-d20-20");
        assert_eq!(face_glyph_name(&die, 1), "dicefont-d20-1");
    }

    #[test]
    fn random_face_in_range() {
        let die = DIE_TYPES[0]; // D4
        for _ in 0..100 {
            let face = random_face(&die);
            assert!(face >= 1 && face <= 4);
        }
    }
}
