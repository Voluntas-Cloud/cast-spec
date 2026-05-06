//! `revocable_credential` — invalidatable token, server-side revocation list.

/// Sentinel for `revocable_credential`.
pub struct RevocableCredential;

cast::concept! {
    name: "revocable_credential",
    summary: "Credential can be invalidated. Requires server-side \
              state tracking which credentials have been revoked.",
    anchors: [cast_stdlib::trust::revocable_credential::RevocableCredential],
    tags: ["cast_stdlib", "trust"],
}
