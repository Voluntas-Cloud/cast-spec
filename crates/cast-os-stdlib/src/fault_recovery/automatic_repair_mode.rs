//! `automatic_repair_mode` — OS attempts self-repair.

/// Sentinel for `automatic_repair_mode`.
pub struct AutomaticRepairMode;

cast::concept! {
    name: "automatic_repair_mode",
    summary: "OS attempts self-repair.",
    anchors: [cast_os_stdlib::fault_recovery::automatic_repair_mode::AutomaticRepairMode],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
