//! `checkpoint_restore_process` — save and restore process state.

/// Sentinel for `checkpoint_restore_process`.
pub struct CheckpointRestoreProcess;

cast::concept! {
    name: "checkpoint_restore_process",
    summary: "save and restore process state.",
    anchors: [cast_os_stdlib::fault_recovery::checkpoint_restore_process::CheckpointRestoreProcess],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
