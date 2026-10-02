//! Icon name constants.
//!
//! Each constant represents a GTK-compatible icon name
//! (`la-{name}-symbolic` / `la-{name}-solid-symbolic`). Generated at
//! build time.

include!(concat!(env!("OUT_DIR"), "/icons.rs"));
