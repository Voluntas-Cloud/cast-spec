//! `tcp_proxying` — forward raw TCP connections.

/// Sentinel for `tcp_proxying`.
pub struct TcpProxying;

cast::concept! {
    name: "tcp_proxying",
    summary: "Forward raw TCP connections. The proxy operates below the \
              application protocol; it can route by SNI or destination \
              but does not understand the bytes flowing through.",
    anchors: [cast_stdlib::networking::tcp_proxying::TcpProxying],
    tags: ["cast_stdlib", "networking"],
}
