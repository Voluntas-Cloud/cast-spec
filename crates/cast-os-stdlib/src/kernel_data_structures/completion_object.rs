//! `completion_object` — one-time event synchronization.

/// Sentinel for `completion_object`.
pub struct CompletionObject;

cast::concept! {
    name: "completion_object",
    summary: "one-time event synchronization.",
    anchors: [cast_os_stdlib::kernel_data_structures::completion_object::CompletionObject],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
