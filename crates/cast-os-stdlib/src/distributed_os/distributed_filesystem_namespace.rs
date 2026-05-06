//! `distributed_filesystem_namespace` — shared namespace across nodes.

/// Sentinel for `distributed_filesystem_namespace`.
pub struct DistributedFilesystemNamespace;

cast::concept! {
    name: "distributed_filesystem_namespace",
    summary: "shared namespace across nodes.",
    anchors: [cast_os_stdlib::distributed_os::distributed_filesystem_namespace::DistributedFilesystemNamespace],
    tags: ["cast_os_stdlib", "distributed_os"],
}
