//! `hybrid_poll_interrupt_io` — mix polling and interrupts.

/// Sentinel for `hybrid_poll_interrupt_io`.
pub struct HybridPollInterruptIo;

cast::concept! {
    name: "hybrid_poll_interrupt_io",
    summary: "mix polling and interrupts.",
    anchors: [cast_os_stdlib::io_architecture::hybrid_poll_interrupt_io::HybridPollInterruptIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
