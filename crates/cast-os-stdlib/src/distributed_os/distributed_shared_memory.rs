//! `distributed_shared_memory` — memory abstraction across machines.

/// Sentinel for `distributed_shared_memory`.
pub struct DistributedSharedMemory;

cast::concept! {
    name: "distributed_shared_memory",
    summary: "memory abstraction across machines.",
    anchors: [cast_os_stdlib::distributed_os::distributed_shared_memory::DistributedSharedMemory],
    tags: ["cast_os_stdlib", "distributed_os"],
}
