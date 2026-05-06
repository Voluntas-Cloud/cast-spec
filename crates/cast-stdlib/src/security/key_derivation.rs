//! `key_derivation` — derive keys from secret material safely.

/// Sentinel for `key_derivation`.
pub struct KeyDerivation;

cast::concept! {
    name: "key_derivation",
    summary: "Derive keys from secret material safely. A KDF turns one \
              secret into many purpose-bound keys with domain \
              separation; ad-hoc \"hash the password\" schemes always \
              get this wrong somewhere.",
    anchors: [cast_stdlib::security::key_derivation::KeyDerivation],
    tags: ["cast_stdlib", "security"],
}
