//! `sealed_secret_management_system` — secrets are encrypted, scoped, rotated, injected, and audited.

/// Sentinel for `sealed_secret_management_system`.
pub struct SealedSecretManagementSystem;

cast::concept! {
    name: "sealed_secret_management_system",
    summary: "Secrets are stored encrypted, scoped, rotated, \
              injected, and audited. Composes secret_scoping, \
              secret_rotation, secret_injection, \
              encrypted_at_rest_storage, hardware_backed_key, \
              envelope_encryption, credential_revocation, and \
              audit_log. Used for API keys, database credentials, \
              TLS keys, mobile private keys, and cluster service \
              secrets.",
    anchors: [cast_stdlib::patterns::sealed_secret_management_system::SealedSecretManagementSystem],
    tags: ["cast_stdlib", "patterns"],
}
