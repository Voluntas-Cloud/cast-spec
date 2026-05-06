//! `oom_recovery` — recover from memory exhaustion.

/// Sentinel for `oom_recovery`.
pub struct OomRecovery;

cast::concept! {
    name: "oom_recovery",
    summary: "recover from memory exhaustion.",
    anchors: [cast_os_stdlib::fault_recovery::oom_recovery::OomRecovery],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
