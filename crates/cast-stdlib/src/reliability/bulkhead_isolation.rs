//! `bulkhead_isolation` — isolate failures between subsystems.

/// Sentinel for `bulkhead_isolation`.
pub struct BulkheadIsolation;

cast::concept! {
    name: "bulkhead_isolation",
    summary: "Isolate failures between subsystems. Resource pools, \
              threads, or queues are partitioned so a failure in one \
              compartment cannot drown the whole ship.",
    anchors: [cast_stdlib::reliability::bulkhead_isolation::BulkheadIsolation],
    tags: ["cast_stdlib", "reliability"],
}
