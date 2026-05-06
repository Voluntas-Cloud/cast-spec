//! `secure_workstation_os` — OS optimized for isolation and policy enforcement.

/// Sentinel for `secure_workstation_os`.
pub struct SecureWorkstationOs;

cast::concept! {
    name: "secure_workstation_os",
    summary: "OS optimized for isolation and policy enforcement.",
    anchors: [cast_os_stdlib::os_use_cases::secure_workstation_os::SecureWorkstationOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
