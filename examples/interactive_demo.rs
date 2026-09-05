//! Interactive demo for nerd-fonts-gtk.
//!
//! Demonstrates all major capabilities of the library:
//! - Icon name resolution (`resolve_icon_codepoint`)
//! - GTK icon name resolution (`resolve_gtk_nerd_icon`)
//! - Icon color application (`apply_icon_color`)
//! - Text label coloring (`apply_text_color`)
//! - Software rendering onto pixel buffers (`drawing` module)
//!
//! Run with:
//! ```sh
//! cargo run --example interactive_demo --features gtk,render,embed-fonts
//! ```

use nerd_fonts_gtk::color::Color;
use nerd_fonts_gtk::drawing;
use nerd_fonts_gtk::gtk::apply_text_color;
use nerd_fonts_gtk::gtk::resolve_gtk_nerd_icon;
use nerd_fonts_gtk::init;
use nerd_fonts_gtk::resolve_icon_codepoint;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::DropDown;
use gtk4::Entry;
use gtk4::Frame;
use gtk4::Image;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::glib;
use gtk4::prelude::*;

const APP_ID: &str = "io.smearor.nerd_fonts_gtk.interactive_demo";

/// A curated selection of Nerd Font icons to display in the icon gallery.
const GALLERY_ICONS: &[&str] = &[
    "nf-fa-gamepad",
    "nf-md-cube",
    "nf-linux-tux",
    "nf-fa-star",
    "nf-md-cog",
    "nf-fa-heart",
    "nf-md-battery",
    "nf-fa-music",
    "nf-md-weather_sunny",
    "nf-fa-camera",
    "nf-md-email",
    "nf-fa-bug",
];

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    init(None);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("nerd-fonts-gtk Interactive Demo")
        .default_width(1024)
        .default_height(1280)
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
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("nerd-fonts-gtk Interactive Demo")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Explore icon resolution, color application, and software rendering.")
        .halign(Align::Center)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    main_box.append(&build_resolution_section());
    main_box.append(&build_gallery_section());
    main_box.append(&build_label_section());
    main_box.append(&build_rendering_section());

    let scrolled = gtk4::ScrolledWindow::builder()
        .child(&main_box)
        .hexpand(true)
        .vexpand(true)
        .build();

    window.set_child(Some(&scrolled));
    window.present();
}

// ---------------------------------------------------------------------------
// Section 1: Icon Resolution
// ---------------------------------------------------------------------------

fn build_resolution_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("1. Icon Name Resolution")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Enter a Nerd Font icon name (e.g. nf-fa-gamepad) to resolve its Unicode codepoint and GTK icon name.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let input_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();

    let entry = Entry::builder().placeholder_text("nf-fa-gamepad").text("nf-fa-gamepad").hexpand(true).build();
    input_row.append(&entry);

    let resolve_button = Button::with_label("Resolve");
    input_row.append(&resolve_button);

    section.append(&input_row);

    let result_label = Label::builder()
        .label("Codepoint: —\nGTK icon name: —")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    section.append(&result_label);

    // Quick-select dropdown
    let quick_label = Label::builder().label("Or pick from common icons:").halign(Align::Start).build();
    section.append(&quick_label);

    let icon_names: Vec<&str> = GALLERY_ICONS.to_vec();
    let dropdown = DropDown::from_strings(&icon_names);
    section.append(&dropdown);

    // Initial resolution
    do_resolve(&result_label, "nf-fa-gamepad");

    resolve_button.connect_clicked(glib::clone!(
        #[weak]
        entry,
        #[weak]
        result_label,
        move |_| {
            let name = entry.text().to_string();
            do_resolve(&result_label, &name);
        }
    ));

    entry.connect_activate(glib::clone!(
        #[weak]
        resolve_button,
        move |_| {
            resolve_button.emit_clicked();
        }
    ));

    dropdown.connect_selected_notify(glib::clone!(
        #[weak]
        entry,
        #[weak]
        resolve_button,
        move |dd| {
            if let Some(text) = dd.selected_item().and_then(|i| i.downcast::<gtk4::StringObject>().ok()) {
                entry.set_text(text.string().as_str());
                resolve_button.emit_clicked();
            }
        }
    ));

    Frame::builder().label("Icon Resolution").child(&section).build()
}

fn do_resolve(result_label: &Label, name: &str) {
    if name.is_empty() {
        result_label.set_label("Codepoint: —\nGTK icon name: —");
        return;
    }
    let cp = resolve_icon_codepoint(name);
    let gtk_name = resolve_gtk_nerd_icon(name);
    match cp {
        Some(c) => result_label.set_label(&format!("Codepoint: U+{:04X} ('{}')\nGTK icon name: {}", c as u32, c, gtk_name.as_deref().unwrap_or("—"))),
        None => result_label.set_label(&format!("Codepoint: not found\nGTK icon name: {}", gtk_name.as_deref().unwrap_or("—"))),
    }
}

// ---------------------------------------------------------------------------
// Section 2: Icon Gallery with Color Controls
// ---------------------------------------------------------------------------

fn build_gallery_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("2. Icon Gallery with Color Controls")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Nerd Font icons rendered as labels using the @font-face CSS loaded by init(). Adjust the RGBA sliders to recolor the preview icon.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    // Icon grid - use Labels with Nerd Font codepoints + nerd-icon CSS class
    let grid = gtk4::Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let cols = 6u32;

    let gallery_css = ".nerd-icon-large { font-size: 32px; } .nerd-icon-clickable { padding: 6px; border-radius: 6px; } .nerd-icon-clickable:hover { background-color: rgba(100, 180, 255, 0.15); } .nerd-icon-selected { background-color: rgba(100, 180, 255, 0.25); }";
    let gallery_provider = gtk4::CssProvider::new();
    gallery_provider.load_from_data(gallery_css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &gallery_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    for (i, icon_name) in GALLERY_ICONS.iter().enumerate() {
        let codepoint = resolve_icon_codepoint(icon_name).map(|c| c.to_string()).unwrap_or_else(|| "?".to_string());

        let icon = Label::builder()
            .label(&codepoint)
            .css_classes(["nerd-icon", "nerd-icon-large", "nerd-icon-clickable"])
            .halign(Align::Center)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        let label = Label::builder()
            .label(*icon_name)
            .css_classes(["caption", "dim-label"])
            .wrap(true)
            .max_width_chars(18)
            .build();

        let vbox = Box::builder().orientation(Orientation::Vertical).spacing(4).halign(Align::Center).build();
        vbox.append(&icon);
        vbox.append(&label);

        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        grid.attach(&vbox, col as i32, row as i32, 1, 1);
    }

    section.append(&grid);

    // Color controls
    let color_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Fill)
        .margin_top(8)
        .build();

    let (r_scale, g_scale, b_scale, a_scale) = build_rgba_sliders(100.0, 180.0, 255.0, 1.0);
    color_box.append(&Label::new(Some("R")));
    color_box.append(&r_scale);
    color_box.append(&Label::new(Some("G")));
    color_box.append(&g_scale);
    color_box.append(&Label::new(Some("B")));
    color_box.append(&b_scale);
    color_box.append(&Label::new(Some("A")));
    color_box.append(&a_scale);

    section.append(&color_box);

    // Preview icon
    let preview_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .build();

    let preview_codepoint = resolve_icon_codepoint("nf-fa-gamepad")
        .map(|c| c.to_string())
        .unwrap_or_else(|| "?".to_string());

    let preview_icon = Label::builder()
        .label(&preview_codepoint)
        .css_classes(["nerd-icon", "nerd-icon-large"])
        .halign(Align::Center)
        .build();

    let color_preview = Label::builder()
        .label("rgba(100, 180, 255, 1.00)")
        .css_classes(["monospace", "dim-label"])
        .build();

    preview_box.append(&preview_icon);
    preview_box.append(&color_preview);
    section.append(&preview_box);

    // Apply initial color via apply_text_color (works on labels)
    let initial_color = current_color(&r_scale, &g_scale, &b_scale, &a_scale);
    apply_text_color(&preview_icon, Some(initial_color));
    update_color_preview(&color_preview, initial_color);

    // Connect sliders to preview icon
    for scale in [&r_scale, &g_scale, &b_scale, &a_scale] {
        scale.connect_value_changed(glib::clone!(
            #[weak]
            r_scale,
            #[weak]
            g_scale,
            #[weak]
            b_scale,
            #[weak]
            a_scale,
            #[weak]
            preview_icon,
            #[weak]
            color_preview,
            move |_| {
                let color = current_color(&r_scale, &g_scale, &b_scale, &a_scale);
                apply_text_color(&preview_icon, Some(color));
                update_color_preview(&color_preview, color);
            }
        ));
    }

    // Make gallery icons clickable to set as preview
    for (i, icon_name) in GALLERY_ICONS.iter().enumerate() {
        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        if let Some(vbox) = grid.child_at(col as i32, row as i32).and_then(|w| w.downcast::<gtk4::Box>().ok()) {
            if let Some(icon_label) = vbox.first_child().and_then(|w| w.downcast::<Label>().ok()) {
                let click = gtk4::GestureClick::new();
                let name = icon_name.to_string();
                click.connect_released(glib::clone!(
                    #[weak]
                    preview_icon,
                    #[weak]
                    r_scale,
                    #[weak]
                    g_scale,
                    #[weak]
                    b_scale,
                    #[weak]
                    a_scale,
                    #[weak]
                    icon_label,
                    #[weak]
                    grid,
                    move |_, _, _, _| {
                        if let Some(cp) = resolve_icon_codepoint(&name) {
                            preview_icon.set_label(&cp.to_string());
                            let color = current_color(&r_scale, &g_scale, &b_scale, &a_scale);
                            apply_text_color(&preview_icon, Some(color));
                        }
                        let rows = (GALLERY_ICONS.len() as i32 + cols as i32 - 1) / cols as i32;
                        for r in 0..rows {
                            for c in 0..cols as i32 {
                                if let Some(child) = grid.child_at(c, r) {
                                    if let Some(lbl) = child.first_child().and_then(|w| w.downcast::<Label>().ok()) {
                                        lbl.remove_css_class("nerd-icon-selected");
                                    }
                                }
                            }
                        }
                        icon_label.add_css_class("nerd-icon-selected");
                    }
                ));
                icon_label.add_controller(click);
            }
        }
    }

    Frame::builder().label("Icon Gallery").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 3: Semantic Text Coloring
// ---------------------------------------------------------------------------

fn build_label_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("3. Semantic Text Coloring")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("apply_text_color is used for semantic status colors (e.g. sensor readings, battery levels). Click a preset to see the color change.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    // Status labels with icons
    let status_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let temp_label = Label::builder()
        .label("\u{F2C9}  Temperature: 23.4 °C")
        .css_classes(["nerd-icon", "title-2"])
        .halign(Align::Center)
        .build();
    status_box.append(&temp_label);

    let battery_label = Label::builder()
        .label("\u{F240}  Battery: 87 %")
        .css_classes(["nerd-icon", "title-2"])
        .halign(Align::Center)
        .build();
    status_box.append(&battery_label);

    let signal_label = Label::builder()
        .label("\u{F1EB}  Signal: Good")
        .css_classes(["nerd-icon", "title-2"])
        .halign(Align::Center)
        .build();
    status_box.append(&signal_label);

    section.append(&status_box);

    // Preset buttons
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Center)
        .margin_top(8)
        .build();

    let normal_btn = Button::with_label("Normal");
    let warning_btn = Button::with_label("Warning");
    let critical_btn = Button::with_label("Critical");
    let success_btn = Button::with_label("Success");
    let reset_btn = Button::with_label("Reset");

    button_box.append(&normal_btn);
    button_box.append(&warning_btn);
    button_box.append(&critical_btn);
    button_box.append(&success_btn);
    button_box.append(&reset_btn);
    section.append(&button_box);

    // Apply initial color (normal)
    apply_text_color(&temp_label, Some(Color::new(0.4, 0.4, 0.4)));
    apply_text_color(&battery_label, Some(Color::new(0.4, 0.4, 0.4)));
    apply_text_color(&signal_label, Some(Color::new(0.4, 0.4, 0.4)));

    normal_btn.connect_clicked(glib::clone!(
        #[weak]
        temp_label,
        #[weak]
        battery_label,
        #[weak]
        signal_label,
        move |_| {
            let color = Color::new(0.4, 0.4, 0.4);
            apply_text_color(&temp_label, Some(color));
            apply_text_color(&battery_label, Some(color));
            apply_text_color(&signal_label, Some(color));
        }
    ));

    warning_btn.connect_clicked(glib::clone!(
        #[weak]
        temp_label,
        #[weak]
        battery_label,
        #[weak]
        signal_label,
        move |_| {
            let color = Color::new(0.9, 0.6, 0.0);
            apply_text_color(&temp_label, Some(color));
            apply_text_color(&battery_label, Some(color));
            apply_text_color(&signal_label, Some(color));
        }
    ));

    critical_btn.connect_clicked(glib::clone!(
        #[weak]
        temp_label,
        #[weak]
        battery_label,
        #[weak]
        signal_label,
        move |_| {
            let color = Color::new(0.9, 0.2, 0.2);
            apply_text_color(&temp_label, Some(color));
            apply_text_color(&battery_label, Some(color));
            apply_text_color(&signal_label, Some(color));
        }
    ));

    success_btn.connect_clicked(glib::clone!(
        #[weak]
        temp_label,
        #[weak]
        battery_label,
        #[weak]
        signal_label,
        move |_| {
            let color = Color::new(0.2, 0.7, 0.3);
            apply_text_color(&temp_label, Some(color));
            apply_text_color(&battery_label, Some(color));
            apply_text_color(&signal_label, Some(color));
        }
    ));

    reset_btn.connect_clicked(glib::clone!(
        #[weak]
        temp_label,
        #[weak]
        battery_label,
        #[weak]
        signal_label,
        move |_| {
            apply_text_color::<Color>(&temp_label, None);
            apply_text_color::<Color>(&battery_label, None);
            apply_text_color::<Color>(&signal_label, None);
        }
    ));

    Frame::builder().label("Semantic Coloring").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 4: Software Rendering
// ---------------------------------------------------------------------------

fn build_rendering_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("4. Software Rendering (ab_glyph)")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Icons rendered onto raw RGBA pixel buffers without GTK, then displayed as GTK images.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let grid = gtk4::Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let cols = 4u32;
    let icon_size: u32 = 96;

    for (i, icon_name) in GALLERY_ICONS.iter().take(8).enumerate() {
        let mut pixels = vec![0u8; (icon_size * icon_size * 4) as usize];
        drawing::fill_background(&mut pixels, icon_size, icon_size, [30, 30, 30, 255]);
        drawing::draw_nerd_font_icon(&mut pixels, icon_size, icon_size, icon_name, true, resolve_icon_codepoint, Some([100, 180, 255, 255]));
        drawing::draw_label_text(&mut pixels, icon_size, icon_size, &icon_name.replace("nf-", ""), false, None);

        let bytes = glib::Bytes::from(&pixels);
        let rowstride = icon_size * 4;
        let texture = gtk4::gdk::MemoryTexture::new(icon_size as i32, icon_size as i32, gtk4::gdk::MemoryFormat::R8g8b8a8, &bytes, rowstride as usize);

        let image = Image::from_paintable(Some(&texture));
        image.set_halign(Align::Center);
        image.set_pixel_size(icon_size as i32);
        image.set_width_request(icon_size as i32);
        image.set_height_request(icon_size as i32);

        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        grid.attach(&image, col as i32, row as i32, 1, 1);
    }

    section.append(&grid);

    // Progress bar demo (dynamic)
    let progress_label = Label::builder().label("Progress bar rendering demo (drag slider to update):").halign(Align::Start).build();
    section.append(&progress_label);

    let progress_width: u32 = 256;
    let progress_height: u32 = 32;

    let render_progress = move |value: f32| -> gtk4::gdk::MemoryTexture {
        let mut pixels = vec![0u8; (progress_width * progress_height * 4) as usize];
        drawing::fill_background(&mut pixels, progress_width, progress_height, [20, 20, 20, 255]);
        drawing::draw_progress_bar(&mut pixels, progress_width, progress_height, value, [100, 180, 255, 255]);
        let bytes = glib::Bytes::from_owned(pixels);
        gtk4::gdk::MemoryTexture::new(
            progress_width as i32,
            progress_height as i32,
            gtk4::gdk::MemoryFormat::R8g8b8a8,
            &bytes,
            (progress_width * 4) as usize,
        )
    };

    let progress_texture = render_progress(0.65);
    let progress_image = Image::from_paintable(Some(&progress_texture));
    progress_image.set_halign(Align::Center);
    progress_image.set_pixel_size(progress_width as i32);
    progress_image.set_width_request(progress_width as i32);
    progress_image.set_height_request(progress_height as i32);
    section.append(&progress_image);

    let progress_slider = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    progress_slider.set_value(0.65);
    progress_slider.set_halign(Align::Center);
    progress_slider.set_width_request(256);
    section.append(&progress_slider);

    progress_slider.connect_value_changed(glib::clone!(
            #[weak]
            progress_image,
            move |slider| {
                let texture = render_progress(slider.value() as f32);
                progress_image.set_paintable(Some(&texture));
            }
        ));

    Frame::builder().label("Software Rendering").child(&section).build()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn build_rgba_sliders(r: f64, g: f64, b: f64, a: f64) -> (Scale, Scale, Scale, Scale) {
    let r_scale = Scale::with_range(Orientation::Horizontal, 0.0, 255.0, 1.0);
    r_scale.set_value(r);
    r_scale.set_hexpand(true);

    let g_scale = Scale::with_range(Orientation::Horizontal, 0.0, 255.0, 1.0);
    g_scale.set_value(g);
    g_scale.set_hexpand(true);

    let b_scale = Scale::with_range(Orientation::Horizontal, 0.0, 255.0, 1.0);
    b_scale.set_value(b);
    b_scale.set_hexpand(true);

    let a_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.05);
    a_scale.set_value(a);
    a_scale.set_hexpand(true);

    (r_scale, g_scale, b_scale, a_scale)
}

fn current_color(r: &Scale, g: &Scale, b: &Scale, a: &Scale) -> Color {
    Color::new_rgba(r.value() / 255.0, g.value() / 255.0, b.value() / 255.0, a.value())
}

fn update_color_preview(label: &Label, color: Color) {
    label.set_label(&format!(
        "rgba({}, {}, {}, {:.2})",
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8,
        color.a
    ));
}
