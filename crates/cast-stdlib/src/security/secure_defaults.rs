//! `secure_defaults` — safe behavior without configuration.

/// Sentinel for `secure_defaults`.
pub struct SecureDefaults;

cast::concept! {
    name: "secure_defaults",
    summary: "Safe behavior without configuration. The unconfigured \
              system is locked down — no anonymous access, no debug \
              endpoints, conservative crypto. Operators relax it on \
              purpose, never by accident.",
    anchors: [cast_stdlib::security::secure_defaults::SecureDefaults],
    tags: ["cast_stdlib", "security"],
}
