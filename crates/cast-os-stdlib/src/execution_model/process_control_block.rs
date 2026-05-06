//! `process_control_block` — core process metadata structure.

/// Sentinel for `process_control_block`.
pub struct ProcessControlBlock;

cast::concept! {
    name: "process_control_block",
    summary: "core process metadata structure.",
    anchors: [cast_os_stdlib::execution_model::process_control_block::ProcessControlBlock],
    tags: ["cast_os_stdlib", "execution_model"],
}
