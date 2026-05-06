//! `soft_lockup_detection` — detect CPU stuck in kernel.

/// Sentinel for `soft_lockup_detection`.
pub struct SoftLockupDetection;

cast::concept! {
    name: "soft_lockup_detection",
    summary: "detect CPU stuck in kernel.",
    anchors: [cast_os_stdlib::fault_recovery::soft_lockup_detection::SoftLockupDetection],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
