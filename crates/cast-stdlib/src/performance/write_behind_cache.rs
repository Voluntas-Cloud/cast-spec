//! `write_behind_cache` — writes buffered before backing store.

/// Sentinel for `write_behind_cache`.
pub struct WriteBehindCache;

cast::concept! {
    name: "write_behind_cache",
    summary: "Writes buffered before backing store. Returns to caller \
              fast; the cache flushes asynchronously. Risk: data lost \
              if the cache crashes before the flush.",
    anchors: [cast_stdlib::performance::write_behind_cache::WriteBehindCache],
    tags: ["cast_stdlib", "performance"],
}
