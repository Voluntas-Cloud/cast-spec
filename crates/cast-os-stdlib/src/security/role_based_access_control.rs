//! `role_based_access_control` — authority by assigned role.

/// Sentinel for `role_based_access_control`.
pub struct RoleBasedAccessControl;

cast::concept! {
    name: "role_based_access_control",
    summary: "authority by assigned role.",
    anchors: [cast_os_stdlib::security::role_based_access_control::RoleBasedAccessControl],
    tags: ["cast_os_stdlib", "security"],
}
