//! Runtime GTK4 version detection for CSS adaptation.
//!
//! Different GTK4 versions ship different CSS parsers with varying levels of
//! `@font-face` support. This module provides a lightweight wrapper around the
//! runtime version functions exposed by gtk4-rs so that the CSS generation
//! logic can branch on the actual library version rather than the
//! compile-time feature flags.

/// Detected GTK4 runtime version.
///
/// Obtained via [`GtkVersion::runtime`] when the `gtk` feature is enabled.
/// The fields mirror the values returned by `gtk4::major_version()`,
/// `gtk4::minor_version()`, and `gtk4::micro_version()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GtkVersion {
    /// Major version component (e.g. `4` for GTK 4.14.2).
    pub major: u32,
    /// Minor version component (e.g. `14` for GTK 4.14.2).
    pub minor: u32,
    /// Micro/patch version component (e.g. `2` for GTK 4.14.2).
    pub micro: u32,
}

impl GtkVersion {
    /// Creates a new `GtkVersion` from explicit components.
    pub const fn new(major: u32, minor: u32, micro: u32) -> Self {
        Self { major, minor, micro }
    }

    /// Queries the currently loaded GTK4 library and returns its version.
    #[cfg(feature = "gtk")]
    pub fn runtime() -> Self {
        Self {
            major: gtk4::major_version(),
            minor: gtk4::minor_version(),
            micro: gtk4::micro_version(),
        }
    }

    /// Returns `true` when the version is at least `major.minor`.
    pub fn at_least(&self, major: u32, minor: u32) -> bool {
        (self.major, self.minor) >= (major, minor)
    }

    /// Returns `true` when the version is at least `major.minor.micro`.
    pub fn at_least_micro(&self, major: u32, minor: u32, micro: u32) -> bool {
        (self.major, self.minor, self.micro) >= (major, minor, micro)
    }
}

impl std::fmt::Display for GtkVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_display() {
        let v = GtkVersion::new(4, 14, 2);
        assert_eq!(v.to_string(), "4.14.2");
    }

    #[test]
    fn at_least_same_version() {
        let v = GtkVersion::new(4, 10, 0);
        assert!(v.at_least(4, 10));
    }

    #[test]
    fn at_least_newer_minor() {
        let v = GtkVersion::new(4, 14, 0);
        assert!(v.at_least(4, 10));
    }

    #[test]
    fn at_least_older_minor() {
        let v = GtkVersion::new(4, 8, 0);
        assert!(!v.at_least(4, 10));
    }

    #[test]
    fn at_least_micro_same() {
        let v = GtkVersion::new(4, 12, 1);
        assert!(v.at_least_micro(4, 12, 1));
    }

    #[test]
    fn at_least_micro_newer() {
        let v = GtkVersion::new(4, 12, 5);
        assert!(v.at_least_micro(4, 12, 1));
    }

    #[test]
    fn at_least_micro_older() {
        let v = GtkVersion::new(4, 12, 0);
        assert!(!v.at_least_micro(4, 12, 1));
    }

    #[test]
    fn ordering_works() {
        let old = GtkVersion::new(4, 8, 0);
        let new = GtkVersion::new(4, 14, 2);
        assert!(old < new);
        assert!(new > old);
    }

    #[cfg(feature = "gtk")]
    #[test]
    fn runtime_version_is_gtk4() {
        let v = GtkVersion::runtime();
        assert_eq!(v.major, 4, "runtime GTK major should be 4");
    }
}
