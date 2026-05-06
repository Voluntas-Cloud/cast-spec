//! `rollback_update` — revert failed OS update.

/// Sentinel for `rollback_update`.
pub struct RollbackUpdate;

cast::concept! {
    name: "rollback_update",
    summary: "revert failed OS update.",
    anchors: [cast_os_stdlib::fault_recovery::rollback_update::RollbackUpdate],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
