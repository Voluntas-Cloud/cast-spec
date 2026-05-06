//! `tls_termination` — decrypt TLS at edge/proxy.

/// Sentinel for `tls_termination`.
pub struct TlsTermination;

cast::concept! {
    name: "tls_termination",
    summary: "Decrypt TLS at the edge or proxy. Beyond the termination \
              point the bytes are clear-text on whatever transport \
              follows; that hop is part of the threat model whether you \
              wrote it down or not.",
    anchors: [cast_stdlib::networking::tls_termination::TlsTermination],
    tags: ["cast_stdlib", "networking"],
}
