//! `shared_memory_ipc` — processes communicate via shared memory.

/// Sentinel for `shared_memory_ipc`.
pub struct SharedMemoryIpc;

cast::concept! {
    name: "shared_memory_ipc",
    summary: "processes communicate via shared memory.",
    anchors: [cast_os_stdlib::ipc::shared_memory_ipc::SharedMemoryIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
