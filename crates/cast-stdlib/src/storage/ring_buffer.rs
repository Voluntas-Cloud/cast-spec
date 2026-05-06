//! `ring_buffer` — fixed-capacity in-memory circular buffer with wrap-around indices.

/// Sentinel for `ring_buffer`.
pub struct RingBuffer;

cast::concept! {
    name: "ring_buffer",
    summary: "Fixed-capacity in-memory queue with two indices (head, \
              tail) that wrap around modulo capacity. Memory is \
              allocated once at construction; pushes and pops are \
              O(1) and lock-free in the single-producer/single-\
              consumer variant. When full, the oldest entry is \
              overwritten (overwrite mode) or the push fails (block \
              mode). Distinct from `bounded_log` (disk-backed, \
              count-bounded with hysteresis) — ring_buffer is in-RAM, \
              fixed-size, and oldest-overwritten.",
    anchors: [cast_stdlib::storage::ring_buffer::RingBuffer],
    tags: ["cast_stdlib", "storage"],
}
