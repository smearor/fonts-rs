//! Initialization module: builder pattern for configuring and initializing nerd-fonts-gtk.

pub mod error;
pub mod options;

pub use error::InitError;
pub use options::InitOptions;

/// Convenience wrapper for [`InitOptions::new().init()`] with default options.
///
/// Call once at application startup. For custom configuration, use
/// [`InitOptions`] directly.
///
/// # Errors
///
/// Returns [`InitError`] if initialization fails. See [`InitOptions::init`].
pub fn init() -> Result<(), InitError> {
    InitOptions::new().init()
}
