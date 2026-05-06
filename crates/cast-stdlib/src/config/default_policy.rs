//! `default_policy` — behavior when no explicit policy exists.

/// Sentinel for `default_policy`.
pub struct DefaultPolicy;

cast::concept! {
    name: "default_policy",
    summary: "Behavior when no explicit policy exists. A real default, \
              not 'undefined'; ideally the safe default, so missing \
              policy is never the worst case.",
    anchors: [cast_stdlib::config::default_policy::DefaultPolicy],
    tags: ["cast_stdlib", "config"],
}
