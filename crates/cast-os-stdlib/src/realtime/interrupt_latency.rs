//! `interrupt_latency` — delay before interrupt handler runs.

/// Sentinel for `interrupt_latency`.
pub struct InterruptLatency;

cast::concept! {
    name: "interrupt_latency",
    summary: "delay before interrupt handler runs.",
    anchors: [cast_os_stdlib::realtime::interrupt_latency::InterruptLatency],
    tags: ["cast_os_stdlib", "realtime"],
}
