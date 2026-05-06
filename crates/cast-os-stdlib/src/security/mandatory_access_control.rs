//! `mandatory_access_control` — system policy enforces access.

/// Sentinel for `mandatory_access_control`.
pub struct MandatoryAccessControl;

cast::concept! {
    name: "mandatory_access_control",
    summary: "system policy enforces access.",
    anchors: [cast_os_stdlib::security::mandatory_access_control::MandatoryAccessControl],
    tags: ["cast_os_stdlib", "security"],
}
