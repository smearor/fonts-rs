# fonts-rs-emoji-metadata

[![crates.io](https://img.shields.io/crates/v/fonts-rs-emoji-metadata.svg)](https://crates.io/crates/fonts-rs-emoji-metadata)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT AND Unicode-3.0 AND Apache-2.0](https://img.shields.io/badge/License-MIT%20AND%20Unicode--3.0%20AND%20Apache--2.0-orange.svg)](https://spdx.org/licenses/)

Shared CLDR/UTS-#51 emoji metadata for emoji font crates (build resource).

## Overview

Build-resource crate carrying the vendored CLDR/UTS-#51 emoji metadata
shared by the Noto Emoji and Blobmoji family crates. Dependent crates
locate the directory through the `DEP_EMOJI_METADATA_DIR` environment
variable emitted by `build.rs` (requires declaring this crate as a
normal dependency - `links` metadata does not propagate via
`[build-dependencies]`).

Not intended for direct use by applications.

## License

MIT. See `upstream.toml` for metadata provenance.
