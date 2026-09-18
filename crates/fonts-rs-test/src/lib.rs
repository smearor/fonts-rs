//! Shared example utilities for fonts-rs font family crates.
//!
//! Provides generic functions for common GTK example patterns:
//! - [`run_gtk_icon_theme`] — GtkIconTheme lookup
//! - [`run_composed_svg_text`] — composed SVG text
//! - [`run_css_class_html`] — CSS @font-face HTML
//! - [`run_embed_fonts_render`] — ab_glyph software rendering
//! - [`run_svg_gresource`] — SVG glyphs from GResource
//! - [`run_svg_html`] — SVG HTML page
//! - [`run_gtk_gresource`] — GTK GResource font loading
//! - [`run_system_font_fallback`] — system font fallback

#[cfg(feature = "gtk")]
mod app;
#[cfg(feature = "gtk")]
mod css_html;
#[cfg(feature = "gtk")]
mod gtk_gresource;
#[cfg(feature = "gtk")]
mod icon_theme;
#[cfg(all(feature = "gtk", feature = "render"))]
mod render;
#[cfg(feature = "gtk")]
mod svg_compose;
#[cfg(feature = "gtk")]
mod svg_gresource;
#[cfg(feature = "gtk")]
mod svg_html;
#[cfg(feature = "gtk")]
mod system_font_fallback;

#[cfg(feature = "gtk")]
pub use app::run_gtk_app;
#[cfg(feature = "gtk")]
pub use css_html::run_css_class_html;
#[cfg(feature = "gtk")]
pub use gtk_gresource::run_gtk_gresource;
#[cfg(feature = "gtk")]
pub use icon_theme::run_gtk_icon_theme;
#[cfg(all(feature = "gtk", feature = "render"))]
pub use render::run_embed_fonts_render;
#[cfg(feature = "gtk")]
pub use svg_compose::run_composed_svg_text;
#[cfg(feature = "gtk")]
pub use svg_gresource::run_svg_gresource;
#[cfg(feature = "gtk")]
pub use svg_html::run_svg_html;
#[cfg(feature = "gtk")]
pub use system_font_fallback::run_system_font_fallback;
