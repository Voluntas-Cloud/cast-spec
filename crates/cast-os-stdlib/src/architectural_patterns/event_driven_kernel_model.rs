//! `event_driven_kernel_model` — kernel subsystems respond to events.

/// Sentinel for `event_driven_kernel_model`.
pub struct EventDrivenKernelModel;

cast::concept! {
    name: "event_driven_kernel_model",
    summary: "kernel subsystems respond to events.",
    anchors: [cast_os_stdlib::architectural_patterns::event_driven_kernel_model::EventDrivenKernelModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
