//! `distributed_filesystem` — filesystem spanning machines.

/// Sentinel for `distributed_filesystem`.
pub struct DistributedFilesystem;

cast::concept! {
    name: "distributed_filesystem",
    summary: "filesystem spanning machines.",
    anchors: [cast_os_stdlib::filesystem_storage::distributed_filesystem::DistributedFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
