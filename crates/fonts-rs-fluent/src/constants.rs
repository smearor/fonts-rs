//! Icon name constants.
//!
//! Each constant represents a GTK-compatible icon name
//! (`fluent-{name}-symbolic` or `fluent-filled-{name}-symbolic` under
//! the `filled` variant). Generated at build time.

include!(concat!(env!("OUT_DIR"), "/icons.rs"));
