//! `interrupt_affinity_control` — place interrupts on selected CPUs.

/// Sentinel for `interrupt_affinity_control`.
pub struct InterruptAffinityControl;

cast::concept! {
    name: "interrupt_affinity_control",
    summary: "place interrupts on selected CPUs.",
    anchors: [cast_os_stdlib::realtime::interrupt_affinity_control::InterruptAffinityControl],
    tags: ["cast_os_stdlib", "realtime"],
}
