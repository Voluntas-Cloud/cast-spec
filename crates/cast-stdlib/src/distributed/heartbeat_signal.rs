//! `heartbeat_signal` — periodic liveness indication.

/// Sentinel for `heartbeat_signal`.
pub struct HeartbeatSignal;

cast::concept! {
    name: "heartbeat_signal",
    summary: "Periodic liveness indication. Absence of heartbeats — \
              not the presence of negative news — is what tells the \
              cluster a node is gone.",
    anchors: [cast_stdlib::distributed::heartbeat_signal::HeartbeatSignal],
    tags: ["cast_stdlib", "distributed"],
}
