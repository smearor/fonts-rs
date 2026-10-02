//! Shared CSS loading helper for the GTK font families.

/// Loads `css` into a `CssProvider` and attaches it to the default
/// display at application priority.
///
/// Shared by every font family's init path - the provider is added on
/// every call; a duplicate provider is harmless and preserves retry
/// semantics when no display existed on the first call.
///
/// No-op when no display is available (e.g. headless).
pub fn load_css(css: &str) {
    let provider = gtk4::CssProvider::new();
    #[cfg(feature = "v4_12")]
    {
        provider.load_from_string(css);
    }
    #[cfg(not(feature = "v4_12"))]
    {
        #[allow(deprecated)]
        provider.load_from_data(css);
    }
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}
