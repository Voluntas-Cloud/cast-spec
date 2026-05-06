//! `coalescing_writes` — collapse pending writes to the same key into one effective write.

/// Sentinel for `coalescing_writes`.
pub struct CoalescingWrites;

cast::concept! {
    name: "coalescing_writes",
    summary: "When multiple writes target the same key while a prior \
              write is still pending, drop or merge the older ones so \
              the storage tier sees one effective write per key. \
              Where `batching` amortises per-call overhead across \
              unrelated items, coalescing exploits that intermediate \
              values are observable only by the final write — useful \
              when the source is bursty (UI state, sensor reading, \
              latest-value-wins counters).",
    anchors: [cast_stdlib::performance::coalescing_writes::CoalescingWrites],
    tags: ["cast_stdlib", "performance"],
}
