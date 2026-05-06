//! `deny_by_default` — absence of permission means no.

/// Sentinel for `deny_by_default`.
pub struct DenyByDefault;

cast::concept! {
    name: "deny_by_default",
    summary: "Absence of permission means no. Adding a new resource \
              or action does not implicitly grant access; explicit \
              allow rules are required.",
    anchors: [cast_stdlib::config::deny_by_default::DenyByDefault],
    tags: ["cast_stdlib", "config"],
}
