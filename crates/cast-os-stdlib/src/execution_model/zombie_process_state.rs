//! `zombie_process_state` — exited process waiting to be reaped.

/// Sentinel for `zombie_process_state`.
pub struct ZombieProcessState;

cast::concept! {
    name: "zombie_process_state",
    summary: "exited process waiting to be reaped.",
    anchors: [cast_os_stdlib::execution_model::zombie_process_state::ZombieProcessState],
    tags: ["cast_os_stdlib", "execution_model"],
}
