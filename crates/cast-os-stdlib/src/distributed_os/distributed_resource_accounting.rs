//! `distributed_resource_accounting` — track resources across nodes.

/// Sentinel for `distributed_resource_accounting`.
pub struct DistributedResourceAccounting;

cast::concept! {
    name: "distributed_resource_accounting",
    summary: "track resources across nodes.",
    anchors: [cast_os_stdlib::distributed_os::distributed_resource_accounting::DistributedResourceAccounting],
    tags: ["cast_os_stdlib", "distributed_os"],
}
