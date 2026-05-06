//! `snapshot_storage` — point-in-time coherent state capture.

/// Sentinel for `snapshot_storage`.
pub struct SnapshotStorage;

cast::concept! {
    name: "snapshot_storage",
    summary: "Point-in-time state capture. A coherent view of the \
              system at a single moment; readers see all writes that \
              committed before the snapshot and none that committed \
              after.",
    anchors: [cast_stdlib::storage::snapshot_storage::SnapshotStorage],
    tags: ["cast_stdlib", "storage"],
}
