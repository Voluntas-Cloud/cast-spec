//! `context_switch_boundary` — transition between running execution contexts.

/// Sentinel for `context_switch_boundary`.
pub struct ContextSwitchBoundary;

cast::concept! {
    name: "context_switch_boundary",
    summary: "transition between running execution contexts.",
    anchors: [cast_os_stdlib::kernel_user_boundary::context_switch_boundary::ContextSwitchBoundary],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
