//! `policy_engine_boundary` — policy decisions isolated from business logic.

/// Sentinel for `policy_engine_boundary`.
pub struct PolicyEngineBoundary;

cast::concept! {
    name: "policy_engine_boundary",
    summary: "Policy decisions isolated from business logic. Business \
              code asks 'is this allowed?'; the policy engine answers.",
    anchors: [cast_stdlib::architecture::policy_engine_boundary::PolicyEngineBoundary],
    tags: ["cast_stdlib", "architecture"],
}
