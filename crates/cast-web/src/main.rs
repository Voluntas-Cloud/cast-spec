//! cast-web binary entry. CLI parses `--socket` / `--http` /
//! `--mailbox-dir` and hands them off to `shim::http_server::serve`.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cast_web::shim::http_server::{ServeOptions, serve};

#[derive(Parser, Debug)]
#[command(name = "cast-web", about = "Browser UI + LLM-mailbox shim over a cast-watch socket")]
struct Args {
    /// Path to the cast-watch unix socket to relay to.
    #[arg(long)]
    socket: PathBuf,

    /// Address to bind the HTTP server.
    #[arg(long, default_value = "127.0.0.1:8765")]
    http: SocketAddr,

    /// Mailbox directory (created if absent). Required to enable
    /// the `/mailbox/*` routes; omit to run a graph-only viewer.
    #[arg(long)]
    mailbox_dir: Option<PathBuf>,

    /// Watcher-log directory (created if absent). When set, every
    /// `POST /watcher/query` request and response is tee'd to this
    /// directory as a numbered `<NNN>-query.json` /
    /// `<NNN+1>-response.json` pair. Omit to drop disk logging.
    #[arg(long)]
    watcher_log_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let args = Args::parse();

    serve(ServeOptions {
        addr: args.http,
        watch_socket: args.socket,
        mailbox_dir: args.mailbox_dir,
        watcher_log_dir: args.watcher_log_dir,
    })
    .await
}
