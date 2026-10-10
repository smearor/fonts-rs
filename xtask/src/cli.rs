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
        /// Re-pin `ref` to the latest upstream state before syncing:
        /// commit pins track the default HEAD, tag pins track the
        /// newest version tag. Branch refs already float - `--update`
        /// is a plain sync for them. Review the rewritten
        /// `upstream.toml` as part of the bump commit.
        #[arg(long)]
        update: bool,
    },
    /// Report upstream drift: moved refs, newer HEADs/tags and
    /// changed URL artifacts, aggregated across vendored crates.
    Check {
        /// Workspace package to check; all vendored crates when
        /// omitted.
        #[arg(short, long)]
        package: Option<String>,
        /// Emit the report as JSON on stdout for CI parsing.
        #[arg(long)]
        json: bool,
    },
    /// Verify the vendored tree matches upstream.lock (SHA-256).
    Verify {
        /// Workspace package to verify; all vendored crates when
        /// omitted.
        #[arg(short, long)]
        package: Option<String>,
    },
}
