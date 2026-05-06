//! `storage_class_selection` — choose storage by performance/durability.

/// Sentinel for `storage_class_selection`.
pub struct StorageClassSelection;

cast::concept! {
    name: "storage_class_selection",
    summary: "Choose storage by performance and durability. Workloads \
              declare what they need (latency, replication, encryption, \
              cost class); provisioning maps that to a concrete backend \
              instead of hard-coding it.",
    anchors: [cast_stdlib::resources::storage_class_selection::StorageClassSelection],
    tags: ["cast_stdlib", "resources"],
}
