//! Application shell - window, paned layout, action wiring.

use crate::app::grid::GlyphGrid;
use crate::app::sidebar::SelectionSidebar;
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use miette::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub mod grid;
pub mod sidebar;

/// Application ID for the icon set builder.
const APP_ID: &str = "io.smearor.fonts_rs.iconset_builder";

/// The icon-capable sheet families feeding the combined grid - kept in
/// `ICON_SOURCES` order so grid cells map 1:1 to spec sources.
fn families() -> Vec<Rc<dyn fonts_rs_cheat_sheet::DynCheatSheetFamily>> {
    use fonts_rs_cheat_sheet::FamilyAdapter;
    vec![
        Rc::new(FamilyAdapter::<fonts_rs_eva_cheat_sheet::EvaSheet>::new()),
        Rc::new(FamilyAdapter::<fonts_rs_fluent_cheat_sheet::FluentSheet>::new()),
        Rc::new(FamilyAdapter::<fonts_rs_heroicons_cheat_sheet::HeroiconsSheet>::new()),
        Rc::new(FamilyAdapter::<fonts_rs_line_awesome_cheat_sheet::LineAwesomeSheet>::new()),
        Rc::new(FamilyAdapter::<fonts_rs_lucide_cheat_sheet::LucideSheet>::new()),
        Rc::new(FamilyAdapter::<fonts_rs_tabler_cheat_sheet::TablerSheet>::new()),
    ]
}

/// Equal margins on all four sides - gtk4 removed `set_margin_all`.
pub(crate) fn margins(widget: &impl IsA<gtk4::Widget>, margin: i32) {
    widget.set_margin_top(margin);
    widget.set_margin_bottom(margin);
    widget.set_margin_start(margin);
    widget.set_margin_end(margin);
}

/// Runs the builder application.
pub fn run() -> Result<glib::ExitCode> {
    let application = adw::Application::builder().application_id(APP_ID).build();
    application.connect_activate(|application| {
        if let Err(error) = activate(application) {
            eprintln!("iconset-builder: activation failed: {error:?}");
        }
    });
    Ok(application.run())
}

/// Builds the window: grid pane (search + chips + FlowBox) and the
/// selection sidebar (settings, picks, collisions, actions).
fn activate(application: &adw::Application) -> Result<()> {
    let families = families();
    for family in &families {
        family.init()?;
    }

    // Font-family cells render the PUA codepoint as text - the family's
    // `glyph_css_class` binds the font, `.builder-glyph` sizes it.
    let css = gtk4::CssProvider::new();
    css.load_from_string(".builder-glyph { font-size: 28px; min-height: 38px; }");
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &css, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let selection = Rc::new(RefCell::new(fonts_rs_iconset_builder::IconSelection::new()));
    let settings = Rc::new(RefCell::new(fonts_rs_iconset_builder::BuilderSettings::default()));

    let sidebar = SelectionSidebar::new(selection.clone(), settings.clone());
    let grid = GlyphGrid::new(families, selection.clone(), {
        let sidebar = sidebar.clone();
        move || sidebar.refresh()
    });

    let paned = gtk4::Paned::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .start_child(&grid.widget())
        .end_child(&sidebar.widget())
        .position(720)
        .shrink_start_child(false)
        .shrink_end_child(false)
        .build();

    let header = adw::HeaderBar::new();
    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&paned));

    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("Icon Set Builder")
        .default_width(1100)
        .default_height(720)
        .content(&toolbar_view)
        .build();
    sidebar.attach(&window);
    window.present();
    Ok(())
}
