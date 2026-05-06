//! Query socket — the LLM-facing surface.
//!
//! JSON-lines over a Unix domain socket. One request per line, one
//! response per line. Connections that subscribe also receive
//! broadcast notifications when the snapshot changes.
//!
//! Modeled on a JSON-lines control-socket pattern common to long-
//! running daemons — request/response on the same line, broadcasts
//! interleaved on subscribed connections.

pub mod dispatch;
pub mod filter;
pub mod format;
pub mod help_data;
pub mod protocol;
pub mod query;
pub mod walk;

use std::sync::Arc;

use serde::Serialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

use crate::state::{self, LiveState};

// Re-export everything that was pub in the flat socket module.
pub use dispatch::{
    concepts_for_path_body, dispatch_request, graph_for_tag_body, handle_rebuild,
    rules_for_path_body, snapshot_body, status_body, tree_body, tree_expand_body,
    unresolved_in_file_body,
};
pub use format::render_with_format;
pub use help_data::{help_body, manual_body};
pub use protocol::{
    Broadcast, BroadcastKind, Dim, HelpEntry, HelpField, IoBridgeHit, ManualField, ManualLanguage,
    ManualMacro, ManualSpecSource, ManualWarningKind, ManualWorkflow, OutputFormat, PathHit,
    QueryPredicate, Request, Response, ResponseBody, Stream, WalkEdge, PROTOCOL_VERSION,
    default_hops,
};
pub use query::{
    handle_query_flashlight, handle_query_select, handle_query_serialize, handle_query_walk,
    widen_concept_set, QueryStage, SelectedRecords, WalkOutcome,
};
pub use walk::{handle_jump, is_external_lang, walk_io_into_foreign_target};

cast::concept! {
    name: "connection_lifecycle",
    summary: "Each accepted connection is a single async task: read a \
              JSON line, parse to Request, dispatch to its handler, \
              write the JSON-encoded Response, loop. A `Subscribe` \
              request upgrades the connection so subsequent broadcasts \
              arrive interleaved with future responses on the same \
              socket.",
    anchors: [
        crate::socket::QueryServer,
        crate::socket::handle_connection,
        CAST::AS_PRIMITIVE::crate::socket::fanout,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::socket::fanout,
    concept: "connection_lifecycle",
    why:     "connection_lifecycle continues into the broadcast pump at fanout()",
}

cast::continues_in! {
    target:  cast_stdlib::architecture::task_per_connection::TaskPerConnection,
    concept: "connection_lifecycle",
    why:     "QueryServer's accept loop spawns one async task per \
              connection (handle_connection); each task owns its \
              socket for the connection's lifetime, runs its protocol \
              loop, and exits when the peer disconnects. Per-\
              connection isolation: a slow or misbehaving client \
              affects only its own task.",
    tags:    ["cast_watch"],
}

cast::concept! {
    name: "subscription_fanout",
    summary: "Broadcast snapshot/rebuild events to every subscribed \
              connection via a tokio broadcast channel. Each subscriber \
              owns its own Receiver; if a subscriber lags past the \
              channel capacity the receiver returns Lagged and the \
              subscriber is dropped (with a tracing warn) rather than \
              block the snapshot writer.",
    anchors: [
        crate::socket::QueryServer,
        crate::socket::Broadcast,
        crate::socket::fanout,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::storage::ring_buffer::RingBuffer,
    concept: "subscription_fanout",
    why:     "Each subscriber's tokio broadcast Receiver is backed by \
              a fixed-capacity ring: the channel keeps the most-recent \
              N broadcasts and a Receiver that lags past N is dropped \
              with `Lagged` rather than allowed to block the producer. \
              Bounded memory per subscriber, regardless of how slow \
              the subscriber is.",
    tags:    ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::messaging::publish_subscribe::PublishSubscribe,
    concept: "subscription_fanout",
    why:     "Classic publish-subscribe: the snapshot writer publishes \
              one Broadcast; every subscribed Receiver consumes it \
              independently. The producer doesn't know who's listening \
              and never coordinates with subscribers.",
    tags:    ["cast_watch"],
}

cast::rule! {
    rule: "Fanout never blocks the producer. A subscriber whose \
           channel is full is dropped (its events lost, a warning \
           logged) — never queued in unbounded memory and never \
           forced to block the commit path.",
    why:  "Snapshot commits must be O(1) in the number of \
           subscribers and O(1) in subscriber speed. A slow LLM \
           client (or a half-disconnected socket whose other end \
           stopped reading) cannot be allowed to back-pressure the \
           snapshot writer or queue unbounded memory while waiting.",
    governs: [
        crate::socket::fanout,
    ],
    tags: ["cast_watch"],
}

cast::anti_pattern! {
    avoid:      "Sharing subscription state through a Mutex<Vec<Sender>> \
                 that QueryServer locks to deliver each broadcast.",
    why:        "A naive shared-mutex registry serializes broadcasts \
                 across all subscribers and ties the commit path to \
                 lock contention. It also makes 'drop a slow \
                 subscriber' awkward — the producer would have to \
                 mutate the registry while iterating it.",
    instead:    "Use a tokio broadcast channel (or per-subscriber \
                 bounded mpsc) so the producer's send is lock-free \
                 and per-subscriber backpressure is handled by the \
                 channel's `try_send` semantics.",
    instead_at: crate::socket::fanout,
    governs: [
        crate::socket::fanout,
        crate::socket::QueryServer,
    ],
    tags: ["cast_watch"],
}

/// The query server. Owns the listener, accepts connections, dispatches
/// each line to the appropriate handler.
pub struct QueryServer {
    _private: (),
}

/// Build a fresh QueryServer. Stateless today — kept so the
/// `cast_watch_daemon` anchor stays meaningful and so that future
/// per-server config has a home.
pub fn new() -> QueryServer {
    QueryServer { _private: () }
}

/// Bind the socket and accept loop. Returns when the listener errors.
pub async fn serve(
    socket_path: &std::path::Path,
    state: Arc<LiveState>,
) -> anyhow::Result<()> {
    let _ = std::fs::remove_file(socket_path);
    if let Some(parent) = socket_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }
    let listener = UnixListener::bind(socket_path)?;
    tracing::info!(path = %socket_path.display(), "cast-watch query socket bound");
    loop {
        let (stream, _addr) = listener.accept().await?;
        let st = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, st).await {
                tracing::warn!(error = %e, "connection ended");
            }
        });
    }
}

/// Per-connection async loop. Read line → parse Request → dispatch →
/// write Response. A `Subscribe` request upgrades the connection so
/// subsequent broadcasts arrive on the same socket.
pub async fn handle_connection(
    stream: UnixStream,
    state: Arc<LiveState>,
) -> anyhow::Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half).lines();
    let mut subscribed: Option<tokio::sync::broadcast::Receiver<Broadcast>> = None;

    loop {
        tokio::select! {
            line = reader.next_line() => {
                let line = match line? {
                    Some(l) => l,
                    None => break,
                };
                if line.trim().is_empty() {
                    continue;
                }
                let response = match serde_json::from_str::<Request>(&line) {
                    Ok(req) => {
                        let is_subscribe = matches!(req, Request::Subscribe);
                        let resp = dispatch_request(&state, req).await;
                        if is_subscribe && subscribed.is_none() {
                            subscribed = Some(state.broadcast.subscribe());
                        }
                        resp
                    }
                    Err(e) => Response {
                        snapshot_generation: state::current(&state).generation,
                        body: ResponseBody::Error {
                            message: format!("invalid request: {e}"),
                        },
                    },
                };
                write_json(&mut write_half, &response).await?;
            }
            maybe_bcast = recv_or_pending(&mut subscribed) => {
                match maybe_bcast {
                    Ok(b) => write_json(&mut write_half, &b).await?,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!(missed = n, "subscriber lagged; dropping");
                        subscribed = None;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        subscribed = None;
                    }
                }
            }
        }
    }
    Ok(())
}

/// Helper: receive from the broadcast subscription if present, else
/// return a future that never completes. Lets `tokio::select!` ignore
/// the subscription branch until `Subscribe` upgrades the connection.
async fn recv_or_pending(
    rx: &mut Option<tokio::sync::broadcast::Receiver<Broadcast>>,
) -> Result<Broadcast, tokio::sync::broadcast::error::RecvError> {
    match rx {
        Some(r) => r.recv().await,
        None => std::future::pending().await,
    }
}

/// Send a Broadcast to every subscriber. Non-blocking — see the
/// `subscription_fanout` rule and anti-pattern.
pub fn fanout(broadcast: Broadcast, sender: &tokio::sync::broadcast::Sender<Broadcast>) {
    let _ = sender.send(broadcast);
}

async fn write_json<T: Serialize>(
    stream: &mut tokio::net::unix::OwnedWriteHalf,
    value: &T,
) -> anyhow::Result<()> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    stream.write_all(&bytes).await?;
    stream.flush().await?;
    Ok(())
}

cast::concept! {
    name: "query_socket_server",
    summary: "Unix-domain-socket accept loop. Binds the socket, \
              accepts connections, hands each off to a connection \
              task that reads JSON-line requests and writes \
              JSON-line responses.",
    anchors: [
        crate::socket::QueryServer,
        crate::socket::new,
        crate::socket::serve,
        crate::socket::handle_connection,
        crate::socket::write_json,
        crate::socket::fanout,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "query_socket_server",
    why: "Socket I/O, async scheduling, broadcast-channel timing — \
          all observable external state.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sentinel_type,
    concept: "query_socket_server",
    why: "QueryServer is a unit struct (`_private: ()`) — zero-sized \
          marker today; placeholder for future config.",
}

cast::continues_in! {
    target: cast_stdlib::messaging::request_response,
    concept: "query_socket_server",
    why: lazy,
}
