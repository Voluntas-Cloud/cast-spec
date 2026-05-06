//! `write_through_cache` — writes update cache and backing store.

/// Sentinel for `write_through_cache`.
pub struct WriteThroughCache;

cast::concept! {
    name: "write_through_cache",
    summary: "Writes update cache and backing store. Both are kept in \
              sync; reader after write sees the new value. Slower \
              writes, simpler invariants.",
    anchors: [cast_stdlib::performance::write_through_cache::WriteThroughCache],
    tags: ["cast_stdlib", "performance"],
}
