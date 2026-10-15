# fonts-rs-noto-emoji-sequences

[![crates.io](https://img.shields.io/crates/v/fonts-rs-noto-emoji-sequences.svg)](https://crates.io/crates/fonts-rs-noto-emoji-sequences)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: OFL-1.1](https://img.shields.io/badge/License-OFL--1.1-blue.svg)](https://openfontlicense.org)

Noto Emoji multi-codepoint SVG sequences (ZWJ, flags, modifiers - build resource).

## Overview

Build-resource crate carrying the vendored `resources/svg`
multi-codepoint sequences: ZWJ compositions, regional-indicator flags
and skin-tone modifier sequences. Dependent crates locate the directory
through the `DEP_NOTO_EMOJI_SEQUENCES_SVG_DIR` environment variable
emitted by `build.rs` (requires declaring this crate as a normal
dependency - `links` metadata does not propagate via
`[build-dependencies]`).

Not intended for direct use by applications - use a variant crate such
as `fonts-rs-noto-emoji-bitmap-flags`, `fonts-rs-noto-emoji-color-flags`
or `fonts-rs-noto-emoji-mono` instead. The shared CLDR/UTS-#51 emoji
metadata lives in `fonts-rs-emoji-metadata`.

## License

Noto Emoji artwork is OFL-1.1 - see `resources/OFL.txt`.
