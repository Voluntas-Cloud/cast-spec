//! The Rust shim half of cast-web. An axum HTTP server that hosts
//! three surfaces:
//!
//! - `/ws` — one cast-watch unix socket connection per WebSocket
//!   client. JSON lines pass through verbatim in both directions; the
//!   shim does no schema interpretation.
//! - `/mailbox/*` — POST `/mailbox/send` to append the next-numbered
//!   markdown file; GET `/mailbox/stream` to tail the directory over
//!   Server-Sent Events.
//! - `/static/*` — serve the embedded compiled-from-TypeScript bundle.
//!
//! The shim never reads the cast graph itself; it only relays bytes.
//! See the `cast::rule!` in `crate` (lib.rs) for why this matters.

cast::concept! {
    name: "cast_web_shim",
    summary: "Axum router that fronts cast-watch over WebSocket and \
              over a one-shot HTTP query endpoint, serves a directory \
              of markdown messages over POST/SSE, and serves the \
              embedded TypeScript bundle. The WS handler holds one \
              unix-socket connection per client for the connection's \
              lifetime; reconnects on the browser side establish a \
              new socket connection. `POST /watcher/query` is the \
              one-shot peer of `/ws`: short-lived connection per \
              call, designed for ad-hoc tooling like `dev/cwq.sh`. \
              The shim does no JSON interpretation — the watcher's \
              protocol changes do not require shim changes.",
    anchors: [
        crate::shim::ws_server,
        crate::shim::watch_client,
        crate::shim::http_server,
        crate::shim::watcher_routes,
    ],
    tags: ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::integration::protocol_bridge::ProtocolBridge,
    concept: "cast_web_shim",
    why:     "Bridges browser-facing WebSocket/HTTP frames into the \
              JSON-line Unix-socket protocol cast-watch speaks. \
              Callers on either side use their native protocol; the \
              shim owns the impedance mismatch so neither end has to \
              learn the other's wire format.",
    tags:    ["cast_web"],
}

// Two surfaces front the same cast-watch JSON-line socket. The
// difference is connection lifecycle: WebSocket holds one Unix-
// socket connection per browser tab for the tab's lifetime; the
// HTTP query opens a fresh connection per call. Both pass bytes
// through verbatim.
cast::compare! {
    long_lived @ crate::shim::ws_server::ws_handler:
        "Connection lifecycle is per-tab. The browser opens `/ws` and \
         the shim holds one Unix-socket connection to cast-watch for \
         the WS's entire lifetime. Subscribe responses arrive pushed; \
         reconnect is the browser's responsibility. Designed for the \
         long-running UI panel where the cost of the unix-socket \
         connect amortizes over many requests.",
    one_shot @ crate::shim::watcher_routes::query:
        "Connection lifecycle is per-call. An HTTP POST opens a fresh \
         Unix-socket connection, sends one request, reads one \
         response, closes. No subscribe semantics. Designed for \
         ad-hoc tooling like `dev/cwq.sh`, the assistant query path, \
         and any caller that doesn't want to hold a connection.",
    tags: ["cast_web"],
    note: "Both surfaces speak the watcher's JSON-line protocol \
           verbatim — the shim does no schema interpretation, so a \
           protocol change in cast-watch needs no shim change. The \
           watcher-log tee runs only on the one-shot path: the WS \
           bridge is per-tab and per-tab observability is the \
           browser's responsibility.",
}

pub mod http_server;
pub mod mailbox_routes;
pub mod watch_client;
pub mod watcher_routes;
pub mod ws_server;
