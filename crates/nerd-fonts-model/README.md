# nerd-fonts-model

[![crates.io](https://img.shields.io/crates/v/nerd-fonts-model.svg)](https://crates.io/crates/nerd-fonts-model)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/editions/2024/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Shared data types for Nerd Font icon handling.

## Overview

This crate provides Nerd Fonts-specific model types extending `fonts-rs-model`,
including `IconName` and `IconEntry` types for Nerd Font icon metadata.

## Usage

This crate is typically used as a dependency of `nerd-fonts-generator` and
`nerd-fonts-rs`. You rarely depend on it directly.

```toml
[dependencies]
nerd-fonts-model = "0.5"
```

## License

MIT
