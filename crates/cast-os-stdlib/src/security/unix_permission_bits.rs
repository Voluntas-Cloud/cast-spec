//! `unix_permission_bits` — owner/group/other permission model.

/// Sentinel for `unix_permission_bits`.
pub struct UnixPermissionBits;

cast::concept! {
    name: "unix_permission_bits",
    summary: "owner/group/other permission model.",
    anchors: [cast_os_stdlib::security::unix_permission_bits::UnixPermissionBits],
    tags: ["cast_os_stdlib", "security"],
}
