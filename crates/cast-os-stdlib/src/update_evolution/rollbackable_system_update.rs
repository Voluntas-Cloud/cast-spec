//! `rollbackable_system_update` — revert to previous OS state.

/// Sentinel for `rollbackable_system_update`.
pub struct RollbackableSystemUpdate;

cast::concept! {
    name: "rollbackable_system_update",
    summary: "revert to previous OS state.",
    anchors: [cast_os_stdlib::update_evolution::rollbackable_system_update::RollbackableSystemUpdate],
    tags: ["cast_os_stdlib", "update_evolution"],
}
