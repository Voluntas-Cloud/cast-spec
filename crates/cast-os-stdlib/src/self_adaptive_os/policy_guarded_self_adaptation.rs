//! `policy_guarded_self_adaptation` — adaptation constrained by explicit policy.

/// Sentinel for `policy_guarded_self_adaptation`.
pub struct PolicyGuardedSelfAdaptation;

cast::concept! {
    name: "policy_guarded_self_adaptation",
    summary: "adaptation constrained by explicit policy.",
    anchors: [cast_os_stdlib::self_adaptive_os::policy_guarded_self_adaptation::PolicyGuardedSelfAdaptation],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
