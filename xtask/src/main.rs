//! xtask: vendored-upstream automation for the fonts-rs workspace.
//!
//! Implements Phase 1 of `concepts/planned/REPRODUCABLE.md`: a git
//! fetch backend with commit-keyed sparse-checkout caching, manifest
//! (`upstream.toml`) and lockfile (`upstream.lock`) schemas, the
//! `copy`, `heroicons_keywords` and `iconify_aliases` providers, and
//! the `sync`, `check` and `verify` subcommands.

mod check;
mod cli;
mod fetch;
mod json;
mod lockfile;
mod manifest;
mod paths;
mod providers;
mod sha256;
mod sync;
mod upstream;
mod verify;

use crate::cli::Cli;
use crate::cli::Command;
use clap::Parser;

fn main() -> miette::Result<()> {
    match Cli::parse().command {
        Command::Sync { package } => sync::sync(&package),
        Command::Check { package } => check::check(&package),
        Command::Verify { package } => verify::verify(&package),
    }
}
