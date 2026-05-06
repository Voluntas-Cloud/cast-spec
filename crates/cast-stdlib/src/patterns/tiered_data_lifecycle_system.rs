//! `tiered_data_lifecycle_system` — data moves between hot, warm, cold, archive, and deletion states.

/// Sentinel for `tiered_data_lifecycle_system`.
pub struct TieredDataLifecycleSystem;

cast::concept! {
    name: "tiered_data_lifecycle_system",
    summary: "Data moves between hot, warm, cold, archive, and \
              deletion states. Composes tiered_storage_by_latency, \
              retention_policy, storage_class_selection, compaction, \
              snapshot_storage, encrypted_at_rest_storage, \
              cost_budget_policy, and hard_delete. Used for logs, \
              backups, AI embeddings, media storage, audit records, \
              and large personal archives.",
    anchors: [cast_stdlib::patterns::tiered_data_lifecycle_system::TieredDataLifecycleSystem],
    tags: ["cast_stdlib", "patterns"],
}
