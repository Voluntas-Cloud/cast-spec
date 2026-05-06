//! `cgroup_resource_control` — group resource accounting and limits.

/// Sentinel for `cgroup_resource_control`.
pub struct CgroupResourceControl;

cast::concept! {
    name: "cgroup_resource_control",
    summary: "group resource accounting and limits.",
    anchors: [cast_os_stdlib::isolation::cgroup_resource_control::CgroupResourceControl],
    tags: ["cast_os_stdlib", "isolation"],
}
