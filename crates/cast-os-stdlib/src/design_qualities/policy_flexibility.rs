//! `policy_flexibility` — behavior configurable without rewriting mechanisms.

/// Sentinel for `policy_flexibility`.
pub struct PolicyFlexibility;

cast::concept! {
    name: "policy_flexibility",
    summary: "behavior configurable without rewriting mechanisms.",
    anchors: [cast_os_stdlib::design_qualities::policy_flexibility::PolicyFlexibility],
    tags: ["cast_os_stdlib", "design_qualities"],
}
