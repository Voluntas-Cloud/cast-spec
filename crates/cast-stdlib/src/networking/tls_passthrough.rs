//! `tls_passthrough` — proxy without decrypting TLS.

/// Sentinel for `tls_passthrough`.
pub struct TlsPassthrough;

cast::concept! {
    name: "tls_passthrough",
    summary: "Proxy without decrypting TLS. The handshake reaches the \
              backend; the proxy can route on SNI but cannot inspect \
              payload, log application data, or enforce per-request \
              policy.",
    anchors: [cast_stdlib::networking::tls_passthrough::TlsPassthrough],
    tags: ["cast_stdlib", "networking"],
}
