//! `safe_mode_recovery` — start minimal system for repair.

/// Sentinel for `safe_mode_recovery`.
pub struct SafeModeRecovery;

cast::concept! {
    name: "safe_mode_recovery",
    summary: "start minimal system for repair.",
    anchors: [cast_os_stdlib::fault_recovery::safe_mode_recovery::SafeModeRecovery],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
