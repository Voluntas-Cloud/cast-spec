//! `hard_lockup_detection` — detect CPU not handling interrupts.

/// Sentinel for `hard_lockup_detection`.
pub struct HardLockupDetection;

cast::concept! {
    name: "hard_lockup_detection",
    summary: "detect CPU not handling interrupts.",
    anchors: [cast_os_stdlib::fault_recovery::hard_lockup_detection::HardLockupDetection],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
