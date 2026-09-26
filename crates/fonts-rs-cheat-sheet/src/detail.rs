//! Detail sidebar for the generic cheat sheet.

use crate::family::CheatSheetFamily;
use fonts_rs_model::CodePoint;
use gtk4::Align;
use gtk4::Box;
use gtk4::Button;
use gtk4::Entry;
use gtk4::FlowBox;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::ScrolledWindow;
use gtk4::TextView;
use gtk4::WrapMode;
use gtk4::prelude::*;

/// Populate the sidebar content with details for the given glyph.
pub(super) fn show_detail<F: CheatSheetFamily>(content: &Box, name: &F::Name, codepoint: CodePoint, search_entry: &Entry) {
    while let Some(child) = content.first_child() {
        content.remove(&child);
    }

    let display_name = F::display_name(name);
    let subtitle = F::subtitle(name);
    let keywords = F::keywords(name);
    let categories = F::categories(name);

    // Display name as title.
    let title_label = Label::builder()
        .label(&display_name)
        .css_classes(["sidebar-title"])
        .halign(Align::Start)
        .selectable(true)
        .wrap(true)
        .build();
    content.append(&title_label);

    // Source/family as subtitle.
    let subtitle_label = Label::builder().label(&subtitle).css_classes(["sidebar-subtitle"]).halign(Align::Start).build();
    content.append(&subtitle_label);

    // Name section (CSS class name for icon fonts, glyph name otherwise).
    let name_heading = Label::builder()
        .label(F::NAME_FIELD_LABEL)
        .css_classes(["heading"])
        .halign(Align::Start)
        .margin_top(8)
        .build();
    content.append(&name_heading);

    let name_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).halign(Align::Fill).build();
    let name_val = Label::builder()
        .label(&display_name)
        .css_classes(["monospace", "copyable"])
        .halign(Align::Fill)
        .selectable(true)
        .wrap(true)
        .max_width_chars(20)
        .build();
    let name_copy = Button::builder()
        .icon_name("edit-copy-symbolic")
        .tooltip_text(format!("Copy {}", F::NAME_FIELD_LABEL.to_lowercase()))
        .build();
    let name_for_copy = display_name.clone();
    name_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&name_for_copy);
        }
    });
    name_row.append(&name_val);
    name_row.append(&name_copy);
    content.append(&name_row);

    // Codepoint section.
    let cp_heading = Label::builder()
        .label("Codepoint")
        .css_classes(["heading"])
        .halign(Align::Start)
        .margin_top(8)
        .build();
    content.append(&cp_heading);

    // Codepoint glyph (very large, own line). Families with icon-theme
    // glyphs (e.g. emoji, fontless SVG sets) render the registered SVG -
    // their codepoint is a PUA handle that the system font fallback
    // would draw as an unrelated glyph. Others render the char as text.
    let cp_widget: gtk4::Widget = if let Some(icon_name) = F::glyph_icon_name(name) {
        gtk4::Image::builder()
            .icon_name(&icon_name)
            .pixel_size(96)
            .halign(Align::Center)
            .build()
            .upcast()
    } else {
        let mut cp_char_classes = vec!["sidebar-codepoint-char", "copyable"];
        if let Some(class) = F::GLYPH_CSS_CLASS {
            cp_char_classes.insert(0, class);
        }
        Label::builder()
            .label(codepoint.as_char().to_string())
            .css_classes(cp_char_classes)
            .halign(Align::Center)
            .selectable(true)
            .wrap(true)
            .width_request(200)
            .build()
            .upcast()
    };
    content.append(&cp_widget);

    // Codepoint hex string (own line).
    let cp_hex_label = Label::builder()
        .label(format!("U+{:04X}", codepoint.as_char() as u32))
        .css_classes(["monospace", "sidebar-codepoint-hex", "copyable"])
        .halign(Align::Center)
        .selectable(true)
        .build();
    content.append(&cp_hex_label);

    // Copy character button.
    let cp_copy = Button::builder()
        .icon_name("edit-copy-symbolic")
        .label("Copy character")
        .tooltip_text("Copy character to clipboard")
        .halign(Align::Center)
        .build();
    let cp_char = codepoint.as_char().to_string();
    cp_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&cp_char);
        }
    });
    content.append(&cp_copy);

    // Alias info.
    if let Some(canonical) = F::alias_display(name) {
        let alias_label = Label::builder()
            .label(format!("Alias of: {}", canonical))
            .css_classes(["dim-label"])
            .halign(Align::Start)
            .margin_top(4)
            .wrap(true)
            .max_width_chars(24)
            .build();
        content.append(&alias_label);
    }

    // Keywords (clickable buttons).
    if !keywords.is_empty() {
        let kw_header = Label::builder()
            .label("Keywords")
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_top(12)
            .build();
        content.append(&kw_header);

        let kw_box = FlowBox::builder()
            .column_spacing(4)
            .row_spacing(4)
            .halign(Align::Start)
            .selection_mode(gtk4::SelectionMode::None)
            .build();
        for kw_label in keywords.iter() {
            let btn = Button::builder().label(kw_label).css_classes(["kw-btn"]).build();
            let search_entry_ref = search_entry.clone();
            let kw_label = kw_label.clone();
            btn.connect_clicked(move |_| {
                search_entry_ref.set_text(&kw_label);
            });
            kw_box.insert(&btn, -1);
        }
        content.append(&kw_box);
    }

    // Categories (clickable buttons).
    if !categories.is_empty() {
        let cat_header = Label::builder()
            .label("Categories")
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_top(12)
            .build();
        content.append(&cat_header);

        let cat_box = FlowBox::builder()
            .column_spacing(4)
            .row_spacing(4)
            .halign(Align::Start)
            .selection_mode(gtk4::SelectionMode::None)
            .build();
        for cat_label in categories.iter() {
            let btn = Button::builder().label(cat_label).css_classes(["cat-btn"]).build();
            let search_entry_ref = search_entry.clone();
            let cat_label = cat_label.clone();
            btn.connect_clicked(move |_| {
                search_entry_ref.set_text(&cat_label);
            });
            cat_box.insert(&btn, -1);
        }
        content.append(&cat_box);
    }

    // SVG section.
    if let Some(svg_xml) = F::glyph_svg(name) {
        let svg_heading = Label::builder()
            .label("SVG")
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_top(12)
            .build();
        content.append(&svg_heading);

        let text_view = TextView::builder().editable(false).css_classes(["monospace"]).wrap_mode(WrapMode::Char).build();
        text_view.buffer().set_text(&svg_xml);

        let scrolled = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .min_content_height(150)
            .max_content_height(300)
            .child(&text_view)
            .build();
        content.append(&scrolled);

        let svg_copy = Button::builder()
            .icon_name("edit-copy-symbolic")
            .label("Copy SVG")
            .tooltip_text("Copy SVG XML to clipboard")
            .halign(Align::Start)
            .build();
        let svg_xml_for_copy = svg_xml;
        svg_copy.connect_clicked(move |_| {
            if let Some(display) = gtk4::gdk::Display::default() {
                display.clipboard().set_text(&svg_xml_for_copy);
            }
        });
        content.append(&svg_copy);
    }
}
