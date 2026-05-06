//! `sharding` — split data/work by key.

/// Sentinel for `sharding`.
pub struct Sharding;

cast::concept! {
    name: "sharding",
    summary: "Split data/work by key. Each shard owns a slice of the \
              keyspace; capacity scales with shard count, paid for in \
              cross-shard query complexity.",
    anchors: [cast_stdlib::performance::sharding::Sharding],
    tags: ["cast_stdlib", "performance"],
}
