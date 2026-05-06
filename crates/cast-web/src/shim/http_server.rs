//! Axum router builder + bind/serve.
//!
//! Mounted surfaces:
//! - `/`               — index.html from the embedded bundle
//! - `/static/*path`   — embedded TypeScript bundle (rust-embed)
//! - `/ws`             — WebSocket upgrade, bridges to cast-watch
//! - `/mailbox/*`      — mailbox plane (POST send, GET messages,
//!                       GET meta, GET stream/SSE) when --mailbox-dir
//!                       is provided
//!
//! Everything in this file is wiring. The interesting logic lives
//! in `ws_server`, `watch_client`, `mailbox_routes`, and the
//! `mailbox` module proper.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use axum::extract::FromRef;
use axum::routing::{get, post};
use tracing::info;

use crate::frontend;
use crate::mailbox::{Mailbox, watch_directory};

use super::mailbox_routes::{self, MailboxState};
use super::watcher_routes::{self, WatcherLog, WatcherLogState, WatcherQueryState};
use super::ws_server::{WsState, ws_handler};

/// Composed state: ws bridge needs only the watch socket; mailbox
/// routes need the Mailbox + the directory watch; watcher-query
/// routes need the watch socket and (optionally) the disk log.
/// axum 0.7's `FromRef` derives the per-extractor sub-state.
#[derive(Clone)]
pub struct AppState {
    pub ws: WsState,
    pub mailbox: Option<MailboxState>,
    pub watcher_log: Option<Arc<WatcherLog>>,
    pub watcher_log_state: Option<WatcherLogState>,
}

impl FromRef<AppState> for WsState {
    fn from_ref(input: &AppState) -> Self {
        input.ws.clone()
    }
}

impl FromRef<AppState> for WatcherQueryState {
    fn from_ref(input: &AppState) -> Self {
        WatcherQueryState {
            watch_socket: input.ws.watch_socket.clone(),
            log: input.watcher_log.clone(),
        }
    }
}

impl FromRef<AppState> for MailboxState {
    fn from_ref(input: &AppState) -> Self {
        input
            .mailbox
            .clone()
            .expect("mailbox routes mounted without --mailbox-dir")
    }
}

impl FromRef<AppState> for WatcherLogState {
    fn from_ref(input: &AppState) -> Self {
        input
            .watcher_log_state
            .clone()
            .expect("watcher-log routes mounted without --watcher-log-dir")
    }
}

pub struct ServeOptions {
    pub addr: SocketAddr,
    pub watch_socket: PathBuf,
    pub mailbox_dir: Option<PathBuf>,
    pub watcher_log_dir: Option<PathBuf>,
}

pub async fn serve(opts: ServeOptions) -> Result<()> {
    let mut app = Router::new()
        .route("/", get(frontend::index))
        .route("/static/*path", get(frontend::asset))
        .route("/ws", get(ws_handler))
        .route("/watcher/query", post(watcher_routes::query));

    let mailbox_state = if let Some(dir) = opts.mailbox_dir.as_ref() {
        let mailbox = Mailbox::new(dir.clone());
        mailbox.ensure().await?;
        let watch = watch_directory::start(dir, watch_directory::mailbox_filter())?;
        let state = MailboxState {
            mailbox: Arc::new(mailbox),
            watch: Arc::new(watch),
        };
        app = app
            .route("/mailbox/send", post(mailbox_routes::send))
            .route("/mailbox/clear", post(mailbox_routes::clear))
            .route("/mailbox/messages", get(mailbox_routes::list_messages))
            .route("/mailbox/meta", get(mailbox_routes::meta))
            .route("/mailbox/stream", get(mailbox_routes::stream));
        Some(state)
    } else {
        None
    };

    let (watcher_log, watcher_log_state) = if let Some(dir) = opts.watcher_log_dir.as_ref() {
        let log = WatcherLog::new(dir.clone());
        log.ensure().await?;
        let log = Arc::new(log);
        let watch = watch_directory::start(dir, watch_directory::json_filter())?;
        let state = WatcherLogState {
            log: log.clone(),
            watch: Arc::new(watch),
        };
        app = app
            .route("/watcher/log/entries", get(watcher_routes::list_entries))
            .route("/watcher/log/clear", post(watcher_routes::clear))
            .route("/watcher/log/stream", get(watcher_routes::stream));
        (Some(log), Some(state))
    } else {
        (None, None)
    };

    let state = AppState {
        ws: WsState {
            watch_socket: Arc::new(opts.watch_socket),
        },
        mailbox: mailbox_state,
        watcher_log,
        watcher_log_state,
    };
    let app = app.with_state(state);

    let listener = tokio::net::TcpListener::bind(opts.addr).await?;
    info!(addr = %opts.addr, "cast-web http listening");
    axum::serve(listener, app).await?;
    Ok(())
}
