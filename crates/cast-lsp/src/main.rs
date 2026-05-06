//! CLI entry point for cast-lsp.
//!
//! Parses arguments, sets up tracing, and hands off to `cast_lsp::run`.
//! All logic lives in the library — main is paper-thin.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "cast-lsp",
    about = "Language Server Protocol implementation for cast::*! annotations."
)]
struct Cli {
    /// Project root override. By default cast-lsp uses the rootUri sent
    /// by the editor in the `initialize` request.
    #[arg(long)]
    root: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
    // Trace to stderr — stdout is the LSP transport, must stay clean.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    cast_lsp::run(cli.root)
}
