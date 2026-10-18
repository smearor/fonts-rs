//! The selection sidebar - settings pane, grouped picks with rename
//! editing, collision warnings and Save/Copy/Preview actions.

use crate::app::margins;
use fonts_rs_iconset_builder::BuilderSettings;
use fonts_rs_iconset_builder::IconSelection;
use fonts_rs_iconset_builder::IconSource;
use fonts_rs_iconset_builder::Preview;
use fonts_rs_iconset_builder::ProjectExport;
use fonts_rs_iconset_builder::Scaffold;
use fonts_rs_iconset_builder::SelectedIcon;
use fonts_rs_iconset_builder::SourceKey;
use gtk4::gdk;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

/// The right pane: what the set currently looks like plus the actions
/// that emit it.
pub struct SelectionSidebar {
    /// Shared selection model (grid toggles + rename edits write here).
    selection: Rc<RefCell<IconSelection>>,
    /// Shared `[iconset]` table fields.
    settings: Rc<RefCell<BuilderSettings>>,
    /// The grouped picks list, rebuilt on `refresh()`.
    picks: gtk4::ListBox,
    /// Live collision warning.
    collision_label: gtk4::Label,
    /// Toasts surface save/copy/preview errors non-modally.
    toast_overlay: adw::ToastOverlay,
    /// The application window (set by `attach`) for file dialogs -
    /// shared across `clone_shallow` handles so late attach works.
    window: Rc<RefCell<Option<adw::ApplicationWindow>>>,
}

impl SelectionSidebar {
    /// Builds the sidebar.
    pub fn new(selection: Rc<RefCell<IconSelection>>, settings: Rc<RefCell<BuilderSettings>>) -> Rc<Self> {
        let picks = gtk4::ListBox::new();
        picks.set_selection_mode(gtk4::SelectionMode::None);
        picks.add_css_class("boxed-list");
        margins(&picks, 12);

        let collision_label = gtk4::Label::new(None);
        collision_label.add_css_class("warning");
        margins(&collision_label, 8);
        collision_label.set_visible(false);

        let actions = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        margins(&actions, 8);
        let save_button = gtk4::Button::builder().label("Save icons.toml…").build();
        let export_button = gtk4::Button::builder().label("Export project…").build();
        let copy_button = gtk4::Button::builder().label("Copy scaffold").build();
        let preview_button = gtk4::Button::builder().label("Preview").build();
        actions.append(&save_button);
        actions.append(&export_button);
        actions.append(&copy_button);
        actions.append(&preview_button);

        let settings_group = build_settings_group(settings.clone());
        let content = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        content.append(&settings_group);
        content.append(&collision_label);
        content.append(&picks);
        content.append(&actions);

        let scrolled = gtk4::ScrolledWindow::builder().hexpand(true).child(&content).build();
        let toast_overlay = adw::ToastOverlay::new();
        toast_overlay.set_child(Some(&scrolled));

        let sidebar = Rc::new(Self {
            selection,
            settings,
            picks,
            collision_label,
            toast_overlay,
            window: Rc::new(RefCell::new(None)),
        });
        sidebar.wire_actions(&save_button, &export_button, &copy_button, &preview_button);
        sidebar.refresh();
        sidebar
    }

    /// Root widget for the paned layout.
    pub fn widget(&self) -> adw::ToastOverlay {
        self.toast_overlay.clone()
    }

    /// Stores the window for dialogs/toasts; call after the window
    /// exists.
    pub fn attach(&self, window: &adw::ApplicationWindow) {
        *self.window.borrow_mut() = Some(window.clone());
    }

    /// Rebuilds the picks list + collision state after a model change.
    pub fn refresh(&self) {
        while let Some(child) = self.picks.first_child() {
            self.picks.remove(&child);
        }
        let settings = self.settings.borrow();
        let groups: Vec<(SourceKey, &'static IconSource, BTreeMap<String, SelectedIcon>)> = self
            .selection
            .borrow()
            .groups()
            .map(|(key, source, icons)| (key, source, icons.clone()))
            .collect();
        for (key, source, icons) in groups {
            let subdirectory = key.1.map(|sub| format!("/{sub}")).unwrap_or_default();
            let header = gtk4::Label::builder().label(format!("{}{subdirectory}", source.crate_name)).xalign(0.0).build();
            header.add_css_class("heading");
            self.picks.append(&header);
            for icon in icons.values() {
                self.picks.append(&self.pick_row(key, icon.upstream.clone(), icon.rename.clone()));
            }
        }

        let dupes = self.selection.borrow().collisions(&settings);
        if dupes.is_empty() {
            self.collision_label.set_visible(false);
        } else {
            self.collision_label
                .set_label(&format!("Exported-name collisions: {}", dupes.iter().cloned().collect::<Vec<_>>().join(", ")));
            self.collision_label.set_visible(true);
        }
    }

    /// One pick row: upstream label, rename entry, remove button.
    fn pick_row(&self, source_key: SourceKey, upstream: String, rename: Option<String>) -> gtk4::ListBoxRow {
        let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        let name = gtk4::Label::builder().label(&upstream).xalign(0.0).hexpand(true).build();
        let exported = gtk4::Label::builder().xalign(0.0).build();
        exported.add_css_class("dim-label");
        let rename_entry = gtk4::Entry::builder()
            .placeholder_text("rename…")
            .text(rename.clone().unwrap_or_default())
            .width_chars(10)
            .build();
        let remove = gtk4::Button::builder().icon_name("window-close-symbolic").build();
        remove.add_css_class("flat");

        let update_exported = {
            let selection = self.selection.clone();
            let settings = self.settings.clone();
            let exported = exported.clone();
            let upstream = upstream.clone();
            move |rename: Option<String>| {
                let icon = fonts_rs_iconset_builder::SelectedIcon {
                    upstream: upstream.clone(),
                    rename,
                };
                exported.set_label(&format!("→ {}", selection.borrow().exported_name(&settings.borrow(), &icon)));
            }
        };
        update_exported(rename);
        rename_entry.connect_changed({
            let selection = self.selection.clone();
            let upstream = upstream.clone();
            let update_exported = update_exported.clone();
            move |entry| {
                let text = entry.text().to_string();
                selection
                    .borrow_mut()
                    .set_rename(source_key, &upstream, Some(text.clone()).filter(|t| !t.is_empty()));
                update_exported(Some(text).filter(|t| !t.is_empty()));
            }
        });
        remove.connect_clicked({
            let selection = self.selection.clone();
            let picks = self.picks.clone();
            let upstream = upstream.clone();
            move |button| {
                selection.borrow_mut().remove(source_key, &upstream);
                if let Some(row) = button.ancestor(gtk4::ListBoxRow::static_type()) {
                    picks.remove(&row);
                }
            }
        });

        row_box.append(&name);
        row_box.append(&exported);
        row_box.append(&rename_entry);
        row_box.append(&remove);
        gtk4::ListBoxRow::builder().child(&row_box).build()
    }

    /// Wires Save (FileDialog → toml), Export (FileDialog → project
    /// zip), Copy (scaffold to clipboard) and Preview
    /// (`IconSetBuild::preview` over the emitted spec).
    fn wire_actions(&self, save: &gtk4::Button, export: &gtk4::Button, copy: &gtk4::Button, preview: &gtk4::Button) {
        save.connect_clicked({
            let sidebar = Rc::new(self.clone_shallow());
            move |_| sidebar.save_toml()
        });
        export.connect_clicked({
            let sidebar = Rc::new(self.clone_shallow());
            move |_| sidebar.export_project()
        });
        copy.connect_clicked({
            let sidebar = Rc::new(self.clone_shallow());
            move |_| sidebar.copy_scaffold()
        });
        preview.connect_clicked({
            let sidebar = Rc::new(self.clone_shallow());
            move |_| sidebar.run_preview()
        });
    }

    /// Writes a compile-ready project zip via the save dialog.
    fn export_project(&self) {
        let Some(window) = self.window.borrow().clone() else {
            return;
        };
        let package = ProjectExport::package_name(&self.settings.borrow().name);
        let dialog = gtk4::FileDialog::builder().initial_name(format!("{package}.zip")).build();
        let selection = self.selection.clone();
        let settings = self.settings.clone();
        let overlay = self.toast_overlay.clone();
        dialog.save(Some(&window), gtk4::gio::Cancellable::NONE, move |result| {
            let Ok(file) = result else {
                return;
            };
            let Some(path) = file.path() else {
                overlay.add_toast(adw::Toast::new("no writable path for the chosen file"));
                return;
            };
            match ProjectExport::write_zip(&selection.borrow(), &settings.borrow(), &path) {
                Ok(()) => overlay.add_toast(adw::Toast::new("project exported")),
                Err(error) => overlay.add_toast(adw::Toast::new(&format!("export failed: {error}"))),
            }
        });
    }

    /// Writes the emitted `icons.toml` via the save dialog.
    fn save_toml(&self) {
        let text = match self.selection.borrow().to_toml(&self.settings.borrow()) {
            Ok(text) => text,
            Err(error) => {
                self.toast(&format!("invalid spec: {error}"));
                return;
            }
        };
        let Some(window) = self.window.borrow().clone() else {
            return;
        };
        let dialog = gtk4::FileDialog::builder().initial_name("icons.toml").build();
        let overlay = self.toast_overlay.clone();
        dialog.save(Some(&window), gtk4::gio::Cancellable::NONE, move |result| {
            if let Ok(file) = result
                && let Err(error) = file.replace_contents(text.as_bytes(), None, false, gtk4::gio::FileCreateFlags::NONE, gtk4::gio::Cancellable::NONE)
            {
                overlay.add_toast(adw::Toast::new(&format!("write failed: {error}")));
                return;
            }
            overlay.add_toast(adw::Toast::new("icons.toml saved"));
        });
    }

    /// Puts the scaffold block (Cargo.toml deps + build.rs + include)
    /// on the clipboard.
    fn copy_scaffold(&self) {
        let text = Scaffold::text(&self.selection.borrow(), &self.settings.borrow());
        if let Some(display) = gdk::Display::default() {
            display.clipboard().set_text(&text);
        }
        self.toast("scaffold copied");
    }

    /// Runs `IconSetBuild::preview` over the emitted spec and shows the
    /// exported names + codepoints.
    fn run_preview(&self) {
        let text = match self.selection.borrow().to_toml(&self.settings.borrow()) {
            Ok(text) => text,
            Err(error) => {
                self.toast(&format!("invalid spec: {error}"));
                return;
            }
        };
        match Preview::run(&text) {
            Ok(preview) => self.show_preview(&preview),
            Err(error) => self.toast(&format!("preview failed: {error}")),
        }
    }

    /// Renders the preview result in a dialog - name, codepoint and the
    /// exported SVG per icon.
    fn show_preview(&self, preview: &Preview) {
        let list = gtk4::ListBox::new();
        list.set_selection_mode(gtk4::SelectionMode::None);
        for icon in &preview.icons {
            let image = gtk4::Image::builder().file(icon.file.to_string_lossy()).pixel_size(24).build();
            let name = gtk4::Label::builder().label(&icon.name).xalign(0.0).hexpand(true).build();
            let code = gtk4::Label::builder()
                .label(icon.codepoint.map(|cp| cp.to_string()).unwrap_or_else(|| "—".to_string()))
                .build();
            code.add_css_class("dim-label");
            let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            margins(&row, 4);
            row.append(&image);
            row.append(&name);
            row.append(&code);
            list.append(&row);
        }
        let scrolled = gtk4::ScrolledWindow::builder().min_content_height(360).child(&list).build();
        let dialog = adw::Dialog::builder()
            .title(format!("Preview — {} exported icons", preview.icons.len()))
            .child(&scrolled)
            .content_width(520)
            .content_height(480)
            .build();
        if let Some(window) = self.window.borrow().as_ref() {
            dialog.present(Some(window));
        }
    }

    /// Non-modal feedback.
    fn toast(&self, message: &str) {
        self.toast_overlay.add_toast(adw::Toast::new(message));
    }

    /// Shallow handle for closures - shares the model + widgets.
    fn clone_shallow(&self) -> Self {
        Self {
            selection: self.selection.clone(),
            settings: self.settings.clone(),
            picks: self.picks.clone(),
            collision_label: self.collision_label.clone(),
            toast_overlay: self.toast_overlay.clone(),
            window: self.window.clone(),
        }
    }
}

/// The `[iconset]` fields as `EntryRow`s bound to the shared settings.
fn build_settings_group(settings: Rc<RefCell<BuilderSettings>>) -> adw::PreferencesGroup {
    let group = adw::PreferencesGroup::builder()
        .title("Icon Set")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    for (title, get) in [
        ("name", (|s: &BuilderSettings| s.name.clone()) as fn(&BuilderSettings) -> String),
        ("display_name", |s| s.display_name.clone()),
        ("icon_prefix", |s| s.icon_prefix.clone()),
        ("gresource_prefix", |s| s.gresource_prefix.clone()),
        ("icons_context", |s| s.icons_context.clone()),
    ] {
        let row = adw::EntryRow::builder().title(title).text(get(&settings.borrow())).build();
        row.connect_changed({
            let settings = settings.clone();
            move |entry| {
                let mut settings = settings.borrow_mut();
                let text = entry.text().to_string();
                match title {
                    "name" => settings.name = text,
                    "display_name" => settings.display_name = text,
                    "icon_prefix" => settings.icon_prefix = text,
                    "gresource_prefix" => settings.gresource_prefix = text,
                    "icons_context" => settings.icons_context = text,
                    _ => (),
                }
            }
        });
        group.add(&row);
    }
    group
}
