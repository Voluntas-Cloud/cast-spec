//! `least_privilege_service_os` — system services run with constrained rights.

/// Sentinel for `least_privilege_service_os`.
pub struct LeastPrivilegeServiceOs;

cast::concept! {
    name: "least_privilege_service_os",
    summary: "system services run with constrained rights.",
    anchors: [cast_os_stdlib::architectural_patterns::least_privilege_service_os::LeastPrivilegeServiceOs],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
