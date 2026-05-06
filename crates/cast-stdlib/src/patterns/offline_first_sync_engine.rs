//! `offline_first_sync_engine` — clients work locally and sync changes later.

/// Sentinel for `offline_first_sync_engine`.
pub struct OfflineFirstSyncEngine;

cast::concept! {
    name: "offline_first_sync_engine",
    summary: "Clients work locally and sync changes later. Composes \
              local_state, change_log, sync_engine, \
              conflict_resolution, mergeable_state, crdt_state, \
              eventual_consistency, and external_id_mapping. Used \
              for mobile apps, note/document systems, personal data \
              stores, edge devices, and local-first productivity \
              tools.",
    anchors: [cast_stdlib::patterns::offline_first_sync_engine::OfflineFirstSyncEngine],
    tags: ["cast_stdlib", "patterns"],
}
