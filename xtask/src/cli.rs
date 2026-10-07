use clap::Parser;
use clap::Subcommand;

/// Workspace automation for vendored upstream resources
/// (`upstream.toml` / `upstream.lock`).
#[derive(Parser)]
#[command(name = "xtask", version, about)]
pub struct Cli {
    /// The operation to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Available operations.
#[derive(Subcommand)]
pub enum Command {
    /// Vendor upstream artifacts and metadata, updating upstream.lock.
    Sync {
        /// Workspace package to sync (e.g. fonts-rs-heroicons).
        #[arg(short, long)]
        package: String,
    },
    /// Report whether the manifest ref moved since the last sync.
    Check {
        /// Workspace package to check.
        #[arg(short, long)]
        package: String,
    },
    /// Verify the vendored tree matches upstream.lock (SHA-256).
    Verify {
        /// Workspace package to verify.
        #[arg(short, long)]
        package: String,
    },
}
