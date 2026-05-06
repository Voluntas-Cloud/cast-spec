//! `tombstone_record` — durable deletion marker, distinguishes 'deleted' from 'never existed'.

/// Sentinel for `tombstone_record`.
pub struct TombstoneRecord;

cast::concept! {
    name: "tombstone_record",
    summary: "Durable deletion marker. Tells replicas/readers the \
              record was deleted, distinguishing 'deleted' from \
              'never existed' until compaction reclaims it.",
    anchors: [cast_stdlib::lifecycle::tombstone_record::TombstoneRecord],
    tags: ["cast_stdlib", "lifecycle"],
}
