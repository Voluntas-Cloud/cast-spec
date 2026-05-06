//! `credential_revocation` — invalidate compromised credentials.

/// Sentinel for `credential_revocation`.
pub struct CredentialRevocation;

cast::concept! {
    name: "credential_revocation",
    summary: "Invalidate compromised credentials. There is a fast path \
              to make a token, key, or session unusable; the time \
              between detection and global rejection is part of the \
              security posture.",
    anchors: [cast_stdlib::security::credential_revocation::CredentialRevocation],
    tags: ["cast_stdlib", "security"],
}
