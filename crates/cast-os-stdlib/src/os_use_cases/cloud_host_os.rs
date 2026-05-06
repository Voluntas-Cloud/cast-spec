//! `cloud_host_os` — OS optimized for virtualization and containers.

/// Sentinel for `cloud_host_os`.
pub struct CloudHostOs;

cast::concept! {
    name: "cloud_host_os",
    summary: "OS optimized for virtualization and containers.",
    anchors: [cast_os_stdlib::os_use_cases::cloud_host_os::CloudHostOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
