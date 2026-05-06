//! `host_os` — OS hosting virtualization.

/// Sentinel for `host_os`.
pub struct HostOs;

cast::concept! {
    name: "host_os",
    summary: "OS hosting virtualization.",
    anchors: [cast_os_stdlib::virtualization::host_os::HostOs],
    tags: ["cast_os_stdlib", "virtualization"],
}
