//! `tunnel_transport` — carry traffic through another connection.

/// Sentinel for `tunnel_transport`.
pub struct TunnelTransport;

cast::concept! {
    name: "tunnel_transport",
    summary: "Carry traffic through another connection. An outer \
              transport (TLS, WireGuard, SSH) wraps an inner protocol; \
              the tunnel's authentication is what actually controls \
              who can reach the inner network.",
    anchors: [cast_stdlib::networking::tunnel_transport::TunnelTransport],
    tags: ["cast_stdlib", "networking"],
}
