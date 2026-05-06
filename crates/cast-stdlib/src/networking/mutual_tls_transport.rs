//! `mutual_tls_transport` — encrypted authenticated transport.

/// Sentinel for `mutual_tls_transport`.
pub struct MutualTlsTransport;

cast::concept! {
    name: "mutual_tls_transport",
    summary: "Encrypted authenticated transport. Both ends present \
              certificates; identity is bound to the connection itself \
              instead of a bearer token carried inside it.",
    anchors: [cast_stdlib::networking::mutual_tls_transport::MutualTlsTransport],
    tags: ["cast_stdlib", "networking"],
}
