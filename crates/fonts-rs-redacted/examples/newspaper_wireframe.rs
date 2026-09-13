use std::cell::RefCell;
use std::rc::Rc;

use fonts_rs_redacted::variant;
use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::ColorDialogButton;
use gtk4::ContentFit;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Picture;
use gtk4::glib;
use gtk4::prelude::*;

use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.fonts.rs.redacted.newspaper_wireframe";

/// All Redacted variants: (feature_name, label).
const VARIANTS: &[(&str, &str)] = &[
    ("redacted", "Redacted (Block)"),
    ("script-light", "Script Light"),
    ("script-regular", "Script Regular"),
    ("script-bold", "Script Bold"),
];

/// Base size for glyph rasterization in the composed SVG.
const BASE_GLYPH_WIDTH: u32 = 20;
const BASE_GLYPH_HEIGHT: u32 = 50;

/// Map an ASCII character to its Redacted glyph base name.
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

fn rgba_to_hex(rgba: &gtk4::gdk::RGBA) -> String {
    format!("#{:02x}{:02x}{:02x}", (rgba.red() * 255.0) as u8, (rgba.green() * 255.0) as u8, (rgba.blue() * 255.0) as u8,)
}

/// Extract the viewBox attribute value from an SVG string.
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

/// Extract the inner content of an SVG (between the opening tag and </svg>).
fn extract_svg_inner(svg: &str) -> &str {
    if let Some(svg_start) = svg.find("<svg") {
        if let Some(tag_end) = svg[svg_start..].find('>') {
            let content_start = svg_start + tag_end + 1;
            if let Some(close) = svg.rfind("</svg>") {
                return &svg[content_start..close];
            }
        }
    }
    ""
}

/// Compose a text string into a single SVG with all glyphs side by side.
/// Uses nested `<svg>` elements to handle different glyph viewBoxes.
fn compose_text_svg(text: &str, hex_color: &str) -> String {
    let prefix = variant::GLYPH_PREFIX;
    let gresource_prefix = variant::GRESOURCE_PREFIX;

    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let total_width = (n as u32 * BASE_GLYPH_WIDTH).max(1);

    let mut inner = String::new();
    for (i, ch) in chars.iter().enumerate() {
        let Some(base) = char_to_glyph_base(*ch) else {
            continue;
        };
        let full_name = format!("{prefix}-{base}");
        let resource_path = format!("{gresource_prefix}/{}/{}/{full_name}.svg", fonts_rs_model::SCALABLE_DIR, fonts_rs_model::ICONS_CONTEXT_GLYPHS);
        let Ok(bytes) = gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE) else {
            continue;
        };
        let Ok(svg) = std::str::from_utf8(bytes.as_ref()) else {
            continue;
        };

        let viewbox = extract_svg_viewbox(svg);
        let content = extract_svg_inner(svg);
        let colored = content.replace("currentColor", hex_color);

        let x = i as u32 * BASE_GLYPH_WIDTH;
        inner.push_str(&format!(
            "<svg x=\"{x}\" y=\"0\" width=\"{BASE_GLYPH_WIDTH}\" height=\"{BASE_GLYPH_HEIGHT}\" viewBox=\"{viewbox}\">{colored}</svg>"
        ));
    }

    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{total_width}\" height=\"{BASE_GLYPH_HEIGHT}\">{inner}</svg>")
}

/// Text category controlling which size slider applies.
#[derive(Clone, Copy, PartialEq)]
enum TextCategory {
    Headline,
    ArticleHeadline,
    Body,
}

/// A text block: a single Picture displaying a composed SVG of all glyphs.
struct TextBlock {
    picture: Picture,
    text: String,
    category: TextCategory,
    scale: f64,
    can_justify: bool,
}

struct WireframeState {
    headline_size: i32,
    article_headline_size: i32,
    body_size: i32,
    glyph_spacing: i32,
    justify_body: bool,
    headline_color: gtk4::gdk::RGBA,
    article_headline_color: gtk4::gdk::RGBA,
    body_color: gtk4::gdk::RGBA,
    image_color: gtk4::gdk::RGBA,
    paper_color: gtk4::gdk::RGBA,
    accent_color: gtk4::gdk::RGBA,
    transparency: f64,
    fullscreen: bool,
    css_provider: gtk4::CssProvider,
    window: ApplicationWindow,
    text_blocks: Vec<TextBlock>,
    /// Cached hex colors per category from last texture render.
    cached_headline_hex: String,
    cached_article_headline_hex: String,
    cached_body_hex: String,
}

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    fonts_rs_redacted::register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    Ok(app.run())
}

/// Create a text block as a single Picture widget.
/// `category` selects which size slider applies.
/// `scale` is a multiplier within that category (e.g. 1.4 for masthead title).
/// `can_justify` marks the line as eligible for body justification.
fn create_text_block(text: &str, category: TextCategory, scale: f64, can_justify: bool, all_text_blocks: &mut Vec<TextBlock>) -> Picture {
    let picture = Picture::new();
    picture.set_content_fit(ContentFit::Fill);
    picture.set_can_shrink(true);
    picture.set_halign(Align::Start);
    picture.set_hexpand(false);
    picture.set_vexpand(false);
    picture.set_valign(Align::Start);
    picture.add_css_class("text-line");
    all_text_blocks.push(TextBlock {
        picture: picture.clone(),
        text: text.to_string(),
        category,
        scale,
        can_justify,
    });
    picture
}

/// Create a full-width placeholder image.
fn create_placeholder_image(height: i32) -> Box {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["placeholder-image"])
        .height_request(height)
        .halign(Align::Fill)
        .valign(Align::Start)
        .hexpand(true)
        .build();

    let label = Label::builder()
        .label("PHOTO")
        .css_classes(["placeholder-label"])
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    container.append(&label);
    container
}

/// Create a fixed-width placeholder image for inline use (text wrapping).
fn create_inline_placeholder_image(width: i32, height: i32) -> Box {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["placeholder-image"])
        .width_request(width)
        .height_request(height)
        .halign(Align::Start)
        .valign(Align::Start)
        .hexpand(false)
        .build();

    let label = Label::builder()
        .label("PHOTO")
        .css_classes(["placeholder-label"])
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    container.append(&label);
    container
}

fn build_ui(app: &Application) {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(
        r#"
        window {
            background-color: #2a2a2a;
        }
        .newspaper {
            background-color: #f5f0e8;
            padding: 20px;
            border-radius: 2px;
        }
        .newspaper-rule {
            background-color: #333;
            min-height: 3px;
            margin-top: 4px;
            margin-bottom: 4px;
        }
        .newspaper-rule-thin {
            background-color: #888;
            min-height: 1px;
            margin-top: 2px;
            margin-bottom: 2px;
        }
        .placeholder-image {
            border-radius: 4px;
        }
        .placeholder-label {
            color: rgba(0, 0, 0, 0.3);
            font-size: 18pt;
            font-weight: bold;
            letter-spacing: 4px;
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
        .variant-display {
            color: #ff6600;
            font-size: 12pt;
            font-weight: bold;
            padding: 4px 12px;
            background-color: #2a2a2a;
            border-radius: 4px;
        }
        .hint {
            color: #888;
        }
        .text-line {
            margin-bottom: 0px;
        }
        "#,
    );
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Redacted Newspaper Wireframe")
        .default_width(640)
        .default_height(600)
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
        .spacing(8)
        .margin_start(8)
        .margin_end(8)
        .margin_top(8)
        .margin_bottom(8)
        .build();
    main_box.set_size_request(0, -1);

    // --- Newspaper content (scrollable) ---
    let newspaper = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .css_classes(["newspaper"])
        .halign(Align::Fill)
        .build();
    newspaper.set_size_request(0, -1);

    let newspaper_scroll = gtk4::ScrolledWindow::builder()
        .child(&newspaper)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();

    main_box.append(&newspaper_scroll);

    // --- Active variant ---
    let active_idx = VARIANTS
        .iter()
        .position(|(feat, _)| variant::GLYPH_PREFIX == format!("redacted-{feat}"))
        .unwrap_or(0);
    let (_active_feat, active_label) = VARIANTS[active_idx];

    // --- Collect all text blocks ---
    let mut all_text_blocks: Vec<TextBlock> = Vec::new();

    // --- Masthead ---
    let title = create_text_block("THE DAILY WIREFRAME", TextCategory::Headline, 1.4, false, &mut all_text_blocks);
    title.set_halign(Align::Center);

    let masthead = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .halign(Align::Center)
        .margin_bottom(8)
        .build();
    masthead.append(&title);

    let rule = Box::builder().css_classes(["newspaper-rule"]).build();
    masthead.append(&rule);

    let date_text = "WEDNESDAY  SEPTEMBER 12 2024  LATE EDITION  VOL XLII  NO 42  PRICE $0 50";
    let date_line = create_text_block(date_text, TextCategory::Body, 0.9, false, &mut all_text_blocks);
    date_line.set_halign(Align::Center);
    date_line.set_margin_top(4);
    masthead.append(&date_line);

    let rule2 = Box::builder().css_classes(["newspaper-rule"]).build();
    masthead.append(&rule2);
    newspaper.append(&masthead);

    // --- Hero section ---
    let hero = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    let headline = "BREAKING NEWS STORY UNFOLDS HERE TODAY";
    let headline_pic = create_text_block(headline, TextCategory::Headline, 1.0, false, &mut all_text_blocks);
    hero.append(&headline_pic);

    let subheadline = "DEVELOPING STORY DETAILS EMERGE FROM RELIABLE SOURCES";
    let subheadline_pic = create_text_block(subheadline, TextCategory::ArticleHeadline, 1.0, false, &mut all_text_blocks);
    subheadline_pic.set_margin_top(4);
    hero.append(&subheadline_pic);

    // Hero content: inline image + text (simulates text wrapping)
    let hero_content = Box::builder().orientation(Orientation::Horizontal).spacing(12).margin_top(12).build();

    let hero_image = create_inline_placeholder_image(160, 180);
    hero_content.append(&hero_image);

    let lead_text = Box::builder().orientation(Orientation::Vertical).spacing(4).build();
    let lead_lines = [
        "LOREM IPSUM DOLOR SIT AMET CONSECTETUR ADIPISCING ELIT",
        "SED DO EIUSMOD TEMPOR INCIDIDUNT UT LABORE ET DOLORE",
        "MAGNA ALIQUA  UT ENIM AD MINIM VENIAM QUIS NOSTRUD",
        "EXERCITATION ULLAMCO LABORIS NISI UT ALIQUIP EX EA",
        "COMMODO CONSEQUAT  DUIS AUTE IRURE DOLOR IN REPREHEN",
        "DERIT IN VOLUPTATE VELIT ESSE CILLUM DOLORE EU FUGIAT",
        "NULLA PARIATUR  EXCEPTEUR SINT OCCAECAT CUPIDATAT NON",
    ];
    for (i, line) in lead_lines.iter().enumerate() {
        let can_justify = i < lead_lines.len() - 1;
        let pic = create_text_block(line, TextCategory::Body, 1.0, can_justify, &mut all_text_blocks);
        lead_text.append(&pic);
    }
    hero_content.append(&lead_text);
    hero.append(&hero_content);

    let rule3 = Box::builder().css_classes(["newspaper-rule"]).build();
    hero.append(&rule3);
    newspaper.append(&hero);

    // --- Article grid (3 columns) ---
    let grid = Box::builder().orientation(Orientation::Horizontal).spacing(0).margin_top(8).build();

    // Column 1: full-width image
    {
        let col = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .hexpand(true)
            .margin_start(0)
            .margin_end(12)
            .build();

        let hl = "MARKETS RISE ON POSITIVE DATA";
        let hl_pic = create_text_block(hl, TextCategory::ArticleHeadline, 1.0, false, &mut all_text_blocks);
        col.append(&hl_pic);

        let col_image = create_placeholder_image(120);
        col_image.set_margin_top(6);
        col_image.set_margin_bottom(6);
        col.append(&col_image);

        let body_lines = [
            "LOREM IPSUM DOLOR SIT AMET CONSECTETUR ADIPISCING",
            "ELIT SED DO EIUSMOD TEMPOR INCIDIDUNT UT LABORE",
            "ET DOLORE MAGNA ALIQUA  UT ENIM AD MINIM VENIAM",
            "QUIS NOSTRUD EXERCITATION ULLAMCO LABORIS NISI",
        ];
        for (i, line) in body_lines.iter().enumerate() {
            let can_justify = i < body_lines.len() - 1;
            let pic = create_text_block(line, TextCategory::Body, 1.0, can_justify, &mut all_text_blocks);
            col.append(&pic);
        }
        grid.append(&col);
    }

    // Divider
    {
        let divider = Box::builder()
            .orientation(Orientation::Vertical)
            .css_classes(["newspaper-rule-thin"])
            .width_request(1)
            .build();
        grid.append(&divider);
    }

    // Column 2: inline image with text wrapping
    {
        let col = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .hexpand(true)
            .margin_start(12)
            .margin_end(12)
            .build();

        let hl = "WEATHER UPDATE FOR THE WEEK";
        let hl_pic = create_text_block(hl, TextCategory::ArticleHeadline, 1.0, false, &mut all_text_blocks);
        col.append(&hl_pic);

        let first_lines = [
            "TEMPORIBUS AUTEM QUIBUSDAM ET AUT OFFICIIS DEBITIS",
            "AUT RERUM NECESSITATIBUS SAEPE EVENIET UT ET VOLUPT",
        ];
        for (i, line) in first_lines.iter().enumerate() {
            let can_justify = i < first_lines.len() - 1;
            let pic = create_text_block(line, TextCategory::Body, 1.0, can_justify, &mut all_text_blocks);
            col.append(&pic);
        }

        let inline = Box::builder().orientation(Orientation::Horizontal).spacing(8).margin_top(4).build();

        let inline_img = create_inline_placeholder_image(100, 80);
        inline.append(&inline_img);

        let remaining = Box::builder().orientation(Orientation::Vertical).spacing(4).build();
        let remaining_lines = [
            "ATES REPUDIANDAE SINT ET MOLLITIA NON RECUSANDAE",
            "ITAQUE EARUM RERUM HIC TENETUR A SAPIENTE DELECTUS",
        ];
        for (i, line) in remaining_lines.iter().enumerate() {
            let can_justify = i < remaining_lines.len() - 1;
            let pic = create_text_block(line, TextCategory::Body, 1.0, can_justify, &mut all_text_blocks);
            remaining.append(&pic);
        }
        inline.append(&remaining);
        col.append(&inline);

        let final_line = "UT AUT REICIENDIS VOLUPTATIBUS MAIOR ALIQUAM";
        let pic = create_text_block(final_line, TextCategory::Body, 1.0, false, &mut all_text_blocks);
        col.append(&pic);

        grid.append(&col);
    }

    // Divider
    {
        let divider = Box::builder()
            .orientation(Orientation::Vertical)
            .css_classes(["newspaper-rule-thin"])
            .width_request(1)
            .build();
        grid.append(&divider);
    }

    // Column 3: small inline image at top
    {
        let col = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .hexpand(true)
            .margin_start(12)
            .margin_end(0)
            .build();

        let hl = "LOCAL SPORTS TEAM WINS BIG";
        let hl_pic = create_text_block(hl, TextCategory::ArticleHeadline, 1.0, false, &mut all_text_blocks);
        col.append(&hl_pic);

        let inline_img = create_inline_placeholder_image(80, 60);
        inline_img.set_margin_top(4);
        inline_img.set_margin_bottom(4);
        col.append(&inline_img);

        let body_lines = [
            "SIMILIQUE SUNT IN CULPA QUI OFFICIA DESERUNT",
            "MOLLITIA ANIMI  ID EST LABORUM ET DOLORUM FUGA",
            "ET HARUM QUIDEM RERUM FACILIS EST ET EXPEDITA",
            "DISTINCTIO  NAM LIBERO TEMPORIS CUM SOLUTA NOBIS",
        ];
        for (i, line) in body_lines.iter().enumerate() {
            let can_justify = i < body_lines.len() - 1;
            let pic = create_text_block(line, TextCategory::Body, 1.0, can_justify, &mut all_text_blocks);
            col.append(&pic);
        }
        grid.append(&col);
    }

    newspaper.append(&grid);

    // --- Footer ---
    let footer = Box::builder().orientation(Orientation::Vertical).spacing(4).margin_top(12).build();

    let rule4 = Box::builder().css_classes(["newspaper-rule"]).build();
    footer.append(&rule4);

    let ticker = "DOW +1 2%   NASDAQ +0 8%   S&P +1 5%   GOLD $2000   OIL $80   BTC $65000";
    let ticker_pic = create_text_block(ticker, TextCategory::Body, 0.9, false, &mut all_text_blocks);
    ticker_pic.set_halign(Align::Center);
    ticker_pic.set_margin_top(4);
    footer.append(&ticker_pic);

    let folio = "PAGE 1   THE DAILY WIREFRAME   SEPTEMBER 2024";
    let folio_pic = create_text_block(folio, TextCategory::Body, 0.75, false, &mut all_text_blocks);
    folio_pic.set_halign(Align::Center);
    folio_pic.set_margin_top(4);
    footer.append(&folio_pic);

    newspaper.append(&footer);

    // --- Controls panel ---
    let controls = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .css_classes(["control-panel"])
        .build();

    // Headline size slider
    let headline_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();
    let headline_label = Label::builder().label("Headline:").css_classes(["variant-label"]).build();
    headline_row.append(&headline_label);
    let headline_scale = gtk4::Scale::with_range(Orientation::Horizontal, 16.0, 80.0, 2.0);
    headline_scale.set_value(40.0);
    headline_scale.set_draw_value(false);
    headline_scale.set_hexpand(true);
    for mark in [16, 32, 48, 64, 80] {
        headline_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    headline_row.append(&headline_scale);
    controls.append(&headline_row);

    // Article headline size slider
    let article_headline_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();
    let article_headline_label = Label::builder().label("Article Headline:").css_classes(["variant-label"]).build();
    article_headline_row.append(&article_headline_label);
    let article_headline_scale = gtk4::Scale::with_range(Orientation::Horizontal, 8.0, 48.0, 2.0);
    article_headline_scale.set_value(24.0);
    article_headline_scale.set_draw_value(false);
    article_headline_scale.set_hexpand(true);
    for mark in [8, 16, 24, 32, 48] {
        article_headline_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    article_headline_row.append(&article_headline_scale);
    controls.append(&article_headline_row);

    // Body size slider + justify switch
    let body_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();
    let body_label = Label::builder().label("Body:").css_classes(["variant-label"]).build();
    body_row.append(&body_label);
    let body_scale = gtk4::Scale::with_range(Orientation::Horizontal, 6.0, 32.0, 1.0);
    body_scale.set_value(16.0);
    body_scale.set_draw_value(false);
    body_scale.set_hexpand(true);
    for mark in [6, 12, 16, 24, 32] {
        body_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    body_row.append(&body_scale);
    let justify_switch = gtk4::Switch::new();
    justify_switch.set_active(true);
    justify_switch.set_tooltip_text(Some("Justify body text"));
    body_row.append(&justify_switch);
    controls.append(&body_row);

    // Spacing slider
    let spacing_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();
    let spacing_label = Label::builder().label("Spacing:").css_classes(["variant-label"]).build();
    spacing_row.append(&spacing_label);
    let spacing_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 8.0, 1.0);
    spacing_scale.set_value(0.0);
    spacing_scale.set_draw_value(false);
    spacing_scale.set_hexpand(true);
    for mark in [0, 2, 4, 8] {
        spacing_scale.add_mark(mark as f64, gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    spacing_row.append(&spacing_scale);
    controls.append(&spacing_row);

    // Variant display (readonly)
    let variant_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();
    let variant_label = Label::builder().label("Variant:").css_classes(["variant-label"]).build();
    variant_row.append(&variant_label);
    let variant_display = Label::builder().label(active_label).css_classes(["variant-display"]).hexpand(true).build();
    variant_row.append(&variant_display);
    controls.append(&variant_row);

    // Color pickers
    let color_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).build();
    let color_dialog = gtk4::ColorDialog::new();

    let headline_btn = ColorDialogButton::builder().tooltip_text("Headline text").build();
    headline_btn.set_dialog(&color_dialog);
    headline_btn.set_rgba(&gtk4::gdk::RGBA::new(0.1, 0.1, 0.1, 1.0));
    color_row.append(&headline_btn);

    let article_headline_btn = ColorDialogButton::builder().tooltip_text("Article headline text").build();
    article_headline_btn.set_dialog(&color_dialog);
    article_headline_btn.set_rgba(&gtk4::gdk::RGBA::new(0.2, 0.2, 0.2, 1.0));
    color_row.append(&article_headline_btn);

    let body_btn = ColorDialogButton::builder().tooltip_text("Body text").build();
    body_btn.set_dialog(&color_dialog);
    body_btn.set_rgba(&gtk4::gdk::RGBA::new(0.3, 0.3, 0.3, 1.0));
    color_row.append(&body_btn);

    let image_btn = ColorDialogButton::builder().tooltip_text("Image placeholders").build();
    image_btn.set_dialog(&color_dialog);
    image_btn.set_rgba(&gtk4::gdk::RGBA::new(0.82, 0.82, 0.82, 1.0));
    color_row.append(&image_btn);

    let paper_btn = ColorDialogButton::builder().tooltip_text("Paper background").build();
    paper_btn.set_dialog(&color_dialog);
    paper_btn.set_rgba(&gtk4::gdk::RGBA::new(0.96, 0.94, 0.91, 1.0));
    color_row.append(&paper_btn);

    let accent_btn = ColorDialogButton::builder().tooltip_text("Accent (rules)").build();
    accent_btn.set_dialog(&color_dialog);
    accent_btn.set_rgba(&gtk4::gdk::RGBA::new(0.2, 0.2, 0.2, 1.0));
    color_row.append(&accent_btn);

    let opacity_scale = gtk4::Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 5.0);
    opacity_scale.set_value(100.0);
    opacity_scale.set_draw_value(false);
    opacity_scale.set_hexpand(true);
    opacity_scale.set_tooltip_text(Some("Transparency (fullscreen only)"));
    opacity_scale.add_mark(0.0, gtk4::PositionType::Bottom, Some("0%"));
    opacity_scale.add_mark(50.0, gtk4::PositionType::Bottom, Some("50%"));
    opacity_scale.add_mark(100.0, gtk4::PositionType::Bottom, Some("100%"));
    color_row.append(&opacity_scale);

    controls.append(&color_row);

    // Hint label
    let hint = Label::builder()
        .label("F5 = Fullscreen  |  ESC = Exit fullscreen")
        .css_classes(["hint"])
        .build();
    controls.append(&hint);

    let controls_scroll = gtk4::ScrolledWindow::builder()
        .child(&controls)
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Never)
        .build();
    controls_scroll.set_propagate_natural_height(true);

    main_box.append(&controls_scroll);

    window.set_child(Some(&main_box));
    window.present();

    // --- Dynamic CSS ---
    let dynamic_provider = gtk4::CssProvider::new();
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &dynamic_provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    let state = Rc::new(RefCell::new(WireframeState {
        headline_size: 40,
        article_headline_size: 24,
        body_size: 16,
        glyph_spacing: 0,
        justify_body: true,
        headline_color: gtk4::gdk::RGBA::new(0.1, 0.1, 0.1, 1.0),
        article_headline_color: gtk4::gdk::RGBA::new(0.2, 0.2, 0.2, 1.0),
        body_color: gtk4::gdk::RGBA::new(0.3, 0.3, 0.3, 1.0),
        image_color: gtk4::gdk::RGBA::new(0.82, 0.82, 0.82, 1.0),
        paper_color: gtk4::gdk::RGBA::new(0.96, 0.94, 0.91, 1.0),
        accent_color: gtk4::gdk::RGBA::new(0.2, 0.2, 0.2, 1.0),
        transparency: 1.0,
        fullscreen: false,
        css_provider: dynamic_provider,
        window: window.clone(),
        text_blocks: all_text_blocks,
        cached_headline_hex: String::new(),
        cached_article_headline_hex: String::new(),
        cached_body_hex: String::new(),
    }));

    update_dynamic_css(&state);
    render_textures(&state);
    update_justify(&state);
    update_sizes(&state);

    // --- Fullscreen toggle ---
    {
        let state = state.clone();
        let window_clone = window.clone();
        let controls = controls_scroll.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            let mut s = state.borrow_mut();
            match keyval {
                gtk4::gdk::Key::F5 => {
                    if s.fullscreen {
                        s.fullscreen = false;
                        drop(s);
                        window_clone.set_decorated(true);
                        window_clone.unmaximize();
                        controls.set_visible(true);
                        update_dynamic_css(&state);
                    } else {
                        s.fullscreen = true;
                        drop(s);
                        window_clone.set_decorated(false);
                        window_clone.maximize();
                        controls.set_visible(false);
                        update_dynamic_css(&state);
                    }
                    glib::Propagation::Stop
                }
                gtk4::gdk::Key::Escape => {
                    if s.fullscreen {
                        s.fullscreen = false;
                        drop(s);
                        window_clone.set_decorated(true);
                        window_clone.unmaximize();
                        controls.set_visible(true);
                        update_dynamic_css(&state);
                    }
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
        window.add_controller(key_controller);
    }

    // --- Event handlers ---
    {
        let state = state.clone();
        headline_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.headline_size = scale.value() as i32;
            drop(s);
            update_sizes(&state);
        });
    }

    {
        let state = state.clone();
        article_headline_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.article_headline_size = scale.value() as i32;
            drop(s);
            update_sizes(&state);
        });
    }

    {
        let state = state.clone();
        body_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.body_size = scale.value() as i32;
            drop(s);
            update_sizes(&state);
        });
    }

    {
        let state = state.clone();
        spacing_scale.connect_value_changed(move |scale| {
            let mut s = state.borrow_mut();
            s.glyph_spacing = scale.value() as i32;
            drop(s);
            update_dynamic_css(&state);
        });
    }

    {
        let state = state.clone();
        justify_switch.connect_notify_local(Some("active"), move |sw, _| {
            let mut s = state.borrow_mut();
            s.justify_body = sw.is_active();
            drop(s);
            update_justify(&state);
            update_sizes(&state);
        });
    }

    {
        let state = state.clone();
        headline_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.headline_color = btn.rgba();
            drop(s);
            render_textures(&state);
        });
    }

    {
        let state = state.clone();
        article_headline_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.article_headline_color = btn.rgba();
            drop(s);
            render_textures(&state);
        });
    }

    {
        let state = state.clone();
        body_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.body_color = btn.rgba();
            drop(s);
            render_textures(&state);
        });
    }

    {
        let state = state.clone();
        image_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.image_color = btn.rgba();
            drop(s);
            update_dynamic_css(&state);
        });
    }

    {
        let state = state.clone();
        paper_btn.connect_notify_local(Some("rgba"), move |btn, _| {
            let mut s = state.borrow_mut();
            s.paper_color = btn.rgba();
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

/// Get the base size for a text category.
fn category_base_size(s: &WireframeState, category: TextCategory) -> i32 {
    match category {
        TextCategory::Headline => s.headline_size,
        TextCategory::ArticleHeadline => s.article_headline_size,
        TextCategory::Body => s.body_size,
    }
}

/// Get the color for a text category.
fn category_color(s: &WireframeState, category: TextCategory) -> &gtk4::gdk::RGBA {
    match category {
        TextCategory::Headline => &s.headline_color,
        TextCategory::ArticleHeadline => &s.article_headline_color,
        TextCategory::Body => &s.body_color,
    }
}

/// Get the cached hex string for a text category (mutable).
fn category_cached_hex_mut(s: &mut WireframeState, category: TextCategory) -> &mut String {
    match category {
        TextCategory::Headline => &mut s.cached_headline_hex,
        TextCategory::ArticleHeadline => &mut s.cached_article_headline_hex,
        TextCategory::Body => &mut s.cached_body_hex,
    }
}

/// Update halign/hexpand/content-fit on justifiable text blocks based on justify setting.
/// Non-justifiable blocks keep their caller-set halign (e.g. Center for title).
fn update_justify(state: &Rc<RefCell<WireframeState>>) {
    let s = state.borrow();
    let justify_body = s.justify_body;
    for tb in &s.text_blocks {
        if tb.can_justify {
            tb.picture.set_halign(if justify_body { Align::Fill } else { Align::Start });
            tb.picture.set_hexpand(justify_body);
            tb.picture.set_content_fit(if justify_body { ContentFit::Fill } else { ContentFit::Contain });
        }
    }
}

/// Update pixel sizes on all text block Pictures (cheap, no texture recreation).
fn update_sizes(state: &Rc<RefCell<WireframeState>>) {
    let s = state.borrow();
    let glyph_aspect = BASE_GLYPH_WIDTH as f64 / BASE_GLYPH_HEIGHT as f64;
    let justify_body = s.justify_body;
    for tb in &s.text_blocks {
        let base_size = category_base_size(&s, tb.category);
        let height = (base_size as f64 * tb.scale) as i32;
        let effective = tb.can_justify && justify_body;
        if effective {
            tb.picture.set_size_request(0, height);
        } else {
            let char_count = tb.text.chars().count() as f64;
            let width = (char_count * base_size as f64 * tb.scale * glyph_aspect) as i32;
            tb.picture.set_size_request(width, height);
        }
    }
}

/// Rebuild glyph textures for categories whose color changed.
fn render_textures(state: &Rc<RefCell<WireframeState>>) {
    let mut s = state.borrow_mut();
    for category in [TextCategory::Headline, TextCategory::ArticleHeadline, TextCategory::Body] {
        let hex_color = rgba_to_hex(category_color(&s, category));
        let cached = category_cached_hex_mut(&mut s, category);
        if hex_color == *cached {
            continue;
        }
        *cached = hex_color.clone();
        for tb in &s.text_blocks {
            if tb.category != category {
                continue;
            }
            let svg = compose_text_svg(&tb.text, &hex_color);
            let bytes = glib::Bytes::from(svg.as_bytes());
            if let Ok(texture) = gtk4::gdk::Texture::from_bytes(&bytes) {
                tb.picture.set_paintable(Some(&texture));
            }
        }
    }
}

/// Update the dynamic CSS with current colors and transparency.
fn update_dynamic_css(state: &Rc<RefCell<WireframeState>>) {
    let s = state.borrow();
    let alpha = if s.fullscreen { s.transparency } else { 1.0 };

    let accent_hex = rgba_to_hex(&s.accent_color);
    let image_hex = rgba_to_hex(&s.image_color);

    let window_bg = if s.fullscreen {
        "transparent".to_string()
    } else {
        format!(
            "rgb({},{},{})",
            (s.paper_color.red() * 255.0) as u8,
            (s.paper_color.green() * 255.0) as u8,
            (s.paper_color.blue() * 255.0) as u8,
        )
    };

    let paper_alpha = format!(
        "rgba({},{},{},{:.3})",
        (s.paper_color.red() * 255.0) as u8,
        (s.paper_color.green() * 255.0) as u8,
        (s.paper_color.blue() * 255.0) as u8,
        alpha,
    );

    let spacing = s.glyph_spacing;

    let css = format!(
        "window {{ background-color: {window_bg}; }}\n\
         .newspaper {{ background-color: {paper_alpha}; }}\n\
         .newspaper-rule {{ background-color: {accent_hex}; }}\n\
         .newspaper-rule-thin {{ background-color: {accent_hex}; }}\n\
         .text-line {{ margin-bottom: {spacing}px; }}\n\
         .placeholder-image {{\n\
         \x20 background-color: {image_hex};\n\
         \x20 background-image:\n\
         \x20\x20 linear-gradient(45deg, rgba(0,0,0,0.08) 25%, transparent 25%, transparent 75%, rgba(0,0,0,0.08) 75%),\n\
         \x20\x20 linear-gradient(45deg, rgba(0,0,0,0.08) 25%, transparent 25%, transparent 75%, rgba(0,0,0,0.08) 75%);\n\
         \x20 background-size: 16px 16px;\n\
         \x20 background-position: 0 0, 8px 8px;\n\
         }}\n",
    );

    s.css_provider.load_from_string(&css);

    if s.fullscreen
        && let Some(surface) = s.window.surface()
    {
        surface.set_opaque_region(None);
    }
}
