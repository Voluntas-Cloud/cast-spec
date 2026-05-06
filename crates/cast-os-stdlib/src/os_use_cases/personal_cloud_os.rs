//! `personal_cloud_os` — user-owned service platform spanning devices/nodes.

/// Sentinel for `personal_cloud_os`.
pub struct PersonalCloudOs;

cast::concept! {
    name: "personal_cloud_os",
    summary: "user-owned service platform spanning devices/nodes.",
    anchors: [cast_os_stdlib::os_use_cases::personal_cloud_os::PersonalCloudOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
