//! `push_based_interrupt_model` — devices notify OS.

/// Sentinel for `push_based_interrupt_model`.
pub struct PushBasedInterruptModel;

cast::concept! {
    name: "push_based_interrupt_model",
    summary: "devices notify OS.",
    anchors: [cast_os_stdlib::architectural_patterns::push_based_interrupt_model::PushBasedInterruptModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
