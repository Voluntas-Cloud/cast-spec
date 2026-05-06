//! CLI entry point for cast-watch.
//!
//! Parses arguments, sets up tracing, and hands off to `Daemon::start`.
//! All logic lives in the library — main is paper-thin.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cast-watch",
    about = "Live, file-watching cast analyzer with a query socket for LLMs."
)]
struct Cli {
    /// One or more project roots, same shape as cast-extract. Each is
    /// loaded as its own rust-analyzer DB on the initial pass; the
    /// per-handle pipeline is reused for incremental rebuilds.
    #[arg(required = true, num_args = 1..)]
    projects: Vec<PathBuf>,

    /// Path to the Unix domain socket where the query server listens.
    /// Defaults to `/tmp/cast-watch.sock`.
    #[arg(long, default_value = "/tmp/cast-watch.sock")]
    socket: PathBuf,

    /// Eager mode: implementation-file changes trigger an immediate
    /// RA reload. Default is lazy — implementation changes mark
    /// anchors stale and wait for an explicit `rebuild` query.
    #[arg(long, default_value_t = false)]
    eager: bool,

    /// Refuse to silently walk up into an enclosing Cargo workspace.
    /// When set, cast-watch errors out if any project root is a member
    /// of a larger workspace, naming the parent so the user can either
    /// point at the workspace root or restructure the sub-crate as
    /// standalone.
    #[arg(long, default_value_t = false)]
    no_walk_up: bool,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    if cli.no_walk_up {
        for root in &cli.projects {
            cast::assert_standalone_root(root)?;
        }
    }

    cast_watch::start(cli.socket, cli.projects, cli.eager)
}
