//! `interrupt_handler` — driver code responding to hardware interrupt.

/// Sentinel for `interrupt_handler`.
pub struct InterruptHandler;

cast::concept! {
    name: "interrupt_handler",
    summary: "driver code responding to hardware interrupt.",
    anchors: [cast_os_stdlib::driver_model::interrupt_handler::InterruptHandler],
    tags: ["cast_os_stdlib", "driver_model"],
}
