//! `policy_as_code` — rules are versioned and testable.

/// Sentinel for `policy_as_code`.
pub struct PolicyAsCode;

cast::concept! {
    name: "policy_as_code",
    summary: "Rules are versioned and testable. Policy expressed in a \
              language with code review, tests, and a CI pipeline — \
              not in a wiki page someone updates on Tuesdays.",
    anchors: [cast_stdlib::config::policy_as_code::PolicyAsCode],
    tags: ["cast_stdlib", "config"],
}
