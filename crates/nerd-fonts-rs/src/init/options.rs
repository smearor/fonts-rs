//! Builder pattern for nerd-fonts-gtk initialization.

use super::error::InitError;

/// Configuration options for initializing nerd-fonts-gtk.
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
/// // With a custom font base directory (requires `render` feature)
/// # #[cfg(feature = "render")]
/// InitOptions::new().font_base_dir("/usr/share/fonts").init();
/// ```
#[derive(Debug, Clone, Default)]
pub struct InitOptions {
    /// Base directory for font file lookups (render feature only).
    #[cfg(feature = "render")]
    font_base_dir: Option<String>,
}

impl InitOptions {
    /// Creates a new `InitOptions` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base directory for font file lookups.
    ///
    /// Only available with the `render` feature. When not set, fonts are
    /// loaded from system-wide paths or embedded data (`embed-fonts` feature).
    #[cfg(feature = "render")]
    pub fn font_base_dir(mut self, dir: impl Into<String>) -> Self {
        self.font_base_dir = Some(dir.into());
        self
    }

    /// Initializes nerd-fonts-gtk with the configured options.
    ///
    /// - Registers GResource bundles (gtk feature)
    /// - Registers vendored icon GResource (gtk feature)
    /// - Sets font search base directory (render feature, without embed-fonts)
    /// - Loads font-face CSS into GTK display (gtk feature)
    ///
    /// Call once at application startup.
    ///
    /// # Errors
    ///
    /// Returns [`InitError`] if GResource registration or CSS loading fails.
    pub fn init(self) -> Result<(), InitError> {
        #[cfg(feature = "render")]
        crate::fonts::init(self.font_base_dir.as_deref());

        #[cfg(feature = "gtk")]
        {
            gio::resources_register_include!("compiled.gresource").map_err(|e| InitError::GResourceRegister(e.to_string()))?;

            crate::icons::register_icons().map_err(|e| InitError::IconRegister(e.to_string()))?;

            let provider = gtk4::CssProvider::new();
            let css = crate::css::font_face_css();
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
