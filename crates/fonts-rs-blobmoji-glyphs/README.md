# fonts-rs-blobmoji-glyphs

[![crates.io](https://img.shields.io/crates/v/fonts-rs-blobmoji-glyphs.svg)](https://crates.io/crates/fonts-rs-blobmoji-glyphs)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: OFL-1.1](https://img.shields.io/badge/License-OFL--1.1-blue.svg)](https://openfontlicense.org)

Blobmoji single-codepoint SVG artwork (build resource).

## Overview

Build-resource crate carrying the vendored `resources/svg`
single-codepoint artwork. Dependent crates locate the directory through
the `DEP_BLOBMOJI_GLYPHS_SVG_DIR` environment variable emitted by
`build.rs` (requires declaring this crate as a normal dependency -
`links` metadata does not propagate via `[build-dependencies]`).

Not intended for direct use by applications - use a variant crate such
as `fonts-rs-blobmoji-bitmap-flags` or `fonts-rs-blobmoji-color-noflags`
instead.

## License

Blobmoji artwork is OFL-1.1 - see `resources/OFL.txt`.
