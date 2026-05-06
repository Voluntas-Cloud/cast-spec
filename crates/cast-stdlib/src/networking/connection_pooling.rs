//! `connection_pooling` — reuse network connections.

/// Sentinel for `connection_pooling`.
pub struct ConnectionPooling;

cast::concept! {
    name: "connection_pooling",
    summary: "Reuse network connections. Avoid per-request handshake \
              cost; the pool's max-size, idle-timeout, and acquisition \
              ordering decide tail latency more than the protocol does.",
    anchors: [cast_stdlib::networking::connection_pooling::ConnectionPooling],
    tags: ["cast_stdlib", "networking"],
}
