//! `snapshot_vm_state` — capture VM state.

/// Sentinel for `snapshot_vm_state`.
pub struct SnapshotVmState;

cast::concept! {
    name: "snapshot_vm_state",
    summary: "capture VM state.",
    anchors: [cast_os_stdlib::virtualization::snapshot_vm_state::SnapshotVmState],
    tags: ["cast_os_stdlib", "virtualization"],
}
