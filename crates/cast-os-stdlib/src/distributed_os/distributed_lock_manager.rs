//! `distributed_lock_manager` — locks across machines.

/// Sentinel for `distributed_lock_manager`.
pub struct DistributedLockManager;

cast::concept! {
    name: "distributed_lock_manager",
    summary: "locks across machines.",
    anchors: [cast_os_stdlib::distributed_os::distributed_lock_manager::DistributedLockManager],
    tags: ["cast_os_stdlib", "distributed_os"],
}
