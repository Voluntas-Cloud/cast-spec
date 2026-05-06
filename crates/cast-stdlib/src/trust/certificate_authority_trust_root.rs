//! `certificate_authority_trust_root` — the PKI anchor.

/// Sentinel for `certificate_authority_trust_root`.
pub struct CertificateAuthorityTrustRoot;

cast::concept! {
    name: "certificate_authority_trust_root",
    summary: "Trusted root used to validate certificates. Anchors the \
              entire PKI chain; compromise of the root invalidates \
              every leaf.",
    anchors: [cast_stdlib::trust::certificate_authority_trust_root::CertificateAuthorityTrustRoot],
    tags: ["cast_stdlib", "trust"],
}
