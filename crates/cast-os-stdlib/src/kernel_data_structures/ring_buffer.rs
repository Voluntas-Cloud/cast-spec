//! `ring_buffer` — circular buffer for logs/events/I/O.

/// Sentinel for `ring_buffer`.
pub struct RingBuffer;

cast::concept! {
    name: "ring_buffer",
    summary: "circular buffer for logs/events/I/O.",
    anchors: [cast_os_stdlib::kernel_data_structures::ring_buffer::RingBuffer],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
