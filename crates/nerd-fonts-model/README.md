# nerd-fonts-model

Shared data types for Nerd Font icon handling.

## Overview

This crate provides Nerd Fonts-specific model types extending `fonts-rs-model`,
including `IconName` and `IconEntry` types for Nerd Font icon metadata.

## Usage

This crate is typically used as a dependency of `nerd-fonts-generator` and
`nerd-fonts-rs`. You rarely depend on it directly.

```toml
[dependencies]
nerd-fonts-model = "0.1"
```

## License

MIT
