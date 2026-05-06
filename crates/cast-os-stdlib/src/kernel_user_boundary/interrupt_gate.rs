//! `interrupt_gate` — privileged entry for interrupt handling.

/// Sentinel for `interrupt_gate`.
pub struct InterruptGate;

cast::concept! {
    name: "interrupt_gate",
    summary: "privileged entry for interrupt handling.",
    anchors: [cast_os_stdlib::kernel_user_boundary::interrupt_gate::InterruptGate],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
