//! `thread_control_block` — core thread metadata structure.

/// Sentinel for `thread_control_block`.
pub struct ThreadControlBlock;

cast::concept! {
    name: "thread_control_block",
    summary: "core thread metadata structure.",
    anchors: [cast_os_stdlib::execution_model::thread_control_block::ThreadControlBlock],
    tags: ["cast_os_stdlib", "execution_model"],
}
