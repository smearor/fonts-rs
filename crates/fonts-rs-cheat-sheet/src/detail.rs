//! Detail sidebar for the generic cheat sheet.

use crate::app::icon_size_scale;
use crate::detail_context::DetailContext;
use crate::family::CheatSheetFamily;
use crate::xml_highlight;
use fonts_rs_model::CodePoint;
use gtk4::Align;
use gtk4::Box;
use gtk4::Button;
use gtk4::FlowBox;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::ScrolledWindow;
use gtk4::TextView;
use gtk4::WrapMode;
use gtk4::pango::AttrFloat;
use gtk4::pango::AttrList;
use gtk4::prelude::*;
use libadwaita as adw;

/// Populate the sidebar content with details for the given glyph.
pub(super) fn show_detail<F: CheatSheetFamily>(ctx: &DetailContext, name: &F::Name, codepoint: CodePoint) {
    let DetailContext {
        content,
        search_entry,
        tone_picker,
        toasts,
        category_split,
        detail_icon_size,
        ..
    } = ctx;
    let detail_size = detail_icon_size.get();

    // Maps the log2 slider value back to a whole-pixel icon size.
    let log2_to_px = |value: f64| 2.0_f64.powf(value).round();

    // Scales the codepoint preview label via a Pango size attribute
    // relative to its 96px CSS base (`.sidebar-codepoint-char`).
    let scale_attrs = |px: f64| {
        let attrs = AttrList::new();
        attrs.insert(AttrFloat::new_scale(px / 96.0));
        attrs
    };
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

    // Ordering group label (e.g. the emoji picker group) below the subtitle.
    if let Some(group) = F::ordering().and_then(|o| o.group(name.as_ref())) {
        let group_label = Label::builder()
            .label(group.as_str())
            .css_classes(["sidebar-subtitle", "dim-label"])
            .halign(Align::Start)
            .build();
        content.append(&group_label);
    }

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
        .css_classes(["monospace"])
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
    let toasts_for_name = toasts.clone();
    name_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&name_for_copy);
            toasts_for_name.add_toast(adw::Toast::new("Copied name"));
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
            .pixel_size(detail_size.round() as i32)
            .halign(Align::Center)
            .build()
            .upcast()
    } else {
        let mut cp_char_classes = vec!["sidebar-codepoint-char"];
        if let Some(class) = F::GLYPH_CSS_CLASS {
            cp_char_classes.insert(0, class);
        }
        let cp_label = Label::builder()
            .label(codepoint.as_char().to_string())
            .css_classes(cp_char_classes)
            .halign(Align::Center)
            .selectable(true)
            .wrap(true)
            .width_request(200)
            .build();
        cp_label.set_attributes(Some(&scale_attrs(detail_size)));
        cp_label.upcast()
    };

    // Preview size slider (log2 scale like the grid slider) - the
    // chosen size persists across detail views via the context. It
    // sits *above* the preview: resizing the icon grows it downward
    // instead of shifting the slider up and down.
    let detail_size_box = Box::builder().orientation(Orientation::Horizontal).spacing(8).halign(Align::Center).build();
    let detail_size_value = Label::builder()
        .label(format!("{detail_size:.0}px"))
        .css_classes(["monospace"])
        .build();
    let detail_slider = icon_size_scale(detail_size, 260);
    let cp_widget_for_size = cp_widget.clone();
    let detail_size_value_ref = detail_size_value.clone();
    let detail_icon_size_ref = detail_icon_size.clone();
    detail_slider.connect_value_changed(move |slider| {
        let px = log2_to_px(slider.value());
        if let Some(image) = cp_widget_for_size.downcast_ref::<gtk4::Image>() {
            image.set_pixel_size(px as i32);
        } else if let Some(label) = cp_widget_for_size.downcast_ref::<Label>() {
            label.set_attributes(Some(&scale_attrs(px)));
        }
        detail_size_value_ref.set_label(&format!("{px:.0}px"));
        detail_icon_size_ref.set(px);
    });
    detail_size_box.append(&detail_slider);
    detail_size_box.append(&detail_size_value);
    content.append(&detail_size_box);
    content.append(&cp_widget);

    // Codepoint hex string (own line).
    let cp_hex = format!("U+{:04X}", codepoint.as_char() as u32);
    let cp_hex_row = Box::builder().orientation(Orientation::Horizontal).spacing(8).halign(Align::Center).build();
    let cp_hex_label = Label::builder()
        .label(&cp_hex)
        .css_classes(["monospace", "sidebar-codepoint-hex"])
        .halign(Align::Center)
        .selectable(true)
        .build();
    let cp_hex_copy = Button::builder().icon_name("edit-copy-symbolic").tooltip_text("Copy codepoint").build();
    let toasts_for_hex = toasts.clone();
    cp_hex_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&cp_hex);
            toasts_for_hex.add_toast(adw::Toast::new("Copied codepoint"));
        }
    });
    cp_hex_row.append(&cp_hex_label);
    cp_hex_row.append(&cp_hex_copy);
    content.append(&cp_hex_row);

    // Copy character button.
    let cp_copy = Button::builder()
        .icon_name("edit-copy-symbolic")
        .label("Copy character")
        .tooltip_text("Copy character to clipboard")
        .halign(Align::Center)
        .build();
    let cp_char = codepoint.as_char().to_string();
    let toasts_for_char = toasts.clone();
    cp_copy.connect_clicked(move |_| {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&cp_char);
            toasts_for_char.add_toast(adw::Toast::new("Copied character"));
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
            let split_ref = category_split.clone();
            btn.connect_clicked(move |_| {
                search_entry_ref.set_text(&cat_label);
                // Reveal the axis the user just filtered by.
                if let Some(split) = &split_ref {
                    split.set_show_sidebar(true);
                }
            });
            cat_box.insert(&btn, -1);
        }
        content.append(&cat_box);
    }

    // Variant navigation via the ordering contract: a variant links
    // back to its base; a base with alternates lists them as clickable
    // glyph cells. Replaces the popover interaction - variants stay
    // reachable from the sidebar while the grid stays collapsed.
    if let Some(ord) = F::ordering() {
        if let Some(base) = ord.base_of(name.as_ref())
            && let (Some(base_name), Some(base_cp)) = (F::parse_name(base), F::codepoint(base))
        {
            let base_header = Label::builder()
                .label("Variant of")
                .css_classes(["heading"])
                .halign(Align::Start)
                .margin_top(12)
                .build();
            content.append(&base_header);
            let btn = Button::builder()
                .label(base)
                .css_classes(["cat-btn"])
                .halign(Align::Start)
                .tooltip_text(format!("Open {base}"))
                .build();
            let ctx_ref = ctx.clone();
            btn.connect_clicked(move |_| show_detail::<F>(&ctx_ref, &base_name, base_cp));
            content.append(&btn);
        }

        let variants = ord.alternates(name.as_ref());
        if !variants.is_empty() {
            let var_header = Label::builder()
                .label("Variants")
                .css_classes(["heading"])
                .halign(Align::Start)
                .margin_top(12)
                .build();
            content.append(&var_header);
            let var_box = FlowBox::builder()
                .column_spacing(6)
                .row_spacing(6)
                .halign(Align::Start)
                .selection_mode(gtk4::SelectionMode::None)
                .build();
            for &variant in variants {
                let (Some(variant_name), Some(variant_cp)) = (F::parse_name(variant), F::codepoint(variant)) else {
                    continue;
                };
                let icon: gtk4::Widget = if let Some(icon_name) = F::glyph_icon_name(&variant_name) {
                    gtk4::Image::builder()
                        .icon_name(&icon_name)
                        .pixel_size(32)
                        .halign(Align::Center)
                        .build()
                        .upcast()
                } else {
                    // The variant char needs the family font class,
                    // otherwise Pango falls back to any installed font
                    // covering the PUA codepoint and draws an
                    // unrelated glyph.
                    let mut variant_classes = vec!["cheat-icon-label"];
                    if let Some(class) = F::GLYPH_CSS_CLASS {
                        variant_classes.insert(0, class);
                    }
                    Label::builder()
                        .label(variant_cp.as_char().to_string())
                        .css_classes(variant_classes)
                        .halign(Align::Center)
                        .build()
                        .upcast()
                };
                let cell = Box::builder().orientation(Orientation::Vertical).spacing(2).halign(Align::Center).build();
                cell.append(&icon);
                cell.append(
                    &Label::builder()
                        .label(variant)
                        .css_classes(["caption", "dim-label"])
                        .max_width_chars(12)
                        .wrap(true)
                        .halign(Align::Center)
                        .build(),
                );
                let btn = Button::builder().child(&cell).css_classes(["flat"]).tooltip_text(variant).build();
                let ctx_ref = ctx.clone();
                btn.connect_clicked(move |_| show_detail::<F>(&ctx_ref, &variant_name, variant_cp));
                var_box.insert(&btn, -1);
            }
            content.append(&var_box);
        }

        // Default-tone picking: a variant becomes the shared default
        // of its tone group ("set skin tone once, apply everywhere").
        // The base glyph has no alternates position - it only offers
        // the reset while a non-default index is stored.
        if let Some(group) = ord.tone_group(name.as_ref()) {
            let picked_index = ord
                .base_of(name.as_ref())
                .and_then(|base| ord.alternates(base).iter().position(|v| *v == name.as_ref()));
            let stored = tone_picker.selected(group);
            if picked_index.is_some() || stored.is_some() {
                let is_default = picked_index == stored;
                let label = if is_default { "Reset default tone" } else { "Set as default tone" };
                let tone_btn = Button::builder()
                    .label(label)
                    .css_classes(["cat-btn"])
                    .halign(Align::Start)
                    .margin_top(8)
                    .tooltip_text("Apply this tone to every glyph in the group")
                    .build();
                let ctx_ref = ctx.clone();
                let name_for_tone = name.clone();
                tone_btn.connect_clicked(move |_| {
                    match (picked_index, is_default) {
                        (Some(index), false) => ctx_ref.tone_picker.select(group, index),
                        _ => ctx_ref.tone_picker.reset(group),
                    }
                    // Re-render the sidebar so the button label
                    // reflects the new selection state.
                    show_detail::<F>(&ctx_ref, &name_for_tone, codepoint);
                });
                content.append(&tone_btn);
            }
        }
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

        let text_view = TextView::builder()
            .editable(false)
            .css_classes(["monospace"])
            .wrap_mode(WrapMode::Char)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(10)
            .margin_end(10)
            .build();
        let svg_buffer = text_view.buffer();
        svg_buffer.set_text(&svg_xml);
        xml_highlight::apply(&svg_buffer, &svg_xml, adw::StyleManager::default().is_dark());

        // `.card` separates the code block from the rest of the
        // sidebar; propagate_natural_height lets it grow with the
        // content up to the cap instead of claiming fixed space.
        let scrolled = ScrolledWindow::builder()
            .css_classes(["card"])
            .hexpand(true)
            .vexpand(true)
            .min_content_height(80)
            .max_content_height(300)
            .propagate_natural_height(true)
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
        let toasts_for_svg = toasts.clone();
        svg_copy.connect_clicked(move |_| {
            if let Some(display) = gtk4::gdk::Display::default() {
                display.clipboard().set_text(&svg_xml_for_copy);
                toasts_for_svg.add_toast(adw::Toast::new("Copied SVG"));
            }
        });
        content.append(&svg_copy);
    }
}
