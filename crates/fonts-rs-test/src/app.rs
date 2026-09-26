//! GTK application boilerplate.
//!
//! Provides [`run_gtk_app`] as a convenience for creating and running
//! a GTK application with a custom `build_ui` closure.

use gtk4::Application;
use gtk4::glib;
use gtk4::prelude::*;
use miette::IntoDiagnostic;
use miette::Result;

/// Creates and runs a GTK application.
///
/// Initializes GTK, creates an `Application` with the given `app_id`,
/// connects `build_ui` to the `activate` signal, and runs the app.
///
/// The app runs with [`ApplicationFlags::NON_UNIQUE`]: examples are
/// demos, so every process must be a real instance. With the default
/// single-instance behavior a second launch would only forward
/// `activate` to the running process over D-Bus and exit - the running
/// instance would then open another window with its own (stale)
/// embedded font variant instead of the newly built one.
///
/// # Errors
///
/// Returns an error if GTK initialization fails.
pub fn run_gtk_app(app_id: &str, build_ui: impl Fn(&Application) + 'static) -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    let app = Application::builder()
        .application_id(app_id)
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();
    app.connect_activate(build_ui);
    Ok(app.run())
}
