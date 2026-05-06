//! `override_policy` — scoped exception to normal policy.

/// Sentinel for `override_policy`.
pub struct OverridePolicy;

cast::concept! {
    name: "override_policy",
    summary: "Scoped exception to normal policy. Documented, time-bound, \
              and audit-logged; otherwise overrides become silent \
              shadow policy.",
    anchors: [cast_stdlib::config::override_policy::OverridePolicy],
    tags: ["cast_stdlib", "config"],
}
