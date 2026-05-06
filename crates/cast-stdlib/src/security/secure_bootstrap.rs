//! `secure_bootstrap` — initial trust establishment is protected.

/// Sentinel for `secure_bootstrap`.
pub struct SecureBootstrap;

cast::concept! {
    name: "secure_bootstrap",
    summary: "Initial trust establishment is protected. The first key, \
              first identity, first authority arrive through a path \
              that the system itself can later defend; everything after \
              first trust inherits whatever it was.",
    anchors: [cast_stdlib::security::secure_bootstrap::SecureBootstrap],
    tags: ["cast_stdlib", "security"],
}
