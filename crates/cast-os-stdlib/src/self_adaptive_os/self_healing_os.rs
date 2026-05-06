//! `self_healing_os` — OS detects and recovers from faults.

/// Sentinel for `self_healing_os`.
pub struct SelfHealingOs;

cast::concept! {
    name: "self_healing_os",
    summary: "OS detects and recovers from faults.",
    anchors: [cast_os_stdlib::self_adaptive_os::self_healing_os::SelfHealingOs],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
