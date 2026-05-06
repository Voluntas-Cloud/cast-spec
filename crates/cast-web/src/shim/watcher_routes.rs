//! HTTP surface for one-shot synchronous queries to cast-watch.
//!
//! `POST /watcher/query` accepts a JSON body — the cast-watch
//! request as it would be sent over the unix socket — opens a
//! short-lived `WatchClient` connection, forwards one line, reads
//! one line back, and returns it.
//!
//! Why this exists alongside `/ws`: the WebSocket bridge is the
//! right shape for browser-side persistent sessions (REPL pattern),
//! but ad-hoc command-line tooling like `dev/cwq.sh` wants a single
//! curl-able endpoint. Both surfaces reach the same watcher; this
//! one is for callers who don't want to hold a connection.
//!
//! When a watcher-log directory is configured, every request and
//! response is tee'd to disk as a numbered JSON file in the same
//! shape as the user/assistant mailbox. That makes my queries
//! observable: the UI tails the directory and renders my watcher
//! conversation alongside the chat thread.

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::{Stream, StreamExt};
use serde::Serialize;
use serde_json::Value;
use tokio_stream::wrappers::BroadcastStream;
use tracing::warn;

use crate::mailbox::watch_directory::{DirectoryEvent, DirectoryWatch};

use super::watch_client::WatchClient;

// One round trip through `POST /watcher/query`. Each stage is the
// thing that happens to the request as it travels caller → watcher
// → caller, plus the disk-tee side effect. The flow forks at
// read_response: the parsed body goes to both the log writer and
// straight back to the caller; logging is a side effect, not a
// delivery channel.
cast::pipeline! {
    stages: {
        encode_request   @ crate::shim::watcher_routes::query,
        connect_watcher  @ crate::shim::watch_client::WatchClient,
        send_query       @ crate::shim::watch_client::WatchClient,
        read_response    @ crate::shim::watch_client::WatchClient,
        tee_to_log       @ crate::shim::watcher_routes::WatcherLog,
        return_to_caller @ crate::shim::watcher_routes::query,
    },
    flow: [
        encode_request   -> connect_watcher,
        connect_watcher  -> send_query,
        send_query       -> read_response,
        read_response    -> tee_to_log,
        read_response    -> return_to_caller,
        tee_to_log       -> return_to_caller,
    ],
    cyclic: false,
    entry: encode_request,
    tags: ["cast_web"],
    note: "tee_to_log only fires when --watcher-log-dir is configured \
           AND the watcher response parsed as JSON. append_turn writes \
           two files (query + response) atomically and runs \
           trim_to(MAX_TURNS, TARGET_TURNS) to keep the directory \
           bounded; log failures are warned-and-swallowed so the HTTP \
           response is never blocked by housekeeping.",
}

#[derive(Clone)]
pub struct WatcherQueryState {
    pub watch_socket: Arc<PathBuf>,
    /// When set, every request/response pair is tee'd to this
    /// directory. None disables logging (the response still flows
    /// through the HTTP body).
    pub log: Option<Arc<WatcherLog>>,
}

#[derive(Clone)]
pub struct WatcherLogState {
    pub log: Arc<WatcherLog>,
    pub watch: Arc<DirectoryWatch>,
}

/// On-disk log of watcher queries. Each turn writes two files:
/// `<NNN>-query.json` (the request) and `<NNN+1>-response.json`
/// (the response). Atomic next-number allocation via O_CREAT|O_EXCL,
/// same shape as `crate::mailbox::Mailbox::append`.
pub struct WatcherLog {
    pub root: PathBuf,
}

/// Auto-trim ceiling: when the watcher-log contains more than this
/// many turns after a successful `append_turn`, the oldest turns are
/// pruned down to `WATCHER_LOG_TARGET_TURNS`. The 10-turn buffer
/// keeps trimming from happening on every single append once we're
/// near the cap — we let the directory grow to 100 then drop to 90,
/// rather than thrashing one delete per write at exactly 100.
pub const WATCHER_LOG_MAX_TURNS: usize = 100;
pub const WATCHER_LOG_TARGET_TURNS: usize = 90;

impl WatcherLog {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub async fn ensure(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.root)
            .await
            .with_context(|| format!("create watcher-log dir {}", self.root.display()))
    }

    /// Allocate the next pair of filenames atomically and write
    /// both. Returns the (query_filename, response_filename) pair
    /// so the response handler can include them in its log line.
    pub async fn append_turn(
        &self,
        request: &Value,
        response: &Value,
    ) -> Result<(String, String)> {
        // Recreate the dir if it disappeared between startup and this
        // call — common during dev when someone clears /tmp.
        tokio::fs::create_dir_all(&self.root)
            .await
            .with_context(|| format!("ensure watcher-log dir {}", self.root.display()))?;
        let req_pretty = serde_json::to_string_pretty(request)?;
        let res_pretty = serde_json::to_string_pretty(response)?;
        for offset in 0..16u64 {
            let n = next_number(&self.root).await? + offset;
            let q_id = format!("{n:03}");
            let r_id = format!("{:03}", n + 1);
            let q_name = format!("{q_id}-query.json");
            let r_name = format!("{r_id}-response.json");
            let q_path = self.root.join(&q_name);
            let r_path = self.root.join(&r_name);
            match write_exclusive(&q_path, &req_pretty).await {
                Ok(()) => {
                    write_exclusive(&r_path, &res_pretty)
                        .await
                        .with_context(|| format!("write response {}", r_path.display()))?;
                    // Prune oldest turns past the cap. Failures here
                    // do NOT fail the append — the turn is already on
                    // disk and observable; trimming is housekeeping.
                    if let Err(e) = self
                        .trim_to(WATCHER_LOG_MAX_TURNS, WATCHER_LOG_TARGET_TURNS)
                        .await
                    {
                        warn!(err = %e, "watcher-log trim failed");
                    }
                    return Ok((q_name, r_name));
                }
                Err(e) if is_already_exists(&e) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(anyhow!(
            "could not allocate next watcher-log filename after 16 retries"
        ))
    }

    /// If the directory holds more than `max_turns` turns, remove the
    /// oldest until only `target` remain. A "turn" is a `<N>-query.json`
    /// file paired with its `<N+1>-response.json`; we identify turns by
    /// the query files and remove both members of each victim pair.
    /// Returns the number of files removed (so removed/2 turns).
    pub async fn trim_to(&self, max_turns: usize, target: usize) -> Result<usize> {
        if target >= max_turns {
            return Ok(0);
        }
        let mut entries = match tokio::fs::read_dir(&self.root).await {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(e) => return Err(e.into()),
        };
        let mut by_n: std::collections::BTreeMap<u64, PathBuf> =
            std::collections::BTreeMap::new();
        while let Some(e) = entries.next_entry().await? {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            if let Some(n) = leading_number(&path) {
                by_n.insert(n, path);
            }
        }
        let query_turns: Vec<u64> = by_n
            .iter()
            .filter(|(_, p)| {
                p.file_name()
                    .and_then(|s| s.to_str())
                    .map(|n| n.contains("-query"))
                    .unwrap_or(false)
            })
            .map(|(n, _)| *n)
            .collect();
        if query_turns.len() <= max_turns {
            return Ok(0);
        }
        let to_remove = query_turns.len() - target;
        let mut removed = 0usize;
        for q_n in query_turns.iter().take(to_remove) {
            for n in [*q_n, q_n + 1] {
                if let Some(path) = by_n.get(&n) {
                    match tokio::fs::remove_file(path).await {
                        Ok(()) => removed += 1,
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                        Err(e) => warn!(err = %e, path = %path.display(), "trim remove failed"),
                    }
                }
            }
        }
        Ok(removed)
    }
}

fn leading_number(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?;
    let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

async fn next_number(root: &Path) -> Result<u64> {
    let mut entries = match tokio::fs::read_dir(root).await {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(1),
        Err(e) => return Err(e.into()),
    };
    let mut max = 0u64;
    while let Some(e) = entries.next_entry().await? {
        if let Some(n) = leading_number(&e.path()) {
            max = max.max(n);
        }
    }
    Ok(max + 1)
}

async fn write_exclusive(path: &Path, content: &str) -> Result<()> {
    use tokio::fs::OpenOptions;
    use tokio::io::AsyncWriteExt;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .await?;
    f.write_all(content.as_bytes()).await?;
    f.flush().await?;
    Ok(())
}

fn is_already_exists(err: &anyhow::Error) -> bool {
    err.downcast_ref::<std::io::Error>()
        .map(|e| e.kind() == std::io::ErrorKind::AlreadyExists)
        .unwrap_or(false)
}

// ── Listing endpoints ────────────────────────────────────────────────

/// One round-trip in the watcher log: a query file paired with the
/// next-numbered response file, both already parsed as JSON.
#[derive(Serialize)]
pub struct LogTurn {
    pub turn: u64,
    pub query_filename: String,
    pub response_filename: String,
    pub query: Value,
    pub response: Value,
}

#[derive(Serialize)]
struct LogEntries {
    turns: Vec<LogTurn>,
}

pub async fn list_entries(State(state): State<WatcherLogState>) -> impl IntoResponse {
    match read_log(&state.log).await {
        Ok(turns) => Json(LogEntries { turns }).into_response(),
        Err(e) => {
            warn!(err = %e, "watcher-log list failed");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn stream(
    State(state): State<WatcherLogState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.watch.events.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|item| async move {
        match item {
            Ok(DirectoryEvent { filename }) => {
                Some(Ok(Event::default().event("file").data(filename)))
            }
            Err(_) => None,
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}

async fn read_log(log: &WatcherLog) -> Result<Vec<LogTurn>> {
    let mut entries = match tokio::fs::read_dir(&log.root).await {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let mut by_n: std::collections::BTreeMap<u64, PathBuf> = std::collections::BTreeMap::new();
    while let Some(e) = entries.next_entry().await? {
        let path = e.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        if let Some(n) = leading_number(&path) {
            by_n.insert(n, path);
        }
    }

    // Pair adjacent (query, response) files: a turn is a query
    // numbered N where N+1 exists with the `-response` suffix.
    let mut turns: Vec<LogTurn> = Vec::new();
    let entries: Vec<(u64, PathBuf)> = by_n.into_iter().collect();
    let mut i = 0;
    while i + 1 < entries.len() {
        let (q_n, q_path) = &entries[i];
        let (r_n, r_path) = &entries[i + 1];
        let q_name = q_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let r_name = r_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if *r_n == q_n + 1 && q_name.contains("-query") && r_name.contains("-response") {
            let q_text = tokio::fs::read_to_string(q_path).await?;
            let r_text = tokio::fs::read_to_string(r_path).await?;
            turns.push(LogTurn {
                turn: *q_n,
                query_filename: q_name.to_string(),
                response_filename: r_name.to_string(),
                query: serde_json::from_str(&q_text).unwrap_or(Value::Null),
                response: serde_json::from_str(&r_text).unwrap_or(Value::Null),
            });
            i += 2;
        } else {
            i += 1;
        }
    }
    Ok(turns)
}

pub async fn query(
    State(state): State<WatcherQueryState>,
    Json(req): Json<Value>,
) -> impl IntoResponse {
    let line = match serde_json::to_string(&req) {
        Ok(s) => s,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, format!("encode request: {e}"))
                .into_response()
        }
    };

    let mut client = match WatchClient::connect(state.watch_socket.as_path()).await {
        Ok(c) => c,
        Err(e) => {
            warn!(err = %e, "watcher connect failed");
            return (
                StatusCode::BAD_GATEWAY,
                format!("cast-watch unreachable: {e}"),
            )
                .into_response();
        }
    };

    if let Err(e) = client.send_line(&line).await {
        warn!(err = %e, "watcher write failed");
        return (StatusCode::BAD_GATEWAY, format!("watcher write: {e}"))
            .into_response();
    }

    match client.read_line().await {
        Ok(Some(response_line)) => {
            // Parse the watcher's response as JSON and re-emit so
            // the HTTP body is a real JSON object rather than a
            // string-quoted line. If parsing fails (shouldn't —
            // watcher always emits valid JSON-line), fall back to
            // returning the raw text.
            let parsed = serde_json::from_str::<Value>(&response_line).ok();
            // Tee to disk if a watcher-log directory is configured.
            if let (Some(log), Some(value)) = (state.log.as_ref(), parsed.as_ref()) {
                if let Err(e) = log.append_turn(&req, value).await {
                    warn!(err = %e, "watcher-log append failed");
                }
            }
            match parsed {
                Some(value) => Json(value).into_response(),
                None => (
                    StatusCode::OK,
                    [(axum::http::header::CONTENT_TYPE, "application/json")],
                    response_line,
                )
                    .into_response(),
            }
        }
        Ok(None) => (
            StatusCode::BAD_GATEWAY,
            "cast-watch closed before responding",
        )
            .into_response(),
        Err(e) => {
            warn!(err = %e, "watcher read failed");
            (StatusCode::BAD_GATEWAY, format!("watcher read: {e}")).into_response()
        }
    }
}
