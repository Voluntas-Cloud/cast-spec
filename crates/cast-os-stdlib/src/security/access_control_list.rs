//! `access_control_list` — per-object permission list.

/// Sentinel for `access_control_list`.
pub struct AccessControlList;

cast::concept! {
    name: "access_control_list",
    summary: "per-object permission list.",
    anchors: [cast_os_stdlib::security::access_control_list::AccessControlList],
    tags: ["cast_os_stdlib", "security"],
}
