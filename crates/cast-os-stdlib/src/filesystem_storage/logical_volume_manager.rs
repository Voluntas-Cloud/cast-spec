//! `logical_volume_manager` — manage logical block volumes.

/// Sentinel for `logical_volume_manager`.
pub struct LogicalVolumeManager;

cast::concept! {
    name: "logical_volume_manager",
    summary: "manage logical block volumes.",
    anchors: [cast_os_stdlib::filesystem_storage::logical_volume_manager::LogicalVolumeManager],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
