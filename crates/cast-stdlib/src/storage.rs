//! Storage & persistence patterns.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only the
//! group sentinel, cross-concept comparisons, and the category rule.

pub mod append_only_log;
pub mod bounded_log;
pub mod compaction;
pub mod content_addressed_cache;
pub mod copy_on_write_overlay;
pub mod deduplicated_storage;
pub mod durable_volume;
pub mod encrypted_at_rest_storage;
pub mod ephemeral_storage;
pub mod filesystem_namespace;
pub mod immutable_blob_store;
pub mod log_structured_storage;
pub mod mutable_index_over_immutable_data;
pub mod mvcc_generation_log;
pub mod object_storage;
pub mod ring_buffer;
pub mod schema_versioned_storage;
pub mod snapshot_storage;
pub mod tiered_storage_by_latency;
pub mod transactional_storage;
pub mod write_ahead_log;

cast::concept! {
    name: "storage",
    summary: "Umbrella for the storage stdlib category. Storage & \
              persistence patterns.",
    anchors: [
        crate::storage::append_only_log,
        crate::storage::bounded_log,
        crate::storage::compaction,
        crate::storage::content_addressed_cache,
        crate::storage::copy_on_write_overlay,
        crate::storage::deduplicated_storage,
        crate::storage::durable_volume,
        crate::storage::encrypted_at_rest_storage,
        crate::storage::ephemeral_storage,
        crate::storage::filesystem_namespace,
        crate::storage::immutable_blob_store,
        crate::storage::log_structured_storage,
        crate::storage::mutable_index_over_immutable_data,
        crate::storage::mvcc_generation_log,
        crate::storage::object_storage,
        crate::storage::ring_buffer,
        crate::storage::schema_versioned_storage,
        crate::storage::snapshot_storage,
        crate::storage::tiered_storage_by_latency,
        crate::storage::transactional_storage,
        crate::storage::write_ahead_log,
    ],
    tags: ["cast_stdlib", "storage"],
}

/// Sentinel for the storage stdlib group.
pub struct StorageGroup;

cast::compare! {
    cow_overlay @ cast_stdlib::storage::copy_on_write_overlay::CopyOnWriteOverlay:
        "in-memory, RAM tracks churn rate, dies with the process, cheap version diffs and time-travel within process lifetime",
    mvcc_log    @ cast_stdlib::storage::mvcc_generation_log::MvccGenerationLog:
        "durable, survives restarts, supports cross-process audit; each lookup pays an index hop",
    note: "Two families of 'keep history' shape. CoW wins on memory and speed under sparse writes within a single process; MVCC log wins on durability and cross-process auditability.",
    tags: ["cast_stdlib", "storage", "versioning"],
}

cast::rule! {
    rule: "Separate identity, location, version, and content.",
    why:  "Mixing those is how storage systems become cursed \
           archaeology — every layer ends up redoing version-detection, \
           identity comparison, and content equality at the same time, \
           and a change to any axis breaks the others silently.",
    governs: [cast_stdlib::storage::StorageGroup],
    tags: ["cast_stdlib", "storage"],
}
