//! Shared Nerd Font integration for GTK4 projects.
//!
//! Provides icon name resolution, font loading, software rendering,
//! and CSS generation for Nerd Font symbols.
//!
//! Copyright (c) 2026 smearor
//! Licensed under the MIT License.

pub mod color;
pub mod css;
pub mod icons;

#[cfg(feature = "gtk")]
pub mod gtk;

#[cfg(feature = "render")]
pub mod drawing;
#[cfg(feature = "render")]
pub mod fonts;

#[cfg(feature = "web")]
pub mod web;

pub use color::Color;
pub use icons::resolve_icon_codepoint;

/// Initialize nerd-fonts-gtk.
///
/// - Registers GResource (gtk feature)
/// - Registers nerd_gtk_icons (gtk feature)
/// - Sets font search base directory (render feature, without embed-fonts)
/// - Loads font-face CSS into GTK display (gtk feature)
///
/// Call once at application startup.
#[cfg(feature = "gtk")]
pub fn init(#[cfg(feature = "render")] base_dir: Option<&str>, #[cfg(not(feature = "render"))] _base_dir: Option<&str>) {
    #[cfg(feature = "render")]
    fonts::init(base_dir);

    if let Err(e) = gio::resources_register_include!("compiled.gresource") {
        tracing::error!("Failed to register nerd-fonts GResource: {e}");
    }

    if let Err(e) = nerd_gtk_icons::register_icons() {
        tracing::error!("Failed to register nerd font icons: {e}");
    }

    let provider = gtk4::CssProvider::new();
    provider.load_from_data(css::FONT_FACE_CSS);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

/// Initialize without GTK (render-only mode).
#[cfg(not(feature = "gtk"))]
pub fn init(#[cfg(feature = "render")] base_dir: Option<&str>, #[cfg(not(feature = "render"))] _base_dir: Option<&str>) {
    #[cfg(feature = "render")]
    fonts::init(base_dir);
}
