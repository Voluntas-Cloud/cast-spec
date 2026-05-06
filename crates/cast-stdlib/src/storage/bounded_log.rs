//! `bounded_log` — append-only log with a capacity cap and oldest-first eviction.

/// Sentinel for `bounded_log`.
pub struct BoundedLog;

cast::concept! {
    name: "bounded_log",
    summary: "An append-only log with a capacity cap. Once the log \
              exceeds its high-water mark, the oldest entries are \
              dropped down to a low-water mark before the next write \
              proceeds. Hysteresis between the two marks amortizes \
              eviction over many writes instead of paying a delete \
              cost on every append once at capacity. Use when you want \
              the most-recent N records observable without unbounded \
              growth, and don't need durable history beyond N.",
    anchors: [cast_stdlib::storage::bounded_log::BoundedLog],
    tags: ["cast_stdlib", "storage"],
}
