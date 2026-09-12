//! Builder pattern for fonts-rs initialization.

use super::error::InitError;

/// Configuration options for initializing fonts-rs.
///
/// Construct with [`InitOptions::new`] and chain optional configuration
/// methods before calling [`InitOptions::init`].
///
/// # Examples
///
/// ```no_run
/// use nerd_fonts_rs::InitOptions;
///
/// // Default options (no custom font directory)
/// InitOptions::new().init();
///
/// // With a custom font base directory (requires `render` or `pango` feature)
/// # #[cfg(any(feature = "render", feature = "pango"))]
/// InitOptions::new().font_base_dir("/usr/share/fonts").init();
/// ```
#[derive(Debug, Clone, Default)]
pub struct InitOptions {
    /// Base directory for font file lookups (render and pango features).
    #[cfg(any(feature = "render", feature = "pango"))]
    font_base_dir: Option<String>,
}

impl InitOptions {
    /// Creates a new `InitOptions` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base directory for font file lookups.
    ///
    /// Available with the `render` or `pango` feature. When not set, fonts are
    /// loaded from system-wide paths or embedded data (`embed-fonts` feature).
    #[cfg(any(feature = "render", feature = "pango"))]
    pub fn font_base_dir(mut self, dir: impl Into<String>) -> Self {
        self.font_base_dir = Some(dir.into());
        self
    }

    /// Initializes fonts-rs with the configured options.
    ///
    /// - Registers GResource bundles (gtk feature)
    /// - Registers vendored icon GResource (gtk feature)
    /// - Sets font search base directory (render feature, without embed-fonts)
    /// - Loads Nerd Font TTF files into the global Pango-Cairo font map (pango feature)
    /// - Loads icon CSS classes into GTK display (gtk feature)
    ///
    /// Call once at application startup.
    ///
    /// # Errors
    ///
    /// Returns [`InitError`] if GResource registration, font loading, or CSS loading fails.
    pub fn init(self) -> Result<(), InitError> {
        #[cfg(feature = "render")]
        crate::fonts::init(self.font_base_dir.as_deref());

        #[cfg(all(feature = "pango", not(feature = "embed-fonts")))]
        crate::font_loader::set_base_dir(self.font_base_dir.clone());

        #[cfg(feature = "pango")]
        crate::font_loader::load_fonts()?;

        #[cfg(feature = "gtk")]
        {
            gio::resources_register_include!("compiled.gresource").map_err(|e| InitError::GResourceRegister(e.to_string()))?;

            crate::icons::register_icons().map_err(|e| InitError::IconRegister(e.to_string()))?;

            let provider = gtk4::CssProvider::new();
            let css = crate::css::icon_css();
            #[cfg(feature = "v4_12")]
            {
                provider.load_from_string(&css);
            }
            #[cfg(not(feature = "v4_12"))]
            {
                #[allow(deprecated)]
                provider.load_from_data(&css);
            }
            if let Some(display) = gtk4::gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
        }

        Ok(())
    }
}
