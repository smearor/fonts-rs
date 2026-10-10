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
mod git_ref;
mod git_repository;
mod json;
mod layout;
mod lockfile;
mod manifest;
mod providers;
mod sha256;
mod sync;
mod upstream;
mod url;
mod verify;

use crate::cli::Cli;
use crate::cli::Command;
use crate::layout::Crate;
use crate::layout::Package;
use crate::layout::Workspace;
use clap::Parser;

fn main() -> miette::Result<()> {
    match Cli::parse().command {
        Command::Sync { package, update } => Crate::load(Package::from(package))?.sync(update),
        Command::Check { package, json } => Workspace::load(package.map(Package::from))?.check(json),
        Command::Verify { package } => Workspace::load(package.map(Package::from))?.verify(),
    }
}
