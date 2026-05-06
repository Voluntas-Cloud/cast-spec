//! `snapshot_filesystem` — filesystem supports point-in-time snapshots.

/// Sentinel for `snapshot_filesystem`.
pub struct SnapshotFilesystem;

cast::concept! {
    name: "snapshot_filesystem",
    summary: "filesystem supports point-in-time snapshots.",
    anchors: [cast_os_stdlib::filesystem_storage::snapshot_filesystem::SnapshotFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
