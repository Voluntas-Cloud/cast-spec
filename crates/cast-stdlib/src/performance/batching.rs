//! `batching` — combine small operations.

/// Sentinel for `batching`.
pub struct Batching;

cast::concept! {
    name: "batching",
    summary: "Combine small operations. Amortizes per-call overhead \
              across many items; trades a bit of latency for a lot of \
              throughput.",
    anchors: [cast_stdlib::performance::batching::Batching],
    tags: ["cast_stdlib", "performance"],
}
