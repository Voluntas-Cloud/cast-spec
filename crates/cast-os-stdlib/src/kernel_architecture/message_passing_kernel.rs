//! `message_passing_kernel` — kernel architecture centered around IPC messages.

/// Sentinel for `message_passing_kernel`.
pub struct MessagePassingKernel;

cast::concept! {
    name: "message_passing_kernel",
    summary: "kernel architecture centered around IPC messages.",
    anchors: [cast_os_stdlib::kernel_architecture::message_passing_kernel::MessagePassingKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
