//! `tiered_storage_by_latency` — hot/warm/cold by access cost.

/// Sentinel for `tiered_storage_by_latency`.
pub struct TieredStorageByLatency;

cast::concept! {
    name: "tiered_storage_by_latency",
    summary: "Hot/warm/cold storage based on access speed and cost. \
              Each tier names a latency property the implementation \
              must deliver; readers do not need to know which tier \
              their data lives in.",
    anchors: [cast_stdlib::storage::tiered_storage_by_latency::TieredStorageByLatency],
    tags: ["cast_stdlib", "storage"],
}
