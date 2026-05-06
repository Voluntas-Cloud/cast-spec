//! `mutual_tls_identity` — both sides prove identity at TLS handshake.

/// Sentinel for `mutual_tls_identity`.
pub struct MutualTlsIdentity;

cast::concept! {
    name: "mutual_tls_identity",
    summary: "Both client and server prove identity with certificates. \
              Identity is bound to the TLS handshake; no separate \
              authentication step.",
    anchors: [cast_stdlib::trust::mutual_tls_identity::MutualTlsIdentity],
    tags: ["cast_stdlib", "trust"],
}
